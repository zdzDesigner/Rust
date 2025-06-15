use libc::{c_int, fd_set, select, timeval, FD_ISSET, FD_SET, FD_ZERO};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    listener.set_nonblocking(true)?;
    let listener_fd = listener.as_raw_fd();

    let mut read_fds: fd_set = unsafe { std::mem::zeroed() };

    loop {
        unsafe { FD_ZERO(&mut read_fds) };
        unsafe { FD_SET(listener_fd, &mut read_fds) };

        let mut max_fd = listener_fd;
        // 可在此添加更多描述符并更新 max_fd

        let timeout = timeval {
            tv_sec: 1,
            tv_usec: 0,
        };

        match unsafe {
            select(
                max_fd + 1,
                &mut read_fds,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &timeout as *const _ as *mut _,
            )
        } {
            -1 => eprintln!("Select error"),
            0 => println!("Timeout"),
            n => {
                println!("{} descriptors ready", n);
                if unsafe { FD_ISSET(listener_fd, &read_fds) } {
                    if let Ok((stream, _)) = listener.accept() {
                        println!("New connection: {}", stream.peer_addr()?);
                    }
                }
                // 检查其他描述符...
            }
        }
    }
}
