// #[tokio::main]
// async fn main() {
//     let out = tokio::spawn(async {
//         println!("Hello, world!");
//     });
//     _ = out.await;
//     println!("Hello, world!2");
// }


#[tokio::main]
async fn main() {
    tokio::spawn(async {
        println!("1");
    });
    println!("2");
}

