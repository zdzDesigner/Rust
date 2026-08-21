// 局域网文件互传 PC 端服务（Go 版）。
//
// 协议与 Rust 版 crates/file-server 完全一致：
//
//	GET  /                         网页：二维码 + 文件列表 + 拖拽上传
//	GET  /api/list                 JSON [{name,size,mtime}]
//	GET  /api/file/<url编码名>     流式下载
//	POST /api/upload?name=&size=   body 为纯文件字节，size 必须等于 Content-Length
package main

import (
	_ "embed"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"
	"time"

	qrcode "github.com/skip2/go-qrcode"
)

//go:embed index.html
var indexHTML string

type fileEntry struct {
	Name  string `json:"name"`
	Size  int64  `json:"size"`
	Mtime int64  `json:"mtime"`
}

type server struct {
	shareDir string
	port     int
	qrPNG    string
}

func main() {
	shareDir := "./shared"
	if len(os.Args) > 1 {
		shareDir = os.Args[1]
	}
	if err := os.MkdirAll(shareDir, 0o755); err != nil {
		fmt.Fprintf(os.Stderr, "无法创建共享目录 %s: %v\n", shareDir, err)
		os.Exit(1)
	}

	listener, err := net.Listen("tcp", "0.0.0.0:0")
	if err != nil {
		fmt.Fprintf(os.Stderr, "启动 HTTP 服务失败: %v\n", err)
		os.Exit(1)
	}
	port := listener.Addr().(*net.TCPAddr).Port

	ip := lanIP()
	if ip == "" {
		fmt.Fprintln(os.Stderr, "无法确定本机局域网 IP")
		os.Exit(1)
	}

	srv := &server{
		shareDir: shareDir,
		port:     port,
		qrPNG:    qrDataURI(fmt.Sprintf("http://%s:%d/", ip, port)),
	}

	fmt.Printf("共享目录: %s\n", shareDir)
	fmt.Printf("手机访问: http://%s:%d/\n", ip, port)

	if err := openBrowser(fmt.Sprintf("http://127.0.0.1:%d/", port)); err != nil {
		fmt.Fprintf(os.Stderr, "无法自动打开浏览器（请手动访问 http://%s:%d/）: %v\n", ip, port, err)
	}

	httpServer := &http.Server{Handler: srv.routes()}
	httpServer.Serve(listener)
}

func (s *server) routes() http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/" {
			http.NotFound(w, r)
			return
		}
		s.serveIndex(w, r)
	})
	mux.HandleFunc("/api/list", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			methodNotAllowed(w)
			return
		}
		s.apiList(w, r)
	})
	mux.HandleFunc("/api/upload", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			methodNotAllowed(w)
			return
		}
		s.apiUpload(w, r)
	})
	mux.HandleFunc("/api/file/", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			methodNotAllowed(w)
			return
		}
		s.apiDownload(w, r)
	})
	return mux
}

func methodNotAllowed(w http.ResponseWriter) {
	writeJSON(w, http.StatusMethodNotAllowed, map[string]string{"error": "不支持的方法"})
}

func writeJSON(w http.ResponseWriter, status int, body any) {
	data, err := json.Marshal(body)
	if err != nil {
		data = []byte(`{"error":"序列化失败"}`)
	}
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.WriteHeader(status)
	w.Write(data)
}

func (s *server) serveIndex(w http.ResponseWriter, r *http.Request) {
	img := fmt.Sprintf(`<img src="%s" alt="二维码">`, s.qrPNG)
	body := strings.Replace(indexHTML, "__QR_IMG__", img, 1)
	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	w.Write([]byte(body))
}

func (s *server) apiList(w http.ResponseWriter, r *http.Request) {
	entries, err := listEntries(s.shareDir)
	if err != nil {
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
		return
	}
	writeJSON(w, http.StatusOK, entries)
}

func listEntries(dir string) ([]fileEntry, error) {
	items, err := os.ReadDir(dir)
	if err != nil {
		return nil, fmt.Errorf("读取共享目录失败: %w", err)
	}
	entries := make([]fileEntry, 0, len(items))
	for _, item := range items {
		if item.IsDir() {
			continue
		}
		info, err := item.Info()
		if err != nil {
			return nil, fmt.Errorf("获取文件信息失败: %w", err)
		}
		entries = append(entries, fileEntry{
			Name:  item.Name(),
			Size:  info.Size(),
			Mtime: info.ModTime().Unix(),
		})
	}
	sort.Slice(entries, func(i, j int) bool {
		if entries[i].Mtime != entries[j].Mtime {
			return entries[i].Mtime > entries[j].Mtime
		}
		return entries[i].Name < entries[j].Name
	})
	return entries, nil
}

func (s *server) apiDownload(w http.ResponseWriter, r *http.Request) {
	name, err := url.PathUnescape(strings.TrimPrefix(r.URL.Path, "/api/file/"))
	if err != nil || !safeName(name) {
		writeJSON(w, http.StatusBadRequest, map[string]string{"error": "非法文件名"})
		return
	}

	path := filepath.Join(s.shareDir, name)
	file, err := os.Open(path)
	if err != nil {
		msg := "打开文件失败"
		if errors.Is(err, os.ErrNotExist) {
			msg = "文件不存在"
		}
		writeJSON(w, http.StatusNotFound, map[string]string{"error": fmt.Sprintf("%s: %v", msg, err)})
		return
	}
	defer file.Close()

	info, err := file.Stat()
	if err != nil {
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": fmt.Sprintf("获取文件信息失败: %v", err)})
		return
	}

	w.Header().Set("Content-Type", "application/octet-stream")
	w.Header().Set("Content-Disposition", contentDisposition(name))
	w.Header().Set("Content-Length", strconv.FormatInt(info.Size(), 10))
	io.Copy(w, file)
}

func safeName(name string) bool {
	return name != "" && name != "." && name != ".." &&
		!strings.ContainsAny(name, "/\\\x00")
}

func contentDisposition(name string) string {
	fallback := strings.Map(func(c rune) rune {
		if c < 128 {
			return c
		}
		return '_'
	}, name)
	return fmt.Sprintf(`attachment; filename="%s"; filename*=UTF-8''%s`,
		fallback, url.PathEscape(name))
}

// 上传协议：POST /api/upload?name=<url编码>&size=<字节数>，body 为纯文件字节。
// size 存在时必须与 Content-Length 一致，防止截断文件落盘。
func (s *server) apiUpload(w http.ResponseWriter, r *http.Request) {
	sizeStr := r.URL.Query().Get("size")
	bodyLen := r.ContentLength

	if sizeStr != "" {
		size, err := strconv.ParseInt(sizeStr, 10, 64)
		if err != nil || size != bodyLen {
			writeJSON(w, http.StatusBadRequest, map[string]string{
				"error": fmt.Sprintf("size 与 Content-Length 不一致: %s != %d", sizeStr, bodyLen),
			})
			return
		}
	}
	if bodyLen < 0 {
		writeJSON(w, http.StatusBadRequest, map[string]string{"error": "缺少 Content-Length"})
		return
	}

	name := sanitizeName(r.URL.Query().Get("name"))
	dest, err := uniqueDest(s.shareDir, name)
	if err != nil {
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": err.Error()})
		return
	}

	file, err := os.Create(dest)
	if err != nil {
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": fmt.Sprintf("创建目标文件失败: %v", err)})
		return
	}

	written, err := io.Copy(file, io.LimitReader(r.Body, bodyLen))
	closeErr := file.Close()
	if err != nil {
		os.Remove(dest)
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": fmt.Sprintf("接收数据失败: %v", err)})
		return
	}
	if closeErr != nil {
		os.Remove(dest)
		writeJSON(w, http.StatusInternalServerError, map[string]string{"error": fmt.Sprintf("关闭文件失败: %v", closeErr)})
		return
	}
	if written != bodyLen {
		os.Remove(dest)
		writeJSON(w, http.StatusBadRequest, map[string]string{
			"error": fmt.Sprintf("数据不完整: 收到 %d 字节，期望 %d 字节", written, bodyLen),
		})
		return
	}

	writeJSON(w, http.StatusOK, map[string]any{
		"ok":   true,
		"name": filepath.Base(dest),
		"size": written,
	})
}

func sanitizeName(name string) string {
	cleaned := strings.Map(func(c rune) rune {
		switch c {
		case '/', '\\', '\x00', '\r', '\n':
			return '_'
		}
		return c
	}, name)
	cleaned = strings.TrimSpace(cleaned)
	if cleaned == "" || cleaned == "." || cleaned == ".." {
		return fmt.Sprintf("upload_%d", time.Now().UnixMilli())
	}
	return cleaned
}

func uniqueDest(dir, name string) (string, error) {
	dest := filepath.Join(dir, name)
	if _, err := os.Stat(dest); errors.Is(err, os.ErrNotExist) {
		return dest, nil
	}
	ext := filepath.Ext(name)
	stem := strings.TrimSuffix(name, ext)
	if stem == "" {
		stem = name
		ext = ""
	}
	for i := 1; ; i++ {
		candidate := filepath.Join(dir, fmt.Sprintf("%s_%d%s", stem, i, ext))
		if _, err := os.Stat(candidate); errors.Is(err, os.ErrNotExist) {
			return candidate, nil
		}
	}
}

// lanIP 通过 UDP connect 触发路由选择拿本机出口 IP（不产生真实流量）。
func lanIP() string {
	conn, err := net.Dial("udp", "8.8.8.8:80")
	if err != nil {
		return ""
	}
	defer conn.Close()
	if addr, ok := conn.LocalAddr().(*net.UDPAddr); ok {
		return addr.IP.String()
	}
	return ""
}

// qrDataURI 生成二维码 PNG 的 data URI，供网页 <img> 内嵌显示。
func qrDataURI(text string) string {
	png, err := qrcode.Encode(text, qrcode.Medium, 256)
	if err != nil {
		fmt.Fprintf(os.Stderr, "二维码生成失败: %v\n", err)
		return ""
	}
	return "data:image/png;base64," + base64.StdEncoding.EncodeToString(png)
}

func openBrowser(url string) error {
	var cmd *exec.Cmd
	switch runtime.GOOS {
	case "linux":
		cmd = exec.Command("xdg-open", url)
	case "darwin":
		cmd = exec.Command("open", url)
	case "windows":
		cmd = exec.Command("cmd", "/c", "start", "", url)
	default:
		return fmt.Errorf("当前平台 %s 不支持自动打开浏览器", runtime.GOOS)
	}
	return cmd.Start()
}
