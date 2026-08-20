//! 文件传输页面：文件列表、下载、上传与进度展示。
//!
//! [输入]: 扫码获得的 PC 服务器地址；用户点击事件
//! [输出]: TransferState 与渲染函数、下载/上传/刷新操作
//! [定位]: 文件互传功能的 UI 层，业务逻辑在此聚合
//! [同步]: src/transfer.rs、src/lib.rs（Home 结构）

use std::path::PathBuf;
use std::sync::mpsc::{channel, TryRecvError};
use std::time::Duration;

use gpui::{
    div, rgb, Context, Div, FontWeight, InteractiveElement, ParentElement, Styled,
};

use crate::transfer;
use crate::Home;

pub struct TransferState {
    pub base: String,
    pub files: Vec<transfer::FileEntry>,
    pub status: String,
    pub busy: bool,
}

impl TransferState {
    pub fn new(base: String) -> Self {
        Self {
            base,
            files: Vec::new(),
            status: String::new(),
            busy: false,
        }
    }
}

pub fn render_transfer_page(home: &mut Home, cx: &mut Context<Home>) -> Div {
    let state = &mut home.transfer;

    let mut list = div().flex().flex_col().gap_2();
    if state.files.is_empty() && !state.busy {
        list = list.child(
            div()
                .px_4()
                .py_3()
                .rounded_lg()
                .bg(rgb(0xf3f4f6))
                .text_color(rgb(0x6b7280))
                .text_sm()
                .child("共享目录为空"),
        );
    }
    for entry in state.files.clone() {
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
                        .bg(if state.busy { rgb(0x9ca3af) } else { rgb(0x2563eb) })
                        .text_color(rgb(0xffffff))
                        .text_sm()
                        .child(if state.busy { "传输中" } else { "下载" })
                        .on_mouse_down(gpui::MouseButton::Left, {
                            let name = name.clone();
                            cx.listener(move |this, _event, _window, cx| {
                                this.start_download(name.clone(), cx);
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
                                this.page = crate::Page::Home;
                                cx.notify();
                            })
                        }),
                )
                .child(div().flex_1().child(
                    div()
                        .text_color(rgb(0x111827))
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("文件传输"),
                ))
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
                                this.refresh_files(cx);
                            })
                        }),
                ),
        )
        .child(
            div()
                .mt_4()
                .text_color(rgb(0x6b7280))
                .text_xs()
                .child(state.base.clone()),
        )
        .child(div().mt_4().child(list))
        .child(
            div()
                .mt_4()
                .flex()
                .items_center()
                .justify_center()
                .px_8()
                .py_4()
                .rounded_lg()
                .bg(if state.busy { rgb(0x9ca3af) } else { rgb(0x2563eb) })
                .text_color(rgb(0xffffff))
                .text_lg()
                .child(if state.busy { "传输中..." } else { "上传文件" })
                .on_mouse_down(gpui::MouseButton::Left, {
                    cx.listener(|this, _event, _window, cx| {
                        this.start_upload(cx);
                    })
                }),
        )
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

impl Home {
    pub fn refresh_files(&mut self, cx: &mut Context<Self>) {
        if self.transfer.busy {
            return;
        }
        let base = self.transfer.base.clone();
        self.transfer.busy = true;
        self.transfer.status = "正在获取文件列表...".to_string();
        cx.notify();

        cx.spawn(async move |this, cx| {
            let parsed = transfer::TransferServer::parse(&base);
            let result = cx
                .background_executor()
                .spawn(async move { parsed.and_then(|server| transfer::list_files(&server)) })
                .await;

            let _ = this.update(cx, |this, cx| {
                this.transfer.busy = false;
                match result {
                    Ok(files) => {
                        this.transfer.files = files;
                        this.transfer.status = format!(
                            "共 {} 个文件，可下载或上传",
                            this.transfer.files.len()
                        );
                    }
                    Err(err) => {
                        this.transfer.status = format!("获取文件列表失败：{err}");
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn start_download(&mut self, name: String, cx: &mut Context<Self>) {
        if self.transfer.busy {
            return;
        }
        let base = self.transfer.base.clone();
        self.transfer.busy = true;
        self.transfer.status = format!("准备下载 {name}...");
        cx.notify();

        cx.spawn(async move |this, cx| {
            let parsed = transfer::TransferServer::parse(&base);
            let name_for_task = name.clone();
            let (tx, rx) = channel();

            let task = cx.background_executor().spawn(async move {
                let server = parsed?;
                let cache_dir = gpui_mobile::packages::path_provider::cache_directory()
                    .map_err(|e| format!("获取缓存目录失败：{e}"))?;
                let temp_name = format!(
                    "download_{}_{name_for_task}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_millis())
                        .unwrap_or(0)
                );
                let temp_path = cache_dir.join(temp_name);

                transfer::download(&server, &name_for_task, &temp_path, move |copied, total| {
                    let _ = tx.send((copied, total));
                })?;

                let location: String = transfer::save_to_downloads(
                    temp_path.to_str().unwrap_or_default(),
                    &name_for_task,
                )?;
                Ok::<_, String>(location)
            });

            loop {
                let mut latest = None;
                while let Ok((copied, total)) = rx.try_recv() {
                    latest = Some((copied, total));
                }
                if let Some((copied, total)) = latest {
                    let _ = this.update(cx, |this, cx| {
                        this.transfer.status = format!(
                            "下载 {name}：{}/{}",
                            fmt_size(copied),
                            fmt_size(total)
                        );
                        cx.notify();
                    });
                }
                if let Err(TryRecvError::Disconnected) = rx.try_recv() {
                    break;
                }
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;
            }

            let result = task.await;
            let _ = this.update(cx, |this, cx| {
                this.transfer.busy = false;
                this.transfer.status = match result {
                    Ok(location) => format!("下载完成，已保存到 {location}"),
                    Err(err) => format!("下载失败：{err}"),
                };
                cx.notify();
            });
        })
        .detach();
    }

    pub fn start_upload(&mut self, cx: &mut Context<Self>) {
        if self.transfer.busy {
            return;
        }
        let base = self.transfer.base.clone();
        self.transfer.busy = true;
        self.transfer.status = "正在选择文件...".to_string();
        cx.notify();

        cx.spawn(async move |this, cx| {
            let parsed = transfer::TransferServer::parse(&base);
            let (tx, rx) = channel();

            let task = cx.background_executor().spawn(async move {
                let server = parsed?;

                let options = gpui_mobile::packages::file_selector::OpenFileOptions::default();
                let selected = gpui_mobile::packages::file_selector::open_file(&options)
                    .map_err(|e| format!("选择文件失败：{e}"))?
                    .ok_or_else(|| "已取消选择".to_string())?;

                let cache_path = transfer::copy_content_to_cache(&selected.path)?;
                let path = PathBuf::from(cache_path);
                let name = selected.name.clone();

                transfer::upload(&server, &path, &name, move |sent, total| {
                    let _ = tx.send((sent, total));
                })
            });

            loop {
                let mut latest = None;
                while let Ok((sent, total)) = rx.try_recv() {
                    latest = Some((sent, total));
                }
                if let Some((sent, total)) = latest {
                    let _ = this.update(cx, |this, cx| {
                        this.transfer.status =
                            format!("上传中：{}/{}", fmt_size(sent), fmt_size(total));
                        cx.notify();
                    });
                }
                if let Err(TryRecvError::Disconnected) = rx.try_recv() {
                    break;
                }
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;
            }

            let result = task.await;
            let _ = this.update(cx, |this, cx| {
                this.transfer.busy = false;
                this.transfer.status = match result {
                    Ok(()) => "上传完成".to_string(),
                    Err(err) => format!("上传失败：{err}"),
                };
                cx.notify();
            });
        })
        .detach();
    }
}

pub fn fmt_size(bytes: u64) -> String {
    if bytes >= 1073741824 {
        format!("{:.2} GB", bytes as f64 / 1073741824.0)
    } else if bytes >= 1048576 {
        format!("{:.2} MB", bytes as f64 / 1048576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}
