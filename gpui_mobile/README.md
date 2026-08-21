# GPUI Mobile Demo

这是基于 `https://github.com/itsbalamurali/gpui-mobile` 重构后的 Android 示例。

当前项目不再使用 `MainActivity + RustLib` 的普通 JNI 调用模式，而是使用：

- `gpui-mobile`
- `gpui`
- `android-activity`
- Android `NativeActivity`
- Rust `android_main(app: android_activity::AndroidApp)` 入口

## 结构

```text
mobile/
├── Cargo.toml
├── .cargo/config.toml
├── src/lib.rs
├── build.sh
├── install.sh
└── android/
    ├── build.gradle
    ├── gradle.properties
    ├── settings.gradle
    └── app/
        ├── build.gradle
        └── src/main/
            ├── AndroidManifest.xml
            ├── jniLibs/arm64-v8a/
            └── res/values/styles.xml
```

## 关键点

Rust 入口在 `src/lib.rs`：

```rust
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    // 初始化 gpui-mobile Android platform，并启动 GPUI Application。
}
```

Android 启动入口在 `android/app/src/main/AndroidManifest.xml`：

```xml
<activity android:name="dev.gpui.mobile.GpuiActivity">
    <meta-data
        android:name="android.app.lib_name"
        android:value="${nativeLibraryName}" />
</activity>
```

`android/app/build.gradle` 中 `nativeLibraryName` 设置为：

```gradle
manifestPlaceholders = [nativeLibraryName: 'mobile']
```

因此 APK 启动时会加载：

```text
libmobile.so
```

## 环境要求

- Java 11 可作为系统默认 Java
- Java 17 用于 Android Gradle 构建
- Android SDK API 34
- Android NDK `27.0.12077973`
- `cargo-ndk`
- Rust target：`aarch64-linux-android`

## 首次环境准备

新机器（或环境缺失时）先运行一次 SDK/NDK 安装脚本：

```bash
./setup_android_sdk.sh
```

脚本会：

- 安装 Android 命令行工具到 `~/Android/Sdk`
- 接受许可证并安装 `platform-tools`、`platforms;android-34`、`build-tools;34.0.0`、NDK `27.0.12077973`
- 把 `ANDROID_HOME`、`ANDROID_SDK_ROOT`、`ANDROID_NDK_HOME` 写入 `~/.bashrc`（或 `~/.zshrc`）

完成后重载 shell 并验证：

```bash
source ~/.zshrc   # 或 ~/.bashrc
adb devices
```

## 构建

每次修改代码后执行，产出 APK：

```bash
./build.sh
```

构建脚本会执行：

```bash
cargo ndk -t arm64-v8a -P 31 -o android/app/src/main/jniLibs build --release
```

`-P 31` 是必要的，否则 `gpui-mobile` 链接 Android `nativewindow` 时可能失败。

脚本流程：检查 cargo/adb/Java 17 → 添加 Android Rust 目标 → 安装 `cargo-ndk`
→ 编译 Rust 库到 jniLibs → Gradle `assembleDebug` 打包。

产物位于：

```text
android/app/build/outputs/apk/debug/app-debug.apk
```

## 验证

```bash
cargo ndk -t arm64-v8a check
file android/app/build/outputs/apk/debug/app-debug.apk
zipinfo -1 android/app/build/outputs/apk/debug/app-debug.apk | grep 'lib/arm64-v8a'
```

期望 APK 包含：

```text
lib/arm64-v8a/libgpui_mobile-*.so
lib/arm64-v8a/libmobile.so
```

## 安装运行

```bash
./install.sh
```

安装 APK 并启动 App（脚本末尾的 `adb logcat` 会持续输出日志，Ctrl+C 退出）。

启动 Activity：

```text
dev.gpui.mobile.demo/android.app.NativeActivity
```

也可手动安装：

```bash
adb install -r android/app/build/outputs/apk/debug/app-debug.apk
adb shell am start -n dev.gpui.mobile.demo/dev.gpui.mobile.GpuiActivity
```

## 日志

```bash
adb logcat | grep -E "(mobile|gpui|AndroidRuntime)"
```

## 当前 UI

当前首页提供扫码入口和登录界面跳转入口：

```text
GPUI Mobile
扫描二维码
跳转登录界面
```

登录界面是 GPUI 绘制的静态页面，包含深蓝背景、Battery 标识、用户登录卡片、
用户名/密码输入框和登录按钮。输入框点击后会通过 `gpui-mobile` 文本输入回调
显示 Android 软键盘并更新 GPUI 状态，密码以 `*` 掩码显示。

Android 启动窗口背景在 `android/app/src/main/res/values/styles.xml` 和
`android/app/src/main/res/values-v31/styles.xml` 中固定为白色。Launcher 使用
`GpuiActivity extends NativeActivity`，并通过 AndroidX SplashScreen 持续显示
启动画面，直到 Rust 侧 `NATIVE_INITIALIZED` 变为 true，避免真机冷启动加载
native 库期间露出桌面背景。

Android 系统返回键在 `vendor/gpui-mobile/src/android/jni.rs` 中识别为
`AKEYCODE_BACK`，事件会继续分发给 GPUI，同时返回 `InputStatus::Unhandled`，
让 `NativeActivity` 执行默认返回行为并退出到桌面。

二维码扫描通过 `gpui_mobile::packages::qr_scanner::scan_qr_code()` 调用
Android 侧 `GpuiQrScanner`，由 ZXing Android Embedded 打开摄像头扫码并把
二维码文本返回给 GPUI 页面。Manifest 已声明 `android.permission.CAMERA`，
扫码前会通过 `permission_handler` 请求运行时摄像头权限。

当二维码内容是 `https://` 或无协议域名网址时，首页会通过
`gpui_mobile::packages::webview::open_url()` 启动独立的
`GpuiWebViewActivity` 打开原生 Android WebView。这里不使用嵌入式
platform view，避免 NativeActivity 的 Vulkan surface 与 WebView 叠加时出现
空白或黑屏。Manifest 已声明 `android.permission.INTERNET` 和
`ACCESS_NETWORK_STATE`。

当二维码内容不是网址时，首页会把内容通过
`gpui_mobile::packages::webview::open_html()` 在同一个 WebView Activity 中打开：
看起来像 Markdown 的内容会先用 `pulldown-cmark` 转成 HTML，普通文本会做
HTML escape 后放入 `<pre>`。HTML 内容路径会关闭 JavaScript。

## PC ↔ Android 文件互传

PC 端提供两个等价实现（协议一致，任选其一）：

- `crates/file-server/`：Rust 版（tiny_http + qrcode，二进制约 2MB）
- `golang/file_server/`：Go 版（标准库 + skip2/go-qrcode，二进制约 9MB）

Android 端通过扫码直连。

### 使用流程

1. PC 上启动服务（监听随机端口，自动打开浏览器显示二维码）：

   ```bash
   ./file_server.sh [共享目录]   # 默认 ./shared，首次运行自动构建
   ```

   或手动构建启动：

   ```bash
   cd crates/file-server
   cargo build --release
   ./target/release/file-server [共享目录]   # 默认 ./shared
   ```

   Go 版：

   ```bash
   cd golang/file_server
   go build -o file-server .
   ./file-server [共享目录]   # 默认 ./shared
   ```

2. 手机与 PC 连接**同一 WiFi**，打开 App 点击「扫描二维码」扫 PC 网页上的码。
3. 扫码后 App 探活 `/api/list`：命中则进入「文件传输」页，否则按原逻辑用
   WebView 打开。

### 功能

- 手机 → PC：点「上传文件」，经 SAF 选文件后上传到 PC 共享目录
  （`GpuiFilePicker.copyToCache` 先把 content:// URI 拷入 cache 再流式发送）。
- PC → 手机：文件列表逐项「下载」，流式下载到缓存后经
  `GpuiDownloads.saveToDownloads` 存入公共**下载（Downloads）目录**：
  API 29+ 走 `MediaStore.Downloads`（免权限），API 26-28 走公共目录
  （需 `WRITE_EXTERNAL_STORAGE`，已声明 `maxSdkVersion=28`）。
- 两端均显示实时进度（每 200ms 刷新一次）。

### 协议（极简 HTTP，免配对，同局域网）

| 接口 | 说明 |
|------|------|
| `GET /` | 网页：二维码 + 文件列表 + 拖拽上传 |
| `GET /api/list` | JSON `[{name,size,mtime}]`，兼作探活 |
| `GET /api/file/<url编码名>` | 流式下载 |
| `POST /api/upload?name=<url编码>&size=<字节数>` | body 为纯文件字节，`size` 必须与 Content-Length 一致 |

Android 端 HTTP 客户端（`src/transfer.rs`）为纯 `std::net::TcpStream` 实现，
在 GPUI background_executor 中运行，全程流式读写（64KB 缓冲），不引入
tokio/reqwest。

后续可以继续把上游 `gpui-mobile/example/src/screens` 的多页面 Router 迁移进来。
