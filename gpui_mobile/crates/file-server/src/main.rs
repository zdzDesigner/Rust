//! 局域网文件互传 PC 端服务。
//!
//! [输入]: 命令行参数（共享目录）；本机局域网
//! [输出]: HTTP 服务（网页 UI + 文件列表/下载/上传 API）；二维码；自动打开浏览器
//! [定位]: 与 Android GPUI 应用配对的文件交换服务端
//! [同步]: crates/file-server/src/index.html

use std::fs;
use std::io::{Read, Write};
use std::net::UdpSocket;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::UNIX_EPOCH;

use qrcode::render::svg;
use qrcode::QrCode;
use serde_json::json;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

const INDEX_HTML: &str = include_str!("index.html");

struct SharedState {
    dir: PathBuf,
    port: u16,
}

fn main() {
    let dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./shared"));

    if let Err(err) = fs::create_dir_all(&dir) {
        eprintln!("无法创建共享目录 {}: {err}", dir.display());
        std::process::exit(1);
    }

    let server = match Server::http("0.0.0.0:0") {
        Ok(server) => server,
        Err(err) => {
            eprintln!("启动 HTTP 服务失败: {err}");
            std::process::exit(1);
        }
    };

    let port = server
        .server_addr()
        .to_ip()
        .map(|addr| addr.port())
        .unwrap_or(0);
    let ip = match lan_ip() {
        Some(ip) => ip,
        None => {
            eprintln!("无法确定本机局域网 IP");
            std::process::exit(1);
        }
    };

    let url = format!("http://{ip}:{port}/");
    println!("共享目录: {}", dir.display());
    println!("手机访问: {url}");

    if let Err(err) = open_browser(&format!("http://127.0.0.1:{port}/")) {
        eprintln!("无法自动打开浏览器（请手动访问 {url}）: {err}");
    }

    let server = Arc::new(server);
    let state = Arc::new(SharedState { dir, port });

    let mut workers = Vec::new();
    for _ in 0..3 {
        let server = Arc::clone(&server);
        let state = Arc::clone(&state);
        workers.push(std::thread::spawn(move || {
            for request in server.incoming_requests() {
                handle(request, &state);
            }
        }));
    }

    for request in server.incoming_requests() {
        handle(request, &state);
    }

    for worker in workers {
        let _ = worker.join();
    }
}

fn lan_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}

#[cfg(target_os = "linux")]
fn open_browser(url: &str) -> std::io::Result<()> {
    std::process::Command::new("xdg-open").arg(url).spawn().map(|_| ())
}

#[cfg(target_os = "macos")]
fn open_browser(url: &str) -> std::io::Result<()> {
    std::process::Command::new("open").arg(url).spawn().map(|_| ())
}

#[cfg(target_os = "windows")]
fn open_browser(url: &str) -> std::io::Result<()> {
    std::process::Command::new("cmd").args(["/c", "start", "", url]).spawn().map(|_| ())
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn open_browser(url: &str) -> std::io::Result<()> {
    let _ = url;
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "当前平台不支持自动打开浏览器",
    ))
}

type BoxedResponse = Response<Box<dyn Read + Send>>;

fn handle(request: Request, state: &SharedState) {
    let url = request.url().to_string();
    let path = url.split('?').next().unwrap_or(&url).to_string();

    let response: BoxedResponse = match (request.method(), path.as_str()) {
        (&Method::Get, "/") => serve_index(state),
        (&Method::Get, "/api/list") => api_list(state),
        (&Method::Get, p) if p.starts_with("/api/file/") => {
            api_download(&p["/api/file/".len()..], state)
        }
        (&Method::Post, p) if p == "/api/upload" => {
            api_upload(request, state);
            return;
        }
        _ => html_response(StatusCode(404), "404 Not Found".to_string()),
    };

    let _ = request.respond(response);
}

fn serve_index(state: &SharedState) -> BoxedResponse {
    let qr = lan_ip()
        .map(|ip| qr_svg(&format!("http://{ip}:{}/", state.port)))
        .unwrap_or_default();
    let body = INDEX_HTML.replace("__QR_SVG__", &qr);
    string_response(StatusCode(200), body, "text/html; charset=utf-8")
}

fn qr_svg(url: &str) -> String {
    match QrCode::new(url.as_bytes()) {
        Ok(code) => code
            .render::<svg::Color>()
            .dark_color(svg::Color("#111827"))
            .light_color(svg::Color("#ffffff"))
            .quiet_zone(true)
            .build(),
        Err(err) => {
            eprintln!("二维码生成失败: {err}");
            String::new()
        }
    }
}

fn string_response(status: StatusCode, body: String, content_type: &str) -> BoxedResponse {
    let header = Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
        .expect("valid content type header");
    let len = body.len();
    Response::new(
        status,
        vec![header],
        Box::new(std::io::Cursor::new(body.into_bytes())),
        Some(len),
        None,
    )
}

fn html_response(status: StatusCode, body: String) -> BoxedResponse {
    string_response(status, body, "text/html; charset=utf-8")
}

fn json_response(status: StatusCode, body: String) -> BoxedResponse {
    string_response(status, body, "application/json; charset=utf-8")
}

fn api_list(state: &SharedState) -> BoxedResponse {
    match list_entries(&state.dir) {
        Ok(entries) => {
            let body = serde_json::to_string(&entries).unwrap_or_else(|_| "[]".into());
            json_response(StatusCode(200), body)
        }
        Err(err) => json_response(
            StatusCode(500),
            serde_json::to_string(&json!({ "error": err })).unwrap_or_default(),
        ),
    }
}

#[derive(serde::Serialize)]
struct Entry {
    name: String,
    size: u64,
    mtime: u64,
}

fn list_entries(dir: &Path) -> Result<Vec<Entry>, String> {
    let mut entries = Vec::new();
    let read_dir = fs::read_dir(dir).map_err(|e| format!("读取共享目录失败: {e}"))?;
    for item in read_dir {
        let item = item.map_err(|e| format!("读取目录项失败: {e}"))?;
        let file_type = item.file_type().map_err(|e| format!("获取文件类型失败: {e}"))?;
        if !file_type.is_file() {
            continue;
        }
        let name = item.file_name().to_string_lossy().into_owned();
        let metadata = item.metadata().map_err(|e| format!("获取文件信息失败: {e}"))?;
        let mtime = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        entries.push(Entry {
            name,
            size: metadata.len(),
            mtime,
        });
    }
    entries.sort_by(|a, b| b.mtime.cmp(&a.mtime).then_with(|| a.name.cmp(&b.name)));
    Ok(entries)
}

fn api_download(name: &str, state: &SharedState) -> BoxedResponse {
    let Ok(name) = url_decode(name) else {
        return json_response(
            StatusCode(400),
            serde_json::to_string(&json!({ "error": "非法文件名编码" })).unwrap_or_default(),
        );
    };
    if !is_safe_name(&name) {
        return json_response(
            StatusCode(400),
            serde_json::to_string(&json!({ "error": "非法文件名" })).unwrap_or_default(),
        );
    }

    let path = state.dir.join(&name);
    match fs::File::open(&path) {
        Ok(file) => {
            let len = file.metadata().map(|m| m.len()).unwrap_or(0);
            let content_type = Header::from_bytes(
                &b"Content-Type"[..],
                b"application/octet-stream",
            )
            .expect("valid header");
            let disposition = Header::from_bytes(
                &b"Content-Disposition"[..],
                content_disposition(&name).as_bytes(),
            )
            .expect("valid header");
            Response::new(
                StatusCode(200),
                vec![content_type, disposition],
                Box::new(file),
                Some(len as usize),
                None,
            )
        }
        Err(err) => {
            let msg = if err.kind() == std::io::ErrorKind::NotFound {
                "文件不存在"
            } else {
                "打开文件失败"
            };
            json_response(
                StatusCode(404),
                serde_json::to_string(&json!({ "error": format!("{msg}: {err}") }))
                    .unwrap_or_default(),
            )
        }
    }
}

fn content_disposition(name: &str) -> String {
    let ascii_fallback: String = name
        .chars()
        .map(|c| if c.is_ascii() { c } else { '_' })
        .collect();
    format!(
        "attachment; filename=\"{ascii_fallback}\"; filename*=UTF-8''{}",
        encode_uri_component(name)
    )
}

fn encode_uri_component(s: &str) -> String {
    let mut out = String::new();
    for byte in s.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

fn is_safe_name(name: &str) -> bool {
    !name.is_empty()
        && name != "."
        && name != ".."
        && !name.contains('/')
        && !name.contains('\\')
        && !name.contains('\0')
}

fn url_decode(s: &str) -> Result<String, ()> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' => {
                if i + 2 >= bytes.len() {
                    return Err(());
                }
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).map_err(|_| ())?;
                let value = u8::from_str_radix(hex, 16).map_err(|_| ())?;
                out.push(value);
                i += 3;
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            other => {
                out.push(other);
                i += 1;
            }
        }
    }
    String::from_utf8(out).map_err(|_| ())
}

/// 上传协议：`POST /api/upload?name=<url编码>&size=<字节数>`，body 为纯文件字节。
/// `size` 存在时必须与 Content-Length 一致，防止截断文件落盘。
/// 读取 body 后自行 respond（body 必须读完，否则响应连接会被关闭）。
fn api_upload(mut request: Request, state: &SharedState) {
    let response = upload_response(&mut request, state);
    let _ = request.respond(response);
}

fn upload_response(request: &mut Request, state: &SharedState) -> BoxedResponse {
    let url = request.url().to_string();
    let query = url.split_once('?').map(|(_, q)| q).unwrap_or("");

    let name = parse_query_param(query, "name");
    let size = parse_query_param(query, "size").and_then(|s| s.parse::<u64>().ok());
    let body_len = request.body_length().unwrap_or(0) as u64;

    let filename = match name {
        Some(raw) => match url_decode(&raw) {
            Ok(decoded) => sanitize_filename(&decoded),
            Err(()) => sanitize_filename(&raw),
        },
        None => format!("upload_{}", unix_millis()),
    };

    if let Some(expected) = size {
        if expected != body_len {
            return json_response(
                StatusCode(400),
                serde_json::to_string(&json!({
                    "error": format!("size 与 Content-Length 不一致: {expected} != {body_len}")
                }))
                .unwrap_or_default(),
            );
        }
    }

    let dest = unique_dest(&state.dir, &filename);
    let mut output = match fs::File::create(&dest) {
        Ok(file) => file,
        Err(err) => {
            return json_response(
                StatusCode(500),
                serde_json::to_string(&json!({ "error": format!("创建目标文件失败: {err}") }))
                    .unwrap_or_default(),
            )
        }
    };

    let mut reader = request.as_reader();
    let copied = match copy_limited(&mut reader, &mut output, body_len) {
        Ok(copied) => copied,
        Err(err) => {
            let _ = fs::remove_file(&dest);
            return json_response(
                StatusCode(500),
                serde_json::to_string(&json!({ "error": format!("接收数据失败: {err}") }))
                    .unwrap_or_default(),
            );
        }
    };

    if copied != body_len {
        let _ = fs::remove_file(&dest);
        return json_response(
            StatusCode(400),
            serde_json::to_string(&json!({
                "error": format!("数据不完整: 收到 {copied} 字节，期望 {body_len} 字节")
            }))
            .unwrap_or_default(),
        );
    }

    if let Err(err) = output.flush() {
        let _ = fs::remove_file(&dest);
        return json_response(
            StatusCode(500),
            serde_json::to_string(&json!({ "error": format!("刷新文件失败: {err}") }))
                .unwrap_or_default(),
        );
    }

    json_response(
        StatusCode(200),
        serde_json::to_string(&json!({
            "ok": true,
            "name": filename,
            "size": copied,
        }))
        .unwrap_or_default(),
    )
}

fn parse_query_param(query: &str, key: &str) -> Option<String> {
    for part in query.split('&') {
        if let Some((k, v)) = part.split_once('=') {
            if k == key {
                return Some(v.to_string());
            }
        }
    }
    None
}

fn copy_limited(
    reader: &mut dyn Read,
    writer: &mut fs::File,
    limit: u64,
) -> Result<u64, String> {
    let mut remaining = limit;
    let mut copied = 0u64;
    let mut buf = [0u8; 64 * 1024];
    while remaining > 0 {
        let want = remaining.min(buf.len() as u64) as usize;
        let n = reader
            .read(&mut buf[..want])
            .map_err(|e| format!("读取请求体失败: {e}"))?;
        if n == 0 {
            break;
        }
        writer
            .write_all(&buf[..n])
            .map_err(|e| format!("写入文件失败: {e}"))?;
        copied += n as u64;
        remaining -= n as u64;
    }
    Ok(copied)
}

fn unix_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn sanitize_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | '\0' | '\r' | '\n' => '_',
            other => other,
        })
        .collect();
    let cleaned = cleaned.trim();
    if cleaned.is_empty() || cleaned == "." || cleaned == ".." {
        format!("upload_{}", unix_millis())
    } else {
        cleaned.to_string()
    }
}

fn unique_dest(dir: &Path, name: &str) -> PathBuf {
    let path = dir.join(name);
    if !path.exists() {
        return path;
    }
    let (stem, ext) = match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => (stem.to_string(), format!(".{ext}")),
        _ => (name.to_string(), String::new()),
    };
    for i in 1.. {
        let candidate = dir.join(format!("{stem}_{i}{ext}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!()
}
