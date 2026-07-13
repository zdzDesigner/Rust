# Rust Android Demo

最小可行的 Rust Android 应用示例，展示如何通过 JNI 调用 Rust 代码。

## 项目结构

```
gpui_mobile/
├── Cargo.toml              # Rust 项目配置
├── src/
│   └── lib.rs              # Rust JNI 实现
├── android/                # Android 项目
│   ├── app/
│   │   ├── src/main/
│   │   │   ├── java/com/example/rustdemo/
│   │   │   │   ├── MainActivity.java    # Android 主界面
│   │   │   │   └── RustLib.java         # JNI 声明
│   │   │   └── AndroidManifest.xml
│   │   └── build.gradle
│   └── build.gradle
├── build.sh                # 构建脚本
└── install.sh              # 安装脚本
```

## 前置要求

1. **Rust 工具链**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add aarch64-linux-android
   ```

2. **Android SDK/NDK**
   - 安装 Android Studio 或命令行工具
   - 确保 `adb` 在 PATH 中
   - 启用设备的 USB 调试

3. **cargo-ndk**
   ```bash
   cargo install cargo-ndk
   ```

## 构建步骤

### 1. 编译 Rust 库并构建 APK

```bash
./build.sh
```

这会：
- 编译 Rust 代码为 Android 共享库 (`.so`)
- 将 `.so` 复制到 Android 项目的 `jniLibs` 目录
- 构建 debug APK

### 2. 安装到设备

```bash
./install.sh
```

这会：
- 检测连接的 Android 设备
- 安装 APK
- 启动应用
- 显示 logcat 日志（过滤 Rust 相关输出）

## 工作原理

### Rust 侧 (src/lib.rs)

```rust
#[no_mangle]
pub extern "C" fn Java_com_example_rustdemo_RustLib_stringFromJNI(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    // 返回 Java 字符串
    let output = env.new_string("Hello from Rust! 🦀").unwrap();
    output.into_raw()
}
```

关键点：
- `#[no_mangle]`：禁止 Rust 修改函数名
- `extern "C"`：使用 C 调用约定
- 函数名格式：`Java_<包名>_<类名>_<方法名>`

### Java 侧 (RustLib.java)

```java
public class RustLib {
    public static native String stringFromJNI();
    public static native int addFromJNI(int a, int b);
}
```

声明 native 方法，JVM 会在加载库时自动链接到 Rust 实现。

### 加载库 (MainActivity.java)

```java
static {
    System.loadLibrary("rust_android_demo");
}
```

在类加载时加载 Rust 编译的共享库。

## 调试技巧

### 查看 Rust 日志

```bash
adb logcat | grep -E "(Rust|rust_android_demo)"
```

Rust 的 `log::info!` 会输出到 Android logcat。

### 检查库是否加载

```bash
adb shell run-as com.example.rustdemo ls lib/
```

应该能看到 `librust_android_demo.so`。

### 崩溃日志

```bash
adb logcat | grep AndroidRuntime
```

## 与 Go/Zig 的对比

| 特性 | Rust | Go | Zig |
|------|------|-----|-----|
| **FFI 调用约定** | `extern "C"` | `//export` | `export` |
| **库类型** | `cdylib` | `-buildmode=c-shared` | `-dynamic` |
| **符号控制** | `#[no_mangle]` | 自动导出 | `@export` |
| **内存管理** | 手动（JNI 边界） | GC | 手动 |
| **跨编译** | `cargo ndk` | `GOOS=android` | `zig build` |

**Rust 特点**：
- 需要显式 `#[no_mangle]` + `extern "C"`
- 必须用 `cdylib`（不是 `dylib`）
- JNI 边界需要手动管理字符串生命周期
- 零开销抽象，无运行时

## 常见问题

### Q: 编译报错 "linker not found"

确保安装了 Android NDK，并且 NDK 的 clang 在 PATH 中：

```bash
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH
```

### Q: 安装失败 "INSTALL_FAILED_NO_MATCHING_ABIS"

设备架构不匹配。默认构建的是 `arm64-v8a`，如果设备是 32 位：

```bash
cargo ndk -t armeabi-v7a -o android/app/src/main/jniLibs build --release
```

### Q: 应用闪退

查看 logcat：

```bash
adb logcat | grep -A 10 "FATAL EXCEPTION"
```

常见原因：
- 库未正确加载（检查 `System.loadLibrary`）
- JNI 函数签名不匹配（检查包名、类名）
- 内存访问错误（Rust 侧的 unsafe 代码）

## 下一步

- 添加更多 JNI 函数（数组、对象、回调）
- 使用 `ndk-glue` 创建纯 Rust Android 应用（无需 Java）
- 集成 `winit` 或 `android-activity` 实现图形界面
- 参考 `gpui-mobile` 项目学习完整的 GPUI 移植

## 参考资源

- [Rust JNI 文档](https://docs.rs/jni/latest/jni/)
- [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- [Android NDK 官方文档](https://developer.android.com/ndk/guides)
- [GPUI Mobile 项目](https://github.com/itsbalamurali/gpui-mobile)
