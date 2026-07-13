#!/bin/bash
set -e

echo "🔧 Android SDK/NDK 安装脚本"
echo "=============================="

# 安装目录
INSTALL_DIR="$HOME/Android"
SDK_DIR="$INSTALL_DIR/Sdk"
CMDLINE_TOOLS_VERSION="11076708"  # 最新版本
NDK_VERSION="27.0.12077973"
JAVA17_HOME="/usr/lib/jvm/java-17-openjdk-amd64"

run_sdkmanager() {
    JAVA_HOME="$JAVA17_HOME" PATH="$JAVA17_HOME/bin:$PATH" sdkmanager "$@"
}

# 检查是否已安装
if [ -d "$SDK_DIR" ]; then
    echo "⚠️  Android SDK 已存在于 $SDK_DIR"
    read -p "是否继续安装？(y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
fi

# 创建安装目录
echo "📁 创建安装目录..."
mkdir -p "$INSTALL_DIR"
cd "$INSTALL_DIR"

# 下载命令行工具
echo "📥 下载 Android 命令行工具..."
CMDLINE_ZIP="commandlinetools-linux-${CMDLINE_TOOLS_VERSION}_latest.zip"

if [ ! -x "$SDK_DIR/cmdline-tools/latest/bin/sdkmanager" ]; then
    rm -rf cmdline-tools-tmp

    if [ -f "$CMDLINE_ZIP" ] && ! unzip -tq "$CMDLINE_ZIP" > /dev/null 2>&1; then
        echo "⚠️  Android 命令行工具压缩包已损坏，重新下载..."
        rm -f "$CMDLINE_ZIP"
    fi

    if [ ! -f "$CMDLINE_ZIP" ]; then
        wget -q --show-progress "https://dl.google.com/android/repository/$CMDLINE_ZIP"
    fi

    if ! unzip -tq "$CMDLINE_ZIP" > /dev/null 2>&1; then
        echo "❌ Android 命令行工具压缩包校验失败: $INSTALL_DIR/$CMDLINE_ZIP"
        echo "请检查网络或代理后重试。"
        exit 1
    fi

    # 解压
    echo "📦 解压命令行工具..."
    unzip -q "$CMDLINE_ZIP" -d cmdline-tools-tmp
    mkdir -p "$SDK_DIR/cmdline-tools"
    rm -rf "$SDK_DIR/cmdline-tools/latest"
    mv cmdline-tools-tmp/cmdline-tools "$SDK_DIR/cmdline-tools/latest"
    rm -rf cmdline-tools-tmp "$CMDLINE_ZIP"
else
    echo "✅ Android 命令行工具已存在，跳过下载。"
fi

# 设置环境变量
echo "🔧 配置环境变量..."
export ANDROID_HOME="$SDK_DIR"
export ANDROID_SDK_ROOT="$SDK_DIR"
export PATH="$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$PATH"

if [ ! -x "$JAVA17_HOME/bin/java" ]; then
    echo "❌ 未找到 Java 17: $JAVA17_HOME"
    echo "请先安装: sudo apt install -y openjdk-17-jdk"
    exit 1
fi

# 接受许可证
echo "✅ 接受许可证..."
yes | run_sdkmanager --licenses > /dev/null 2>&1 || true

# 安装必要的组件
echo "📦 安装 Android SDK 组件..."
run_sdkmanager "platform-tools" "platforms;android-34" "build-tools;34.0.0"

echo "📦 安装 Android NDK..."
run_sdkmanager "ndk;$NDK_VERSION"

# 设置 NDK 环境变量
export ANDROID_NDK_HOME="$SDK_DIR/ndk/$NDK_VERSION"

# 添加到 shell 配置
SHELL_RC="$HOME/.bashrc"
if [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
fi

echo ""
echo "📝 添加环境变量到 $SHELL_RC..."
cat >> "$SHELL_RC" << EOF

# Android SDK/NDK
export ANDROID_HOME="$SDK_DIR"
export ANDROID_SDK_ROOT="$SDK_DIR"
export ANDROID_NDK_HOME="$SDK_DIR/ndk/$NDK_VERSION"
export PATH="\$ANDROID_HOME/cmdline-tools/latest/bin:\$ANDROID_HOME/platform-tools:\$PATH"
EOF

echo ""
echo "✅ 安装完成！"
echo ""
echo "🔧 下一步："
echo "   1. 重新加载 shell 配置: source $SHELL_RC"
echo "   2. 验证安装: adb devices"
echo "   3. 构建项目: cd /home/zdz/Documents/Try/Rust/learning/gpui_mobile && ./build.sh"
echo ""
echo "📱 连接 Android 设备后："
echo "   - 启用 USB 调试（开发者选项）"
echo "   - 运行: ./install.sh"
