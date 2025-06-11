use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // 绑定到本地端口 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("服务器运行在 http://127.0.0.1:7878");

    // 处理每个连接
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // 创建缓冲读取器
    let buf_reader = BufReader::new(&mut stream);

    // 获取请求首行
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // 根据请求路径路由
    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "Hello, World!"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5)); // 模拟慢请求
            ("HTTP/1.1 200 OK", "Done sleeping!")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "Page not found"),
    };

    // 构建响应
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    // 发送响应
    stream.write_all(response.as_bytes()).unwrap();
}
