#!/bin/bash


# export ANDROID_API=21
# export ANDROID_NDK_HOME=/home/zdz/Android/Sdk/ndk/21.4.7075529
# export NDK_HOME=/home/zdz/Android/Sdk/ndk/21.4.7075529
export ANDROID_API=27
export ANDROID_NDK_HOME=/home/zdz/Android/Sdk/ndk/27.0.12077973
export NDK_HOME=/home/zdz/Android/Sdk/ndk/27.0.12077973
export PATH=${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin:${PATH}
export ANDROID_SYSROOT=${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/sysroot
export ANDROID_TOOLCHAIN=${ANDROID_NDK_HOME}/toolchains/arm-linux-androideabi-4.9/prebuilt/linux-x86_64
# export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
export JAVA_HOME=/usr/lib/jvm/java-23-openjdk
# export ANDROID_USER_HOME=/home/zdz/Android/Sdk

export AR_aarch64_linux_android="llvm-ar"
export AR_armv7_linux_androideabi="llvm-ar"
export AR_x86_64_linux_android="llvm-ar"
export AR_i686_linux_android="llvm-ar"

## 系统环境变量清除
# export ANDROID_PREFS_ROOT=/home/zdz/Android/Sdk

# npx tauri info
# pnpm tauri android init
pnpm tauri dev
# pnpm tauri android dev
# npx tauri android build
# pnpm tauri android dev -- --info
# pnpm tauri android build
# pnpm tauri android build --debug


# java --version
#
#
# adb install -r /home/zdz/Documents/Try/Rust/tauri/tauri_mobile/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk
