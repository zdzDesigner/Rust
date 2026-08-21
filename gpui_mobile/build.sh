#!/bin/bash
set -e

echo "🦀 Building GPUI Mobile Demo"
echo "=============================="

# 解析参数：--release 构建 release 版本，否则构建 debug 版本
BUILD_TYPE=debug
if [ "$1" = "--release" ]; then
    BUILD_TYPE=release
fi

# 检查必要的工具
command -v cargo >/dev/null 2>&1 || { echo "❌ cargo not found. Install Rust first."; exit 1; }
command -v adb >/dev/null 2>&1 || { echo "❌ adb not found. Install Android SDK first."; exit 1; }
command -v unzip >/dev/null 2>&1 || { echo "❌ unzip not found. Install unzip first."; exit 1; }
command -v wget >/dev/null 2>&1 || { echo "❌ wget not found. Install wget first."; exit 1; }

JAVA17_HOME="/usr/lib/jvm/java-17-openjdk-amd64"
GRADLE_VERSION="8.2.1"
GRADLE_URL="https://mirrors.aliyun.com/macports/distfiles/gradle/gradle-${GRADLE_VERSION}-bin.zip"

if [ ! -x "$JAVA17_HOME/bin/java" ]; then
    echo "❌ 未找到 Java 17: $JAVA17_HOME"
    echo "请先安装: sudo apt install -y openjdk-17-jdk"
    exit 1
fi

# 添加 Android 目标（如果未添加）
echo "📦 Ensuring Android targets are installed..."
rustup target add aarch64-linux-android 2>/dev/null || true

# 安装 cargo-ndk（如果未安装）
if ! command -v cargo-ndk >/dev/null 2>&1; then
    echo "📦 Installing cargo-ndk..."
    cargo install cargo-ndk
fi

# 编译 Rust 库
echo "🔨 Building GPUI Mobile library for Android (aarch64, $BUILD_TYPE)..."
if [ "$BUILD_TYPE" = "release" ]; then
    cargo ndk -t arm64-v8a -P 31 -o android/app/src/main/jniLibs build --release
else
    cargo ndk -t arm64-v8a -P 31 -o android/app/src/main/jniLibs build
fi

echo "✅ Rust library built successfully"

# 构建 Android APK
echo "🔨 Building Android APK..."
cd android

# 确保 gradlew 可执行
chmod +x gradlew 2>/dev/null || true

# 如果没有 gradlew，下载并使用本地 Gradle，避免 Ubuntu 源里的旧版本 Gradle。
if [ ! -f "./gradlew" ]; then
    echo "⚠️  gradlew not found, using local Gradle $GRADLE_VERSION..."

    GRADLE_DIR=".gradle/gradle-${GRADLE_VERSION}"
    GRADLE_ZIP=".gradle/gradle-${GRADLE_VERSION}-bin.zip"

    mkdir -p .gradle

    if [ ! -x "$GRADLE_DIR/bin/gradle" ]; then
        wget -c -q --show-progress "$GRADLE_URL" -O "$GRADLE_ZIP"

        if ! unzip -tq "$GRADLE_ZIP" > /dev/null 2>&1; then
            echo "❌ Gradle 压缩包校验失败: $PWD/$GRADLE_ZIP"
            echo "请检查网络或代理后重试。"
            exit 1
        fi

        unzip -q "$GRADLE_ZIP" -d .gradle
    fi

    JAVA_HOME="$JAVA17_HOME" PATH="$JAVA17_HOME/bin:$PATH" "$GRADLE_DIR/bin/gradle" "assemble${BUILD_TYPE^}"
else
    JAVA_HOME="$JAVA17_HOME" PATH="$JAVA17_HOME/bin:$PATH" ./gradlew "assemble${BUILD_TYPE^}"
fi

cd ..

echo "✅ APK built successfully"
echo ""
echo "📱 APK location: android/app/build/outputs/apk/$BUILD_TYPE/app-$BUILD_TYPE.apk"
echo ""
echo "🚀 To install on device:"
echo "   ./install.sh"
