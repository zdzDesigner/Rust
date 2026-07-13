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

# 安装 APK
APK_PATH="android/app/build/outputs/apk/debug/app-debug.apk"

if [ ! -f "$APK_PATH" ]; then
    echo "❌ APK not found. Run ./build.sh first."
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
