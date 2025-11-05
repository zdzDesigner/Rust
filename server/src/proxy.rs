#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    // let url = "https://www.baidu.com";
    // let url = "https://www.openmymind.net/atom.xml";
    let url = "https://www.google.com";
    // let proxy_url = "http://0.0.0.0:20171";
    let proxy_url = "http://127.0.0.1:1081";

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .proxy(reqwest::Proxy::http(proxy_url)?)
        .build()?;

    let content = client.get(url).send().await?.bytes().await?;

    // let content = reqwest::get(url).await?.bytes().await?;
    println!("{content:?}");

    Ok(())
}

// let client = reqwest::Client::builder()
//     .proxy(reqwest::Proxy::http("http://localhost:20171")?)
//     .build()?;
//
// client.get("https://www.baidu.com");

// #[tokio::main]
// async fn main() {
//     println!("Starting async main");
//
//     // 调用异步函数
//     let result = async_task().await;
//     println!("Result: {}", result);
// }
//
// // 示例异步函数
// async fn async_task() -> String {
//     tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
//     "Async task completed!".to_string()
// }
