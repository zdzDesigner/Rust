# 关闭分页，避免脚本输出中途停下来等用户按键。
set pagination off
# 关闭确认提示，方便批处理。
set confirm off

# 读取当前最关键的 SCB Fault 状态寄存器。
define fault_regs
  echo \n=== Fault Registers ===\n
  printf "HFSR  @ 0xE000ED2C = "
  x/1xw 0xE000ED2C
  printf "CFSR  @ 0xE000ED28 = "
  x/1xw 0xE000ED28
  printf "MMFAR @ 0xE000ED34 = "
  x/1xw 0xE000ED34
  printf "BFAR  @ 0xE000ED38 = "
  x/1xw 0xE000ED38
  echo \n
end

# 连接 Renode 暴露的 GDB server。
target remote localhost:3333
# 装载 ELF，并让 GDB 获取符号。
load

# 第一阶段：确认从预期入口启动。
break watchdog_fault_reboot.main
continue

# 第二阶段：观察 fault 落点。
# 这里既看统一裸入口，也看 app 层 onCrashDump 回调是否被调用。
tbreak hal.crash_dump.VectorTable.UsageFault
tbreak hal.crash_dump.VectorTable.HardFault
tbreak hal.crash_dump.crashDumpDispatch
tbreak watchdog_fault_reboot.onCrashDump
tbreak chip.devices.STM32F103.VectorTable._unhandled
echo \n=== Running to Fault ===\n
continue

# 程序停住后，按“当前位置 -> Fault 寄存器 -> 调用栈 -> CPU 通用寄存器”的顺序输出现场。
echo \n=== Stop Location ===\n
frame
fault_regs
bt
info registers

echo \n注意：当前 Renode 平台没有 IWDG 模型，所以这里只能验证 fault/live dump；IWDG 复位后二次启动流程需在真机上验证。\n
