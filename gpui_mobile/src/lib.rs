use gpui::{
    div, rgb, App, AppContext, Application, Context, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Window, WindowOptions,
};

extern crate gpui_mobile;

struct Home {
    scan_status: String,
    scanning: bool,
}

impl Home {
    fn new() -> Self {
        Self {
            scan_status: "尚未扫描".to_string(),
            scanning: false,
        }
    }
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

impl Render for Home {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                                    this.scan_status = match result {
                                        Ok(Some(value)) => match scan_value_to_url(&value) {
                                            Some(url) => {
                                                match gpui_mobile::packages::webview::open_url(&url)
                                                {
                                                    Ok(()) => format!("已通过 WebView 打开：{url}"),
                                                    Err(err) => format!("WebView 打开失败：{err}"),
                                                }
                                            }
                                            None => format!("扫描结果不是网址：{value}"),
                                        },
                                        Ok(None) => "已取消扫描".to_string(),
                                        Err(err) => format!("扫描失败：{err}"),
                                    };
                                    cx.notify();
                                });
                            })
                            .detach();
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
