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

## 构建

```bash
./build.sh
```

构建脚本会执行：

```bash
cargo ndk -t arm64-v8a -P 31 -o android/app/src/main/jniLibs build --release
```

`-P 31` 是必要的，否则 `gpui-mobile` 链接 Android `nativewindow` 时可能失败。

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

启动 Activity：

```text
dev.gpui.mobile.demo/android.app.NativeActivity
```

## 日志

```bash
adb logcat | grep -E "(mobile|gpui|AndroidRuntime)"
```

## 当前 UI

当前只渲染一个最小 GPUI 页面：

```text
GPUI Mobile
扫描二维码
```

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

后续可以继续把上游 `gpui-mobile/example/src/screens` 的多页面 Router 迁移进来。
