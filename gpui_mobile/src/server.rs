//! 手机端 HTTP 文件服务器：app 作为服务器，PC 浏览器直连。
//!
//! [输入]: 私有共享目录路径；本机 WiFi IP
//! [输出]: HTTP 服务（网页 UI + 文件列表/下载/上传 API）；二维码 URL；本地文件导入
//! [定位]: 文件互传功能的网络层，跑在独立 std 线程，与 GPUI 主线程解耦
//! [同步]: crates/file-server/src/main.rs（协议约定）、src/server_page.rs

use std::fs;
use std::io::{Read, Write};
use std::net::UdpSocket;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use qrcode::render::svg;
use qrcode::QrCode;
use serde_json::json;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

use crate::transfer::FileEntry;

const INDEX_HTML: &str = include_str!("../crates/file-server/src/index.html");
const POLL_INTERVAL: Duration = Duration::from_millis(200);
const WORKERS: usize = 2;
const MAX_UPLOAD_BYTES: u64 = 2 * 1024 * 1024 * 1024;

pub struct RunningServer {
    pub dir: PathBuf,
    pub ip: String,
    pub port: u16,
    server: Arc<Server>,
    shutdown: Arc<AtomicBool>,
}

impl RunningServer {
    pub fn start(dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(dir).map_err(|e| format!("创建共享目录 {} 失败: {e}", dir.display()))?;
        let ip = wifi_ip().ok_or_else(|| "无法获取本机 WiFi IP，请确认已连接 WiFi".to_string())?;
        let bind = format!("{ip}:0");
        let server =
            Arc::new(Server::http(&bind).map_err(|e| format!("绑定 WiFi 地址 {bind} 失败: {e}"))?);
        let port = server
            .server_addr()
            .to_ip()
            .map(|addr| addr.port())
            .ok_or_else(|| "获取 HTTP 服务端口失败".to_string())?;
        let shutdown = Arc::new(AtomicBool::new(false));
        for worker_index in 0..WORKERS {
            let worker_server = Arc::clone(&server);
            let worker_shutdown = Arc::clone(&shutdown);
            let dir = dir.to_path_buf();
            let ip = ip.clone();
            if let Err(err) = std::thread::Builder::new()
                .name(format!("file-server-{worker_index}"))
                .spawn(move || {
                    worker_loop(
                        worker_server,
                        worker_shutdown,
                        SharedState { dir, ip, port },
                    )
                })
            {
                shutdown.store(true, Ordering::SeqCst);
                server.unblock();
                return Err(format!("启动服务线程失败: {err}"));
            }
        }
        Ok(Self {
            dir: dir.to_path_buf(),
            ip,
            port,
            server,
            shutdown,
        })
    }

    pub fn url(&self) -> String {
        format!("http://{}:{}/", self.ip, self.port)
    }

    pub fn stop(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        self.server.unblock();
    }
}

impl Drop for RunningServer {
    fn drop(&mut self) {
        self.stop();
    }
}

struct SharedState {
    dir: PathBuf,
    ip: String,
    port: u16,
}

fn worker_loop(server: Arc<Server>, shutdown: Arc<AtomicBool>, state: SharedState) {
    loop {
        if shutdown.load(Ordering::SeqCst) {
            return;
        }
        match server.recv_timeout(POLL_INTERVAL) {
            Ok(Some(request)) => handle(request, &state),
            Ok(None) => continue,
            Err(_) => continue,
        }
    }
}

/// 读取本地共享目录文件列表（供 app 页面 UI 直接使用）。
pub fn list_local(dir: &Path) -> Result<Vec<FileEntry>, String> {
    let mut entries = Vec::new();
    let read_dir = fs::read_dir(dir).map_err(|e| format!("读取共享目录失败: {e}"))?;
    for item in read_dir {
        let item = item.map_err(|e| format!("读取目录项失败: {e}"))?;
        let file_type = item
            .file_type()
            .map_err(|e| format!("获取文件类型失败: {e}"))?;
        if !file_type.is_file() {
            continue;
        }
        let name = item.file_name().to_string_lossy().into_owned();
        let metadata = item
            .metadata()
            .map_err(|e| format!("获取文件信息失败: {e}"))?;
        let mtime = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        entries.push(FileEntry {
            name,
            size: metadata.len(),
            mtime,
        });
    }
    entries.sort_by(|a, b| b.mtime.cmp(&a.mtime).then_with(|| a.name.cmp(&b.name)));
    Ok(entries)
}

fn wifi_ip() -> Option<String> {
    if let Ok(info) = gpui_mobile::packages::network_info::get_network_info() {
        if let Some(ip) = info.wifi_ip {
            if !ip.is_empty() && ip != "0.0.0.0" {
                return Some(ip);
            }
        }
    }
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
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
    let qr = qr_svg(&format!("http://{}:{}/", state.ip, state.port));
    let body = INDEX_HTML.replace("__QR_SVG__", &qr).replace(
        "手机扫描二维码连接此 PC",
        "PC 浏览器访问此地址，上传或下载文件",
    );
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
            log::warn!("二维码生成失败: {err}");
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
    match list_local(&state.dir) {
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
            let content_type =
                Header::from_bytes(&b"Content-Type"[..], b"application/octet-stream")
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

    if body_len > MAX_UPLOAD_BYTES {
        return json_response(
            StatusCode(413),
            serde_json::to_string(&json!({
                "error": format!("文件过大: {body_len} 字节，上限 {MAX_UPLOAD_BYTES}")
            }))
            .unwrap_or_default(),
        );
    }

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

    let (dest, mut output) = match unique_dest(&state.dir, &filename) {
        Ok(result) => result,
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

fn copy_limited(reader: &mut dyn Read, writer: &mut fs::File, limit: u64) -> Result<u64, String> {
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

fn unique_dest(dir: &Path, name: &str) -> Result<(PathBuf, fs::File), String> {
    let (stem, ext) = match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => (stem.to_string(), format!(".{ext}")),
        _ => (name.to_string(), String::new()),
    };
    let mut i = 1;
    loop {
        let candidate = if i == 1 {
            dir.join(name)
        } else {
            dir.join(format!("{stem}_{}{ext}", i - 1))
        };
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => return Ok((candidate, file)),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                i += 1;
                continue;
            }
            Err(err) => {
                return Err(format!("创建目标文件 {} 失败: {err}", candidate.display()));
            }
        }
    }
}

pub fn import_file(dir: &Path, cache_path: &Path, name: &str) -> Result<String, String> {
    let filename = sanitize_filename(name);
    let (dest, mut output) = unique_dest(dir, &filename)?;
    let mut input = match fs::File::open(cache_path) {
        Ok(file) => file,
        Err(err) => {
            let _ = fs::remove_file(&dest);
            return Err(format!(
                "打开待导入文件 {} 失败: {err}",
                cache_path.display()
            ));
        }
    };

    if let Err(err) = std::io::copy(&mut input, &mut output) {
        let _ = fs::remove_file(&dest);
        return Err(format!("导入文件 {} 失败: {err}", cache_path.display()));
    }
    if let Err(err) = output.flush() {
        let _ = fs::remove_file(&dest);
        return Err(format!("刷新导入文件 {} 失败: {err}", dest.display()));
    }
    fs::remove_file(cache_path)
        .map_err(|e| format!("删除导入缓存 {} 失败: {e}", cache_path.display()))?;
    Ok(filename)
}
