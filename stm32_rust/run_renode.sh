#!/bin/bash

# Renode 调试演示脚本

set -e

EXAMPLE_NAME="day01_blinky"
TARGET_TRIPLE="thumbv7m-none-eabi"
BUILD_PROFILE="debug"
ELF_SRC="target/${TARGET_TRIPLE}/${BUILD_PROFILE}/examples/${EXAMPLE_NAME}"
ELF_OUT="dist/${EXAMPLE_NAME}.elf"

echo "=========================================="
echo "  Renode 调试演示"
echo "=========================================="
echo ""

# 编译
echo "[1] 编译 example 并导出固定 ELF..."
cargo build --example "${EXAMPLE_NAME}"
mkdir -p dist
cp "${ELF_SRC}" "${ELF_OUT}"
echo "  ✅ 已导出: ${ELF_OUT}"
echo ""

# 运行 Renode
echo "[2] 启动 Renode 仿真..."
echo ""
renode --disable-xwt --console \
  -e "include @renode/resc/lesson01_blinky.resc" 
 

echo ""
echo "[3] 分析输出..."
echo ""




