//! 与 PC 端 file-server 的 HTTP 传输客户端。
//!
//! [输入]: PC 服务器地址（扫码获得）；本地文件路径；下载目标路径
//! [输出]: 文件列表/下载/上传/探活能力，纯 std::net::TcpStream 实现
//! [定位]: 文件互传功能的网络层，在 GPUI background_executor 中运行
//! [同步]: crates/file-server/src/main.rs（协议约定）

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{IpAddr, TcpStream};
use std::path::Path;
use std::time::Duration;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const IO_TIMEOUT: Duration = Duration::from_secs(30);
const PROBE_TIMEOUT: Duration = Duration::from_millis(2000);
const MAX_HEADER_BYTES: usize = 64 * 1024;
const MAX_LIST_BYTES: usize = 4 * 1024 * 1024;
const CHUNK_SIZE: usize = 64 * 1024;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    #[allow(dead_code)]
    pub mtime: u64,
}

#[derive(Debug, Clone)]
pub struct TransferServer {
    pub ip: IpAddr,
    pub port: u16,
}

impl TransferServer {
    /// 解析扫码得到的地址，如 `http://192.168.1.5:8080/`。
    pub fn parse(url: &str) -> Result<Self, String> {
        let rest = url
            .strip_prefix("http://")
            .ok_or_else(|| format!("仅支持 http:// 地址: {url}"))?;
        let rest = rest.trim_end_matches('/');
        let rest = rest.split('/').next().unwrap_or(rest);

        let (host, port) = match rest.rsplit_once(':') {
            Some((host, port)) => {
                let port: u16 = port
                    .parse()
                    .map_err(|_| format!("端口无效: {url}"))?;
                (host, port)
            }
            None => (rest, 80),
        };

        let ip: IpAddr = host
            .parse()
            .map_err(|_| format!("主机必须是 IP 地址: {host}"))?;
        Ok(Self { ip, port })
    }

    pub fn base(&self) -> String {
        format!("http://{}:{}", self.ip, self.port)
    }

    fn connect(&self, read_timeout: Duration) -> Result<TcpStream, String> {
        let stream = TcpStream::connect_timeout(&(self.ip, self.port).into(), CONNECT_TIMEOUT)
            .map_err(|e| format!("连接 {} 失败: {e}", self.base()))?;
        stream
            .set_read_timeout(Some(read_timeout))
            .map_err(|e| format!("设置读超时失败: {e}"))?;
        stream
            .set_write_timeout(Some(IO_TIMEOUT))
            .map_err(|e| format!("设置写超时失败: {e}"))?;
        Ok(stream)
    }
}

/// 探测地址是否是 file-server（GET /api/list 短超时）。
pub fn probe(server: &TransferServer) -> Result<(), String> {
    let mut stream = server.connect(PROBE_TIMEOUT)?;
    send_headers(
        &mut stream,
        &[
            "GET /api/list HTTP/1.1".to_string(),
            host_header(server),
            "Connection: close".to_string(),
            "Accept: application/json".to_string(),
        ],
    )?;
    let mut reader = BufReader::new(stream);
    let (status, _) = read_headers(&mut reader)?;
    if status == 200 {
        Ok(())
    } else {
        Err(format!("探活失败，服务器返回 {status}"))
    }
}

/// 获取服务器共享目录的文件列表。
pub fn list_files(server: &TransferServer) -> Result<Vec<FileEntry>, String> {
    let mut stream = server.connect(IO_TIMEOUT)?;
    send_headers(
        &mut stream,
        &[
            "GET /api/list HTTP/1.1".to_string(),
            host_header(server),
            "Connection: close".to_string(),
            "Accept: application/json".to_string(),
        ],
    )?;
    let mut reader = BufReader::new(stream);
    let (status, headers) = read_headers(&mut reader)?;
    if status != 200 {
        return Err(format!("获取文件列表失败，服务器返回 {status}"));
    }
    let expected = content_length(&headers)?;
    let body = read_limited(&mut reader, expected, MAX_LIST_BYTES)?;
    serde_json::from_slice(&body).map_err(|e| format!("解析文件列表失败: {e}"))
}

/// 下载文件到 dest，回调 (已拷贝字节, 总字节) 每 100ms 触发一次。
pub fn download(
    server: &TransferServer,
    name: &str,
    dest: &Path,
    mut on_progress: impl FnMut(u64, u64),
) -> Result<(), String> {
    let path = format!("/api/file/{}", encode_path_component(name));
    let mut stream = server.connect(IO_TIMEOUT)?;
    let reader_stream = stream
        .try_clone()
        .map_err(|e| format!("复制连接失败: {e}"))?;
    send_headers(
        &mut stream,
        &[
            format!("GET {path} HTTP/1.1"),
            host_header(server),
            "Connection: close".to_string(),
            "Accept: application/octet-stream".to_string(),
        ],
    )?;

    let mut reader = BufReader::new(reader_stream);
    let (status, headers) = read_headers(&mut reader)?;
    if status != 200 {
        return Err(format!("下载失败，服务器返回 {status}"));
    }
    let total = content_length(&headers)?;

    let file = File::create(dest).map_err(|e| format!("创建文件 {} 失败: {e}", dest.display()))?;
    let mut writer = BufWriter::with_capacity(CHUNK_SIZE, file);

    let mut copied = 0u64;
    let mut last_report = std::time::Instant::now();
    let mut buf = [0u8; CHUNK_SIZE];
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| format!("读取下载数据失败: {e}"))?;
        if n == 0 {
            break;
        }
        writer
            .write_all(&buf[..n])
            .map_err(|e| format!("写入下载文件失败: {e}"))?;
        copied += n as u64;
        if last_report.elapsed() >= Duration::from_millis(100) {
            last_report = std::time::Instant::now();
            on_progress(copied, total);
        }
    }
    writer.flush().map_err(|e| format!("刷新下载文件失败: {e}"))?;

    if total > 0 && copied != total {
        return Err(format!("下载不完整: 收到 {copied} 字节，期望 {total} 字节"));
    }

    on_progress(copied, copied.max(total));
    Ok(())
}

/// 上传本地文件，回调 (已发送字节, 总字节) 每 100ms 触发一次。
pub fn upload(
    server: &TransferServer,
    path: &Path,
    name: &str,
    mut on_progress: impl FnMut(u64, u64),
) -> Result<(), String> {
    let file = File::open(path).map_err(|e| format!("打开上传文件 {} 失败: {e}", path.display()))?;
    let total = file
        .metadata()
        .map_err(|e| format!("读取上传文件信息失败: {e}"))?
        .len();

    let url = format!(
        "/api/upload?name={}&size={}",
        encode_path_component(name),
        total
    );

    let mut stream = server.connect(IO_TIMEOUT)?;
    let reader_stream = stream
        .try_clone()
        .map_err(|e| format!("复制连接失败: {e}"))?;
    send_headers(
        &mut stream,
        &[
            format!("POST {url} HTTP/1.1"),
            host_header(server),
            "Connection: close".to_string(),
            "Content-Type: application/octet-stream".to_string(),
            format!("Content-Length: {total}"),
        ],
    )?;

    let mut reader = BufReader::with_capacity(CHUNK_SIZE, file);
    let mut sent = 0u64;
    let mut last_report = std::time::Instant::now();
    let mut buf = [0u8; CHUNK_SIZE];
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| format!("读取上传文件失败: {e}"))?;
        if n == 0 {
            break;
        }
        stream
            .write_all(&buf[..n])
            .map_err(|e| format!("发送上传数据失败: {e}"))?;
        sent += n as u64;
        if last_report.elapsed() >= Duration::from_millis(100) {
            last_report = std::time::Instant::now();
            on_progress(sent, total);
        }
    }

    let mut resp_reader = BufReader::new(reader_stream);
    let (status, headers) = read_headers(&mut resp_reader)?;
    let body_len = content_length(&headers)?;
    let body = read_limited(&mut resp_reader, body_len, MAX_LIST_BYTES)?;

    if status != 200 {
        let message = String::from_utf8_lossy(&body);
        return Err(format!("上传失败，服务器返回 {status}: {message}"));
    }

    on_progress(sent, total);
    Ok(())
}

/// 通过 JNI 调 GpuiFilePicker.copyToCache，把 SAF content:// URI 拷贝为可读路径。
#[cfg(target_os = "android")]
pub fn copy_content_to_cache(uri: &str) -> Result<String, String> {
    use gpui_mobile::android::jni as jni_helpers;
    use jni::objects::{JObject, JValue};

    jni_helpers::with_env(|env| {
        let activity = jni_helpers::activity(env)?;
        let cls = jni_helpers::find_app_class(env, "dev.gpui.mobile.GpuiFilePicker")?;
        let j_uri = env.new_string(uri).map_err(|e| e.to_string())?;

        let result = env
            .call_static_method(
                &cls,
                jni::jni_str!("copyToCache"),
                jni::jni_sig!("(Landroid/app/Activity;Ljava/lang/String;)Ljava/lang/String;"),
                &[JValue::Object(&activity), JValue::Object(&JObject::from(j_uri))],
            )
            .and_then(|v| v.l())
            .map_err(|e| {
                env.exception_clear();
                e.to_string()
            })?;

        if result.is_null() {
            return Err("copyToCache 返回 null".to_string());
        }
        Ok(jni_helpers::get_string(env, &result))
    })
}

#[cfg(not(target_os = "android"))]
pub fn copy_content_to_cache(uri: &str) -> Result<String, String> {
    let _ = uri;
    Err("copy_content_to_cache 仅支持 Android".to_string())
}

/// 通过 JNI 调 GpuiDownloads.saveToDownloads，把缓存文件保存到公共 Downloads 目录。
/// 返回保存位置的用户可读描述（如 "下载/文件名"）。
#[cfg(target_os = "android")]
pub fn save_to_downloads(cache_path: &str, name: &str) -> Result<String, String> {
    use gpui_mobile::android::jni as jni_helpers;
    use jni::objects::{JObject, JValue};

    jni_helpers::with_env(|env| {
        let activity = jni_helpers::activity(env)?;
        let cls = jni_helpers::find_app_class(env, "dev.gpui.mobile.GpuiDownloads")?;
        let j_path = env.new_string(cache_path).map_err(|e| e.to_string())?;
        let j_name = env.new_string(name).map_err(|e| e.to_string())?;

        let result = env
            .call_static_method(
                &cls,
                jni::jni_str!("saveToDownloads"),
                jni::jni_sig!(
                    "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"
                ),
                &[
                    JValue::Object(&activity),
                    JValue::Object(&JObject::from(j_path)),
                    JValue::Object(&JObject::from(j_name)),
                ],
            )
            .and_then(|v| v.l())
            .map_err(|e| {
                env.exception_clear();
                e.to_string()
            })?;

        if result.is_null() {
            return Err("saveToDownloads 返回 null".to_string());
        }
        Ok(jni_helpers::get_string(env, &result))
    })
}

#[cfg(not(target_os = "android"))]
pub fn save_to_downloads(cache_path: &str, name: &str) -> Result<String, String> {
    let _ = (cache_path, name);
    Err("save_to_downloads 仅支持 Android".to_string())
}

fn host_header(server: &TransferServer) -> String {
    format!("Host: {}:{}", server.ip, server.port)
}

fn send_headers(stream: &mut TcpStream, lines: &[String]) -> Result<(), String> {
    for line in lines {
        stream
            .write_all(line.as_bytes())
            .map_err(|e| format!("发送请求头失败: {e}"))?;
        stream
            .write_all(b"\r\n")
            .map_err(|e| format!("发送请求头失败: {e}"))?;
    }
    stream
        .write_all(b"\r\n")
        .map_err(|e| format!("发送请求头失败: {e}"))?;
    Ok(())
}

fn read_headers(reader: &mut BufReader<TcpStream>) -> Result<(u16, Vec<(String, String)>), String> {
    let mut head = Vec::new();
    let mut tail = [0u8; 4];
    let mut one = [0u8; 1];
    loop {
        reader
            .read_exact(&mut one)
            .map_err(|e| format!("读取响应头失败: {e}"))?;
        head.push(one[0]);
        tail.copy_within(1.., 0);
        tail[3] = one[0];
        if &tail == b"\r\n\r\n" {
            break;
        }
        if head.len() > MAX_HEADER_BYTES {
            return Err("响应头超过 64KB 上限".to_string());
        }
    }

    let text = String::from_utf8_lossy(&head[..head.len() - 4]);
    let mut lines = text.split("\r\n");
    let status_line = lines.next().unwrap_or("");
    let status = parse_status(status_line)?;

    let mut headers = Vec::new();
    for line in lines {
        if let Some((key, value)) = line.split_once(':') {
            headers.push((key.trim().to_ascii_lowercase(), value.trim().to_string()));
        }
    }

    Ok((status, headers))
}

fn parse_status(status_line: &str) -> Result<u16, String> {
    let mut parts = status_line.split_whitespace();
    let _version = parts.next().unwrap_or("");
    let code = parts
        .next()
        .and_then(|c| c.parse::<u16>().ok())
        .ok_or_else(|| format!("响应状态行无效: {status_line}"))?;
    Ok(code)
}

fn content_length(headers: &[(String, String)]) -> Result<u64, String> {
    for (key, value) in headers {
        if key == "content-length" {
            return value
                .parse::<u64>()
                .map_err(|_| format!("Content-Length 无效: {value}"));
        }
    }
    Ok(0)
}

fn read_limited(
    reader: &mut BufReader<TcpStream>,
    expected: u64,
    max_bytes: usize,
) -> Result<Vec<u8>, String> {
    let limit = if expected > 0 {
        (expected as usize).min(max_bytes)
    } else {
        max_bytes
    };
    let mut body = Vec::with_capacity(limit);
    let mut buf = [0u8; CHUNK_SIZE];
    loop {
        let want = (limit - body.len()).min(buf.len());
        if want == 0 {
            break;
        }
        let n = reader
            .read(&mut buf[..want])
            .map_err(|e| format!("读取响应体失败: {e}"))?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&buf[..n]);
    }
    if expected > 0 && (body.len() as u64) < expected {
        return Err(format!(
            "响应体不完整: 收到 {} 字节，期望 {expected} 字节",
            body.len()
        ));
    }
    Ok(body)
}

fn encode_path_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
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
