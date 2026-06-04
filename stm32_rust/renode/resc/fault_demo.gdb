# 关闭分页，避免批量打印寄存器或回溯时停下来等用户按键。
set pagination off
# 关闭确认提示，方便脚本模式自动执行。
set confirm off

# 读取当前最关键的 SCB Fault 状态寄存器。
# 这些地址对应 Cortex-M3 System Control Block：
# - HFSR  : HardFault Status Register
# - CFSR  : Configurable Fault Status Register
# - MMFAR : MemManage Fault Address Register
# - BFAR  : BusFault Address Register
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
# 把 ELF 装载到 Renode 当前机器里，同时让 GDB 拿到符号信息。
load

# 第一阶段：先确认程序从预期入口启动。
# 这里用普通 break，是因为 main 是稳定入口，命中后便于继续下第二阶段断点。
break fault_demo.main
continue

# 第二阶段：只关心“这一次 Fault 最终落到哪里”。
# 现在 Fault 由 hal.crash_dump 统一接管，所以优先观察裸入口和统一分发函数。
tbreak hal.crash_dump.VectorTable.UsageFault
tbreak hal.crash_dump.VectorTable.HardFault
tbreak hal.crash_dump.crashDumpDispatch
tbreak chip.devices.STM32F103.VectorTable._unhandled
echo \n=== Running to Fault ===\n
continue

# 程序停住后，按“当前位置 -> Fault 寄存器 -> 调用栈 -> CPU 通用寄存器”的顺序输出现场。
echo \n=== Stop Location ===\n
frame
fault_regs
bt
info registers
# 如果命中默认 _unhandled，通常说明统一捕获入口没有成功接管这次 Fault。
echo \n如果当前停在 _unhandled，说明这次 Fault 没有进入 hal.crash_dump 统一异常入口，而是落到了默认异常入口。\n
