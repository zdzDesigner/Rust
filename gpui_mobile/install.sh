#!/bin/bash
set -e

echo "📱 Installing GPUI Mobile Demo"
echo "================================"

# 检查 adb
command -v adb >/dev/null 2>&1 || { echo "❌ adb not found"; exit 1; }

# 检查设备连接
echo "🔍 Checking connected devices..."
DEVICES=$(adb devices | grep -v "List of devices" | grep -v "^$" | wc -l)

if [ "$DEVICES" -eq 0 ]; then
    echo "❌ No device connected. Please:"
    echo "   1. Enable USB debugging on your Android device"
    echo "   2. Connect via USB"
    echo "   3. Authorize the computer on your device"
    exit 1
fi

echo "✅ Device(s) connected:"
adb devices
echo ""

# 解析参数：--release 安装 release 版本，否则安装 debug 版本
BUILD_TYPE=debug
if [ "$1" = "--release" ]; then
    BUILD_TYPE=release
fi

# 安装 APK
APK_PATH="android/app/build/outputs/apk/$BUILD_TYPE/app-$BUILD_TYPE.apk"

if [ ! -f "$APK_PATH" ]; then
    echo "❌ APK not found: $APK_PATH"
    echo "Run ./build.sh${1:+ $1} first."
    exit 1
fi

echo "📦 Installing APK..."
adb install -r "$APK_PATH"

echo ""
echo "✅ Installation successful!"
echo ""
echo "🚀 Launching app..."
adb shell am start -n dev.gpui.mobile.demo/dev.gpui.mobile.GpuiActivity

echo ""
echo "📋 Viewing logcat (Ctrl+C to exit):"
adb logcat -c  # 清除旧日志
adb logcat | grep -E "(mobile|gpui|AndroidRuntime)"
