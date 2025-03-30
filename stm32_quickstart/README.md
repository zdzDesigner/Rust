


```sh
cargo readobj --bin stm32_quickstart -- --file-headers

cargo size --bin stm32_quickstart --release -- -A

cargo objdump --bin  stm32_quickstart --release -- --disassemble --no-show-raw-insn --print-imm-hex



```

> `qemu-system-arm` 模拟器
1. 查看支持的板子
```sh
➜ qemu-system-arm -machine help
Supported machines are:
akita                Sharp SL-C1000 (Akita) PDA (PXA270) (deprecated)
ast1030-evb          Aspeed AST1030 MiniBMC (Cortex-M4)
ast2500-evb          Aspeed AST2500 EVB (ARM1176)
ast2600-evb          Aspeed AST2600 EVB (Cortex-A7)
b-l475e-iot01a       B-L475E-IOT01A Discovery Kit (Cortex-M4)
bletchley-bmc        Facebook Bletchley BMC (Cortex-A7)
borzoi               Sharp SL-C3100 (Borzoi) PDA (PXA270) (deprecated)
bpim2u               Bananapi M2U (Cortex-A7)
canon-a1100          Canon PowerShot A1100 IS (ARM946)
cheetah              Palm Tungsten|E aka. Cheetah PDA (OMAP310) (deprecated)
collie               Sharp SL-5500 (Collie) PDA (SA-1110)
connex               Gumstix Connex (PXA255) (deprecated)
cubieboard           cubietech cubieboard (Cortex-A8)
emcraft-sf2          SmartFusion2 SOM kit from Emcraft (M2S010)
fby35-bmc            Facebook fby35 BMC (Cortex-A7)
fby35                Meta Platforms fby35
fp5280g2-bmc         Inspur FP5280G2 BMC (ARM1176)
fuji-bmc             Facebook Fuji BMC (Cortex-A7)
g220a-bmc            Bytedance G220A BMC (ARM1176)
highbank             Calxeda Highbank (ECX-1000)
imx25-pdk            ARM i.MX25 PDK board (ARM926)
integratorcp         ARM Integrator/CP (ARM926EJ-S)
...


```
2. 模拟器执行程序
```sh
➜ qemu-system-arm \
  -cpu cortex-m3 \
  -machine lm3s6965evb \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv7m-none-eabi/debug/examples/hello
```
- `qemu-system-arm`.这是QEMU模拟器.有几种不同的QEMU二进制文件;这个能对ARM机器进行完整的系统仿真
- `-cpu cortex-m3`.这告诉QEMU去模拟一个Cortex-M3 CPU.指定CPU型号可以让我们捕获一些编译错误:
  例如,运行为带有硬件FPU的Cortex-M4F编译的程序回事QEMU执行过程中出错.
- `-machine lm3s6965evb`.这告诉QEMU去模拟LM3S6965EVB,一个包含LM3S6965的评估开发板
- `-nographic`.这告诉QEMu不要去启动GUI.
- `-semihosting-config (..)`.这让QEMU启动semihosting. Semihosting允许仿真设备使用主机的stdout, stderr和stdin,并且在主机上创建文件
- `-kernel $file`.这告诉QEMU运行哪个二进制文件



3. 模拟器debug程序
```sh
➜ qemu-system-arm \
  -cpu cortex-m3 \
  -machine lm3s6965evb \
  -nographic \
  -semihosting-config enable=on,target=native \
  -gdb tcp::3333 \
  -S \
  -kernel target/thumbv7m-none-eabi/debug/examples/hello
```
```sh
➜ gdb-multiarch -q target/thumbv7m-none-eabi/debug/examples/hello

target remote :3333
```






