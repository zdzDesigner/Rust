use gpui::{
    div, px, rgb, App, AppContext, Application, Context, Div, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Window, WindowOptions,
};
use gpui_mobile::{components::material::TextField, KeyboardType};
use pulldown_cmark::{html, Event, Options, Parser};
use std::{cell::RefCell, sync::atomic::Ordering};

extern crate gpui_mobile;

mod server;
mod server_page;
mod transfer;
mod transfer_page;

use server_page::ServerState;
use transfer_page::TransferState;

thread_local! {
    static LOGIN_PENDING_TEXT: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

struct Home {
    page: Page,
    scan_status: String,
    scanning: bool,
    username: TextField,
    password: TextField,
    focused_login_field: Option<LoginField>,
    transfer: TransferState,
    server: ServerState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Login,
    Transfer,
    Server,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoginField {
    Username,
    Password,
}

impl Home {
    fn new() -> Self {
        Self {
            page: Page::Home,
            scan_status: "尚未扫描".to_string(),
            scanning: false,
            username: TextField::new(""),
            password: TextField::new(""),
            focused_login_field: None,
            transfer: TransferState::new(String::new()),
            server: ServerState::new(),
        }
    }
}

fn install_login_keyboard_callback() {
    gpui_mobile::set_text_input_callback(Some(Box::new(|text: &str| {
        LOGIN_PENDING_TEXT.with(|pending| {
            pending.borrow_mut().push(text.to_string());
        });
    })));
    gpui_mobile::TEXT_INPUT_DIRTY.store(true, Ordering::Release);
}

fn scan_value_to_url(value: &str) -> Option<String> {
    let value = value.trim();
    if value.starts_with("http://") {
        return None;
    }

    if let Some(rest) = value.strip_prefix("https://") {
        if is_safe_domain(rest) {
            return Some(value.to_string());
        }
        return None;
    }

    if is_safe_domain(value) {
        return Some(format!("https://{value}"));
    }

    None
}

fn is_safe_domain(value: &str) -> bool {
    let host = value.split('/').next().unwrap_or_default();
    if host.is_empty()
        || host.eq_ignore_ascii_case("localhost")
        || host.ends_with(".local")
        || host.parse::<std::net::IpAddr>().is_ok()
    {
        return false;
    }

    host.contains('.')
        && host
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-'))
}

fn open_scan_value(value: &str) -> Result<String, String> {
    if let Some(url) = scan_value_to_url(value) {
        gpui_mobile::packages::webview::open_url(&url)?;
        return Ok(format!("已通过 WebView 打开：{url}"));
    }

    let html = if looks_like_markdown(value) {
        markdown_html(value)
    } else {
        text_html(value)
    };
    gpui_mobile::packages::webview::open_html(&html)?;
    Ok("已通过 WebView 打开文本内容".to_string())
}

fn looks_like_markdown(value: &str) -> bool {
    value.lines().any(|line| {
        let line = line.trim_start();
        line.starts_with('#')
            || line.starts_with("- ")
            || line.starts_with("* ")
            || line.starts_with("> ")
            || line.starts_with("```")
            || line.contains("**")
            || (line.contains('[') && line.contains("]("))
    })
}

fn markdown_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::empty()).map(|event| match event {
        Event::Html(value) => Event::Text(value),
        Event::InlineHtml(value) => Event::Text(value),
        other => other,
    });

    let mut body = String::new();
    html::push_html(&mut body, parser);
    html_document(&body, false)
}

fn text_html(text: &str) -> String {
    let escaped = escape_html(text);
    html_document(&format!("<pre>{escaped}</pre>"), true)
}

fn html_document(body: &str, preserve_text: bool) -> String {
    let pre_style = if preserve_text {
        "pre{white-space:pre-wrap;word-break:break-word;}"
    } else {
        ""
    };

    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
body{{font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif;margin:24px;line-height:1.65;color:#111827;background:#ffffff;}}
img{{max-width:100%;height:auto;}}
code{{background:#f3f4f6;border-radius:4px;padding:2px 4px;}}
pre{{background:#f3f4f6;border-radius:12px;padding:16px;overflow:auto;}}
blockquote{{border-left:4px solid #d1d5db;margin-left:0;padding-left:16px;color:#4b5563;}}
a{{color:#2563eb;}}
{pre_style}
</style>
</head>
<body>{body}</body>
</html>"#
    )
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

impl Render for Home {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.page {
            Page::Home => self.render_home(cx),
            Page::Login => self.render_login(cx),
            Page::Transfer => transfer_page::render_transfer_page(self, cx),
            Page::Server => server_page::render_server_page(self, cx),
        }
    }
}

impl Home {
    fn render_home(&mut self, cx: &mut Context<Self>) -> Div {
        let button_label = if self.scanning {
            "正在扫描..."
        } else {
            "扫描二维码"
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .bg(rgb(0xffffff))
            .text_color(rgb(0x111827))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(div().text_3xl().child("GPUI Mobile"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x6b7280))
                            .child("使用摄像头扫描二维码"),
                    ),
            )
            .child(
                div()
                    .px_8()
                    .py_4()
                    .rounded_lg()
                    .bg(rgb(if self.scanning { 0x9ca3af } else { 0x2563eb }))
                    .text_color(rgb(0xffffff))
                    .text_lg()
                    .child(button_label)
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            if this.scanning {
                                return;
                            }

                            this.scanning = true;
                            this.scan_status = "等待扫码结果...".to_string();
                            cx.notify();

                            cx.spawn(async |this, cx| {
                                let result = cx
                                    .background_executor()
                                    .spawn(async {
                                        let status = gpui_mobile::packages::permission_handler::request_permission(
                                            gpui_mobile::packages::permission_handler::Permission::Camera,
                                        )?;
                                        if !status.is_granted() {
                                            return Err(format!("需要摄像头权限，当前状态：{status:?}"));
                                        }

                                        gpui_mobile::packages::qr_scanner::scan_qr_code()
                                    })
                                    .await;

                                let _ = this.update(cx, |this, cx| {
                                    this.scanning = false;
                                    match result {
                                        Ok(Some(value)) => {
                                            match transfer::TransferServer::parse(&value) {
                                                Ok(server) => {
                                                    this.scan_status =
                                                        format!("正在连接 PC（{}）...", server.base());
                                                    let base = server.base();
                                                    let fallback_value = value.clone();
                                                    let probe_server = server.clone();
                                                    cx.spawn(async move |this, cx| {
                                                        let probed = cx
                                                            .background_executor()
                                                            .spawn(async move {
                                                                transfer::probe(&probe_server)
                                                            })
                                                            .await;

                                                        let _ = this.update(cx, |this, cx| {
                                                            match probed {
                                                                Ok(()) => {
                                                                    this.transfer =
                                                                        TransferState::new(base);
                                                                    this.page = Page::Transfer;
                                                                    this.scan_status =
                                                                        "已连接到 PC".to_string();
                                                                    this.refresh_files(cx);
                                                                }
                                                                Err(_) => {
                                                                    this.scan_status = match open_scan_value(
                                                                        &fallback_value,
                                                                    ) {
                                                                        Ok(status) => status,
                                                                        Err(err) => format!(
                                                                            "WebView 打开失败：{err}"
                                                                        ),
                                                                    };
                                                                }
                                                            }
                                                            cx.notify();
                                                        });
                                                    })
                                                    .detach();
                                                }
                                                Err(_) => {
                                                    this.scan_status =
                                                        match open_scan_value(&value) {
                                                            Ok(status) => status,
                                                            Err(err) => {
                                                                format!("WebView 打开失败：{err}")
                                                            }
                                                        };
                                                }
                                            }
                                        }
                                        Ok(None) => {
                                            this.scan_status = "已取消扫描".to_string();
                                        }
                                        Err(err) => {
                                            this.scan_status = format!("扫描失败：{err}");
                                        }
                                    }
                                    cx.notify();
                                });
                            })
                            .detach();
                        }),
                    ),
            )
            .child(
                div()
                    .px_8()
                    .py_4()
                    .rounded_lg()
                    .bg(rgb(0x059669))
                    .text_color(rgb(0xffffff))
                    .text_lg()
                    .child("作为服务器")
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.page = Page::Server;
                            this.start_server(cx);
                            cx.notify();
                        }),
                    ),
            )
            .child(
                div()
                    .px_8()
                    .py_4()
                    .rounded_lg()
                    .bg(rgb(0x111827))
                    .text_color(rgb(0xffffff))
                    .text_lg()
                    .child("跳转登录界面")
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.page = Page::Login;
                            cx.notify();
                        }),
                    ),
            )
            .child(
                div()
                    .max_w_96()
                    .px_5()
                    .py_4()
                    .rounded_lg()
                    .bg(rgb(0xf3f4f6))
                    .text_color(rgb(0x374151))
                    .text_sm()
                    .child(self.scan_status.clone()),
            )
    }

    fn render_login(&mut self, cx: &mut Context<Self>) -> Div {
        self.drain_login_pending_text();

        let username_value = input_display(
            &self.username.text,
            "用户名",
            self.focused_login_field == Some(LoginField::Username),
            false,
        );
        let password_value = input_display(
            &self.password.text,
            "密码",
            self.focused_login_field == Some(LoginField::Password),
            true,
        );

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x10243d))
            .px_4()
            .py_6()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(22.0))
                            .h(px(22.0))
                            .rounded_lg()
                            .bg(rgb(0x35e58b))
                            .text_color(rgb(0x06251a))
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("B"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_color(rgb(0xffffff))
                                    .text_base()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("Battery"),
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x8aa0b8))
                                    .text_xs()
                                    .child("查看你的设备状态"),
                            ),
                    ),
            )
            .child(
                div()
                    .mt_8()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .rounded_xl()
                    .bg(rgb(0xf8fafc))
                    .child(
                        div()
                            .text_color(rgb(0x071426))
                            .text_base()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("用户登录"),
                    )
                    .child(
                        login_field(
                            username_value,
                            self.focused_login_field == Some(LoginField::Username),
                        )
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.focused_login_field = Some(LoginField::Username);
                                install_login_keyboard_callback();
                                gpui_mobile::show_keyboard_with_type(KeyboardType::Default);
                                cx.notify();
                            }),
                        ),
                    )
                    .child(
                        login_field(
                            password_value,
                            self.focused_login_field == Some(LoginField::Password),
                        )
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.focused_login_field = Some(LoginField::Password);
                                install_login_keyboard_callback();
                                gpui_mobile::show_keyboard_with_type(KeyboardType::Default);
                                cx.notify();
                            }),
                        ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_8()
                            .py_3()
                            .rounded_lg()
                            .bg(rgb(0x071426))
                            .text_color(rgb(0xffffff))
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("登录"),
                    ),
            )
            .child(
                div()
                    .mt_4()
                    .px_4()
                    .py_3()
                    .rounded_lg()
                    .text_color(rgb(0x8aa0b8))
                    .text_sm()
                    .child("返回首页")
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.page = Page::Home;
                            this.focused_login_field = None;
                            gpui_mobile::hide_keyboard();
                            gpui_mobile::set_text_input_callback(None);
                            cx.notify();
                        }),
                    ),
            )
    }

    fn drain_login_pending_text(&mut self) {
        let texts =
            LOGIN_PENDING_TEXT.with(|pending| pending.borrow_mut().drain(..).collect::<Vec<_>>());
        if texts.is_empty() {
            return;
        }

        let Some(focused) = self.focused_login_field else {
            return;
        };

        let field = match focused {
            LoginField::Username => &mut self.username,
            LoginField::Password => &mut self.password,
        };

        for text in texts {
            match text.as_str() {
                "\x08" => field.delete_at_cursor(),
                "\x1b[D" => field.move_cursor_left(),
                "\x1b[C" => field.move_cursor_right(),
                "\x1b[H" => field.move_cursor_to_start(),
                "\x1b[F" => field.move_cursor_to_end(),
                "\n" | "\r" => {}
                other => field.insert_at_cursor(other),
            }
        }
    }
}

fn input_display(text: &str, placeholder: &str, focused: bool, password: bool) -> String {
    let mut value = if text.is_empty() {
        placeholder.to_string()
    } else if password {
        "*".repeat(text.chars().count())
    } else {
        text.to_string()
    };

    if focused {
        value.push('|');
    }

    value
}

fn login_field(value: String, focused: bool) -> Div {
    div()
        .flex()
        .items_center()
        .h(px(22.0))
        .px_3()
        .rounded_lg()
        .bg(rgb(if focused { 0xe2e8f0 } else { 0xeef2f7 }))
        .text_color(rgb(if focused { 0x111827 } else { 0x7b8794 }))
        .text_xs()
        .child(value)
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("mobile"),
    );

    gpui_mobile::android::jni::install_panic_hook();
    log::info!("android_main: start");

    let _platform = gpui_mobile::android::jni::init_platform(&app);
    let Some(shared) = gpui_mobile::android::jni::shared_platform() else {
        log::error!("android_main: shared_platform unavailable");
        return;
    };

    Application::with_platform(shared.into_rc()).run(|cx: &mut App| {
        match cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Home::new())) {
            Ok(_) => log::info!("android_main: GPUI window opened"),
            Err(err) => log::error!("android_main: failed to open GPUI window: {err:#}"),
        }

        cx.activate(true);
    });

    log::info!("android_main: exit");
}
