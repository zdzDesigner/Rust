use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn Error>> {
    // 代理服务器地址
    let proxy_addr = "127.0.0.1:1080";
    // 目标地址
    let target_addr = "www.google.com:443";

    // 连接到代理服务器
    let mut stream = TcpStream::connect(proxy_addr)?;

    // SOCKS5 握手和连接请求
    establish_socks5_connection(&mut stream, target_addr)?;

    // 通过代理发送 HTTP 请求
    let request = "GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes())?;

    // 读取响应
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    println!("Response: {}", String::from_utf8_lossy(&response));

    Ok(())
}

fn establish_socks5_connection(
    stream: &mut TcpStream,
    target_addr: &str,
) -> Result<(), Box<dyn Error>> {
    // SOCKS5 握手
    let handshake = [0x05, 0x01, 0x00];
    stream.write_all(&handshake)?;

    let mut response = [0; 2];
    stream.read_exact(&mut response)?;

    if response != [0x05, 0x00] {
        return Err("SOCKS5 handshake failed".into());
    }

    // 解析目标地址
    let (addr_type, addr_bytes, port) = parse_target_addr(target_addr)?;

    // 构建连接请求
    let mut request = Vec::new();
    request.push(0x05); // SOCKS version
    request.push(0x01); // CONNECT command
    request.push(0x00); // reserved
    request.push(addr_type); // address type
    request.extend_from_slice(&addr_bytes); // address
    request.push((port >> 8) as u8); // port high byte
    request.push((port & 0xFF) as u8); // port low byte

    stream.write_all(&request)?;

    // 读取连接响应
    let mut response = [0; 10];
    stream.read_exact(&mut response[..4])?;

    if response[1] != 0x00 {
        return Err("SOCKS5 connection failed".into());
    }

    // 根据地址类型读取剩余响应
    match response[3] {
        0x01 => {
            // IPv4
            stream.read_exact(&mut response[4..10])?;
        }
        0x03 => {
            // Domain name
            let len = response[4] as usize;
            stream.read_exact(&mut response[5..5 + len + 2])?;
        }
        0x04 => {
            // IPv6
            stream.read_exact(&mut response[4..22])?;
        }
        _ => return Err("Invalid address type in response".into()),
    }

    Ok(())
}

fn parse_target_addr(addr: &str) -> Result<(u8, Vec<u8>, u16), Box<dyn Error>> {
    let parts: Vec<&str> = addr.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid target address format".into());
    }

    let host = parts[0];
    let port: u16 = parts[1].parse()?;

    // 尝试解析为 IPv4
    if let Ok(ip) = host.parse::<std::net::Ipv4Addr>() {
        return Ok((0x01, ip.octets().to_vec(), port));
    }

    // 尝试解析为 IPv6
    if let Ok(ip) = host.parse::<std::net::Ipv6Addr>() {
        return Ok((0x04, ip.octets().to_vec(), port));
    }

    // 作为域名处理
    if host.len() > 255 {
        return Err("Domain name too long".into());
    }

    let mut addr_bytes = Vec::new();
    addr_bytes.push(host.len() as u8);
    addr_bytes.extend_from_slice(host.as_bytes());

    Ok((0x03, addr_bytes, port))
}
