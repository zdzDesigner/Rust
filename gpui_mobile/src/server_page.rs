//! 文件服务器页面：二维码、共享目录列表、保存到公共下载。
//!
//! [输入]: RunningServer（src/server.rs）；用户点击事件
//! [输出]: ServerState 与渲染函数、启动/停止/刷新/导入/保存操作
//! [定位]: app 作为服务器时的 UI 层，业务逻辑在此聚合
//! [同步]: src/server.rs、src/lib.rs（Home 结构）

use std::time::Duration;

use gpui::{div, px, rgb, Context, Div, FontWeight, InteractiveElement, ParentElement, Styled};

use crate::transfer::{self, FileEntry};
use crate::transfer_page::fmt_size;
use crate::{server, Home, Page};

pub struct ServerState {
    pub running: Option<server::RunningServer>,
    pub qr: Option<qrcode::QrCode>,
    pub url: String,
    pub files: Vec<FileEntry>,
    pub status: String,
    pub busy: bool,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            running: None,
            qr: None,
            url: String::new(),
            files: Vec::new(),
            status: String::new(),
            busy: false,
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_server_page(home: &mut Home, cx: &mut Context<Home>) -> Div {
    let state = &mut home.server;

    let qr_block = match &state.qr {
        Some(qr) => render_qr(qr),
        None => div()
            .px_4()
            .py_6()
            .rounded_lg()
            .bg(rgb(0xf3f4f6))
            .text_color(rgb(0x6b7280))
            .text_sm()
            .child("正在生成二维码..."),
    };

    let mut list = div().flex().flex_col().gap_2();
    if state.files.is_empty() {
        list = list.child(
            div()
                .px_4()
                .py_3()
                .rounded_lg()
                .bg(rgb(0xf3f4f6))
                .text_color(rgb(0x6b7280))
                .text_sm()
                .child("共享目录为空，等待 PC 上传"),
        );
    }
    for entry in &state.files {
        let name = entry.name.clone();
        list = list.child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .px_4()
                .py_3()
                .rounded_lg()
                .bg(rgb(0xf3f4f6))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .child(
                            div()
                                .text_color(rgb(0x111827))
                                .text_sm()
                                .child(name.clone()),
                        )
                        .child(
                            div()
                                .text_color(rgb(0x9ca3af))
                                .text_xs()
                                .child(fmt_size(entry.size)),
                        ),
                )
                .child(
                    div()
                        .px_3()
                        .py_1()
                        .rounded_md()
                        .bg(if state.busy {
                            rgb(0x9ca3af)
                        } else {
                            rgb(0x2563eb)
                        })
                        .text_color(rgb(0xffffff))
                        .text_sm()
                        .child(if state.busy {
                            "处理中"
                        } else {
                            "转移到下载"
                        })
                        .on_mouse_down(gpui::MouseButton::Left, {
                            let name = name.clone();
                            cx.listener(move |this, _event, _window, cx| {
                                this.save_server_file(name.clone(), cx);
                            })
                        }),
                ),
        );
    }

    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(rgb(0xffffff))
        .text_color(rgb(0x111827))
        .px_4()
        .py_6()
        .pt(px(48.0))
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .px_3()
                        .py_2()
                        .rounded_lg()
                        .bg(rgb(0xf3f4f6))
                        .text_color(rgb(0x374151))
                        .text_sm()
                        .child("返回首页")
                        .on_mouse_down(gpui::MouseButton::Left, {
                            cx.listener(|this, _event, _window, cx| {
                                this.stop_server();
                                this.page = Page::Home;
                                cx.notify();
                            })
                        }),
                )
                .child(
                    div().flex_1().child(
                        div()
                            .text_color(rgb(0x111827))
                            .text_base()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("文件服务器"),
                    ),
                )
                .child(
                    div()
                        .px_3()
                        .py_2()
                        .rounded_lg()
                        .bg(if state.busy {
                            rgb(0x9ca3af)
                        } else {
                            rgb(0x059669)
                        })
                        .text_color(rgb(0xffffff))
                        .text_sm()
                        .child(if state.busy {
                            "处理中"
                        } else {
                            "添加手机文件"
                        })
                        .on_mouse_down(gpui::MouseButton::Left, {
                            cx.listener(|this, _event, _window, cx| {
                                this.import_server_file(cx);
                            })
                        }),
                )
                .child(
                    div()
                        .px_3()
                        .py_2()
                        .rounded_lg()
                        .bg(rgb(0x111827))
                        .text_color(rgb(0xffffff))
                        .text_sm()
                        .child("刷新")
                        .on_mouse_down(gpui::MouseButton::Left, {
                            cx.listener(|this, _event, _window, cx| {
                                this.refresh_server_files(cx);
                            })
                        }),
                ),
        )
        .child(
            div()
                .mt_4()
                .flex()
                .flex_col()
                .items_center()
                .gap_2()
                .child(qr_block)
                .child(
                    div()
                        .text_color(rgb(0x6b7280))
                        .text_xs()
                        .child("PC 浏览器访问下方地址（需同一 WiFi）"),
                )
                .child(
                    div()
                        .text_color(rgb(0x111827))
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(state.url.clone()),
                ),
        )
        .child(div().mt_4().child(list))
        .child(
            div()
                .mt_4()
                .px_4()
                .py_3()
                .rounded_lg()
                .bg(rgb(0xf3f4f6))
                .text_color(rgb(0x374151))
                .text_sm()
                .child(state.status.clone()),
        )
}

fn render_qr(qr: &qrcode::QrCode) -> Div {
    const QUIET: usize = 4;
    const MODULE: f32 = 3.0;

    let width = qr.width();
    let cells: Vec<qrcode::Color> = qr.to_colors();
    let dark = qrcode::Color::Dark;

    let mut grid = div().flex().flex_col();
    for y in 0..(width + QUIET * 2) {
        let mut row_div = div().flex().flex_row();
        for x in 0..(width + QUIET * 2) {
            let dark_cell = x >= QUIET
                && y >= QUIET
                && x < width + QUIET
                && y < width + QUIET
                && cells[(y - QUIET) * width + (x - QUIET)] == dark;
            row_div = row_div.child(div().w(px(MODULE)).h(px(MODULE)).bg(rgb(if dark_cell {
                0x111827
            } else {
                0xffffff
            })));
        }
        grid = grid.child(row_div);
    }
    grid.p_3().rounded_lg().bg(rgb(0xffffff))
}

impl Home {
    pub fn start_server(&mut self, cx: &mut Context<Self>) {
        if self.server.running.is_some() || self.server.busy {
            return;
        }
        self.server.busy = true;
        self.server.status = "正在启动服务器...".to_string();
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let dir = gpui_mobile::packages::path_provider::documents_directory()
                        .map_err(|e| format!("获取应用目录失败：{e}"))?
                        .join("shared");
                    let running = server::RunningServer::start(&dir)?;
                    let url = running.url();
                    Ok::<_, String>((running, url))
                })
                .await;

            let _ = this.update(cx, |this, cx| {
                this.server.busy = false;
                match result {
                    Ok((running, url)) => {
                        this.server.qr = qrcode::QrCode::new(url.as_bytes()).ok();
                        this.server.url = url;
                        this.server.status = "服务器已启动".to_string();
                        this.server.running = Some(running);
                        this.refresh_server_files(cx);
                        this.schedule_server_auto_refresh(cx);
                    }
                    Err(err) => {
                        this.server.status = format!("启动失败：{err}");
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn stop_server(&mut self) {
        self.server.running = None;
        self.server.qr = None;
        self.server.url.clear();
        self.server.files.clear();
        self.server.status.clear();
        self.server.busy = false;
    }

    pub fn refresh_server_files(&mut self, cx: &mut Context<Self>) {
        let Some(running) = &self.server.running else {
            return;
        };
        let dir = running.dir.clone();

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { server::list_local(&dir) })
                .await;

            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(files) => {
                        this.server.files = files;
                    }
                    Err(err) => {
                        this.server.status = format!("读取共享目录失败：{err}");
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn save_server_file(&mut self, name: String, cx: &mut Context<Self>) {
        if self.server.busy {
            return;
        }
        let Some(running) = &self.server.running else {
            return;
        };
        let path = running.dir.join(&name);
        let Some(path_str) = path.to_str().map(str::to_string) else {
            self.server.status = "文件路径无效".to_string();
            cx.notify();
            return;
        };

        self.server.busy = true;
        self.server.status = format!("正在保存 {name} 到下载目录...");
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { transfer::save_to_downloads(&path_str, &name) })
                .await;

            let _ = this.update(cx, |this, cx| {
                this.server.busy = false;
                this.server.status = match result {
                    Ok(location) => format!("已保存到 {location}"),
                    Err(err) => format!("保存失败：{err}"),
                };
                this.refresh_server_files(cx);
                cx.notify();
            });
        })
        .detach();
    }

    pub fn import_server_file(&mut self, cx: &mut Context<Self>) {
        if self.server.busy {
            return;
        }
        let Some(running) = &self.server.running else {
            return;
        };
        let dir = running.dir.clone();

        self.server.busy = true;
        self.server.status = "正在选择手机文件...".to_string();
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let options = gpui_mobile::packages::file_selector::OpenFileOptions::default();
                    let selected = gpui_mobile::packages::file_selector::open_file(&options)
                        .map_err(|e| format!("选择文件失败：{e}"))?
                        .ok_or_else(|| "已取消选择".to_string())?;
                    let cache_path = transfer::copy_content_to_cache(&selected.path)?;
                    let cache_path = std::path::PathBuf::from(cache_path);
                    server::import_file(&dir, &cache_path, &selected.name)
                })
                .await;

            let _ = this.update(cx, |this, cx| {
                this.server.busy = false;
                this.server.status = match result {
                    Ok(name) => format!("已加入共享目录：{name}"),
                    Err(err) => format!("添加文件失败：{err}"),
                };
                this.refresh_server_files(cx);
                cx.notify();
            });
        })
        .detach();
    }

    fn schedule_server_auto_refresh(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(5)).await;
            let _ = this.update(cx, |this, cx| {
                if this.page != Page::Server || this.server.running.is_none() {
                    return;
                }
                if !this.server.busy {
                    this.refresh_server_files(cx);
                }
                this.schedule_server_auto_refresh(cx);
            });
        })
        .detach();
    }
}
