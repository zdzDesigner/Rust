use std::error::Error;
use std::fmt::Write as _;
use std::sync::Arc;

use eframe::egui;

#[derive(Default)]
struct App {
    next_window_id: usize,
    windows: Vec<usize>,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading(std::format!("egui 多窗口示例,{}", self.next_window_id));
        ui.heading("egui 多窗口示例");
        ui.label("点击按钮生成一个新的原生窗口。");
        ui.label("点击按钮生成一个新的原生窗口。");

        if ui.button("生成窗口").clicked() {
            self.next_window_id += 1;
            self.windows.push(self.next_window_id);
        }
        if ui.button("关闭窗口").clicked() {
            ui.label("点击按钮生成一个新的原生窗口。");
        }

        ui.separator();
        ui.label(format!("当前生成窗口数量：{}", self.windows.len()));

        let mut closed_windows = Vec::new();
        for window_id in self.windows.iter().copied() {
            let viewport_id = egui::ViewportId::from_hash_of(("generated-window", window_id));
            let title = format!("生成窗口 {window_id}");
            let builder = egui::ViewportBuilder::default()
                .with_title(title.clone())
                .with_inner_size([360.0, 220.0]);

            let close_requested = ui.ctx().show_viewport_immediate(
                viewport_id,
                builder,
                move |ui, _viewport_class| {
                    ui.heading(&title);
                    ui.label("这是由主窗口按钮创建的窗口。可通过主窗口关闭");
                    ui.input(|input| input.viewport().close_requested())
                },
            );

            if close_requested {
                closed_windows.push(window_id);
            }
        }

        self.windows
            .retain(|window_id| !closed_windows.contains(window_id));
    }
}

fn main() {
    if let Err(error) = run_app() {
        let message = format_error_report(&error);
        if let Err(write_error) = std::fs::write(error_log_path(), &message) {
            eprintln!("写入错误日志失败：{write_error}; 原始错误：{message}");
        }
        show_error_dialog(&message);
        eprintln!("{message}");
    }
}

fn run_app() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("egui 主窗口")
            .with_inner_size([800.0, 600.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "egui 主窗口",
        options,
        Box::new(|creation_context| {
            setup_chinese_font(&creation_context.egui_ctx);
            Ok(Box::new(App::default()))
        }),
    )
}

fn error_log_path() -> std::path::PathBuf {
    match std::env::current_exe() {
        Ok(mut path) => {
            path.set_file_name("my_winit_error.log");
            path
        }
        Err(_) => std::path::PathBuf::from("my_winit_error.log"),
    }
}

fn format_error_report(error: &dyn Error) -> String {
    let mut report = format!("eframe 启动失败：{error}\n");
    let mut source = error.source();

    while let Some(error_source) = source {
        let _ = writeln!(report, "原因：{error_source}");
        source = error_source.source();
    }

    report
}

#[cfg(windows)]
fn show_error_dialog(message: &str) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};

    let title = "图形兼容性检查失败";
    let body = format!(
        "程序无法启动图形界面。\n\n{message}\n错误详情已写入 exe 同目录的 my_winit_error.log。"
    );
    let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    let body_wide: Vec<u16> = body.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            body_wide.as_ptr(),
            title_wide.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

#[cfg(not(windows))]
fn show_error_dialog(_message: &str) {}

fn setup_chinese_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let font_name = "droid-sans-fallback".to_owned();

    fonts.font_data.insert(
        font_name.clone(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/DroidSansFallbackFull.ttf"
        ))),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push(font_name.clone());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push(font_name);

    ctx.set_fonts(fonts);
}
