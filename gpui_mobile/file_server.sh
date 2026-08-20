#!/bin/bash
set -e

# PC 端文件互传服务器启动脚本。
# 用法: ./file_server.sh [共享目录]    # 默认 ./shared

BIN="crates/file-server/target/release/file-server"
SHARE_DIR="${1:-shared}"

if [ ! -x "$BIN" ]; then
    echo "🔨 首次运行，构建 file-server..."
    (cd crates/file-server && cargo build --release)
fi

if [ ! -x "$BIN" ]; then
    echo "❌ 构建失败，未找到: $BIN"
    exit 1
fi

mkdir -p "$SHARE_DIR"

echo "🚀 启动文件互传服务器（Ctrl+C 停止）"
exec "$BIN" "$SHARE_DIR"
