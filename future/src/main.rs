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

}
