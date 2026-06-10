use std::num::NonZeroU32;
use std::sync::Arc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    context: Option<Context<Arc<Window>>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attrs = WindowAttributes::default()
            .with_title("winit 示例")
            .with_inner_size(LogicalSize::new(800.0, 600.0));

        let window = event_loop
            .create_window(attrs)
            .expect("创建 winit 窗口失败");
        let window = Arc::new(window);
        let context = Context::new(window.clone()).expect("创建 softbuffer 上下文失败");
        let surface = Surface::new(&context, window.clone()).expect("创建 softbuffer 画布失败");

        eprintln!("winit 窗口已创建，按 Esc 或关闭窗口退出");
        self.context = Some(context);
        self.surface = Some(surface);
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                eprintln!("收到 CloseRequested，退出事件循环");
                event_loop.exit();
            }
            WindowEvent::Destroyed => {
                eprintln!("收到 Destroyed，窗口已被销毁");
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                eprintln!("收到 Esc，退出事件循环");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let Some(surface) = self.surface.as_mut() else {
                    return;
                };
                let size = window.inner_size();
                let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                else {
                    return;
                };

                surface.resize(width, height).expect("调整 softbuffer 画布失败");
                let mut buffer = surface.buffer_mut().expect("获取 softbuffer 缓冲区失败");
                buffer.fill(0xff181818);
                buffer.present().expect("提交 softbuffer 缓冲区失败");
            }
            WindowEvent::Resized(_) => {
                window.request_redraw();
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();

    event_loop.run_app(&mut app)?;

    Ok(())
}
