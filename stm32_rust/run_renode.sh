#!/bin/bash

# Renode 调试演示脚本

echo "=========================================="
echo "  Renode 调试演示"
echo "=========================================="
echo ""

# 编译
echo "[1] 编译项目..."
cargo build 2>&1 | grep -E "(error|warning)" || echo "  ✅ 编译成功"
echo ""

# 运行 Renode
echo "[2] 启动 Renode 仿真..."
echo ""
renode --disable-xwt --console \
  -e "include @renode/resc/lesson01_blinky.resc" 
 

echo ""
echo "[3] 分析输出..."
echo ""





