# Rust Android 开发环境检查清单

## 当前状态

✅ **已安装**：
- Rust 工具链 (rustc 1.97.0)
- Android 目标 (aarch64-linux-android)
- cargo-ndk 工具
- adb 工具

❌ **缺少**：
- Android SDK
- Android NDK
- Gradle

## 快速安装（推荐）

运行一键安装脚本：

```bash
cd /home/zdz/Documents/Try/Rust/learning/gpui_mobile
./setup_android_sdk.sh
```

这会自动安装：
- Android SDK (API 34)
- Android NDK (r27)
- 构建工具 (build-tools 34.0.0)
- 配置环境变量

安装完成后：

```bash
# 1. 重新加载 shell
source ~/.bashrc  # 或 source ~/.zshrc

# 2. 验证安装
adb devices
echo $ANDROID_NDK_HOME

# 3. 构建项目
./build.sh

# 4. 安装到设备（需要先连接 Android 手机）
./install.sh
```

## 手动安装步骤

如果自动脚本失败，按以下步骤手动安装：

### 1. 下载命令行工具

```bash
mkdir -p ~/Android/Sdk
cd ~/Android/Sdk

# 下载
wget https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip

# 解压
unzip commandlinetools-linux-11076708_latest.zip
mkdir -p cmdline-tools/latest
mv cmdline-tools/* cmdline-tools/latest/ 2>/dev/null || true
```

### 2. 设置环境变量

```bash
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_SDK_ROOT=$ANDROID_HOME
export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$PATH
```

### 3. 安装 SDK 组件

```bash
# 接受许可证
yes | sdkmanager --licenses

# 安装组件
sdkmanager "platform-tools" "platforms;android-34" "build-tools;34.0.0"
sdkmanager "ndk;27.0.12077973"
```

### 4. 验证安装

```bash
adb version
sdkmanager --list_installed
```

## 构建和运行

### 构建 APK

```bash
cd /home/zdz/Documents/Try/Rust/learning/gpui_mobile
./build.sh
```

这会：
1. 编译 Rust 代码为 Android 共享库 (.so)
2. 将 .so 复制到 Android 项目
3. 构建 debug APK

产物位置：`android/app/build/outputs/apk/debug/app-debug.apk`

### 安装到设备

```bash
./install.sh
```

前提条件：
- Android 手机已连接
- 已启用 USB 调试（开发者选项）
- 已在手机上授权此电脑

### 查看日志

```bash
adb logcat | grep -E "(Rust|rust_android_demo)"
```

## 常见问题

### Q: sdkmanager 找不到？

确保环境变量已设置：

```bash
export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$PATH
```

### Q: NDK 路径不对？

检查 NDK 安装位置：

```bash
ls $ANDROID_HOME/ndk/
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/27.0.12077973
```

### Q: Gradle 下载失败？

Android 项目会自动下载 Gradle wrapper。如果失败：

```bash
cd android
./gradlew wrapper --gradle-version 8.4
```

### Q: 编译报错 "linker not found"？

确保 NDK 的 clang 在 PATH 中：

```bash
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH
```

### Q: 设备未授权？

在手机上点击"允许 USB 调试"对话框。如果没看到：

```bash
adb kill-server
adb start-server
adb devices
```

## 项目结构说明

```
gpui_mobile/
├── src/lib.rs                    # Rust JNI 代码
├── android/                      # Android 项目
│   ├── app/src/main/
│   │   ├── java/.../MainActivity.java
│   │   └── AndroidManifest.xml
│   └── build.gradle
├── build.sh                      # 构建脚本
├── install.sh                    # 安装脚本
└── setup_android_sdk.sh          # SDK 安装脚本
```

## 下一步学习

1. **理解 JNI 工作原理**
   - 阅读 `src/lib.rs` 中的函数签名规则
   - 理解 `#[no_mangle]` 和 `extern "C"` 的作用

2. **扩展功能**
   - 添加更多 JNI 函数（数组、对象）
   - 实现 Java → Rust 回调
   - 使用 `ndk-glue` 创建纯 Rust 应用

3. **参考项目**
   - gpui-mobile: https://github.com/itsbalamurali/gpui-mobile
   - winit: https://github.com/rust-windowing/winit
   - android-activity: https://github.com/rib/android-activity

## 与 Go/Zig 的关键差异

| 方面 | Rust | Go | Zig |
|------|------|-----|-----|
| **FFI 声明** | `extern "C"` + `#[no_mangle]` | `//export` | `export` |
| **库类型** | `cdylib` | `c-shared` | `shared` |
| **字符串传递** | 手动管理 (JNI) | GC 管理 | 手动管理 |
| **跨编译** | `cargo ndk` | `GOOS=android` | `zig build` |
| **运行时** | 无 | goroutine + GC | 无 |

**Rust 的优势**：零成本抽象，无运行时开销，适合性能敏感场景。

**Rust 的劣势**：FFI 边界需要更多手动代码，学习曲线陡峭。
