use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddr, TcpListener},
};

#[tokio::main]
async fn main() {
    let out = tokio::spawn(async {
        println!("Hello, world 111!");
    });
    let out2 = tokio::spawn(async {
        println!("Hello, world 222!");
    });
    _ = out.await;
    _ = out2.await;
    println!("Hello, world end!");

    // let addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1),8089);
    // std::net::Ipv4Addr::new(a, b, c, d)
    // let ip
    let listener = TcpListener::bind("127.0.0.1:8099").unwrap();
    // loop {
    //     match listener.accept() {
    //         Ok((mut socket, addr)) => {
    //             println!("new client: {addr:?}");
    //             let size = socket
    //                 .write("HTTP/1.1 200 OK\r\n\r\nServer:OK".as_bytes())
    //                 .unwrap();
    //             println!("size:{size}");
    //         }
    //         Err(e) => println!("couldn't get client: {e:?}"),
    //     };
    // }
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let size = stream
                    .write("HTTP/1.1 200 OK\r\n\r\nServer:OK".as_bytes())
                    .unwrap();

                stream.flush().unwrap(); // 长连接场景
                println!("size:{size}");
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        };
    }
}
