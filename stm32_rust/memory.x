MEMORY
{
  /* STM32F103C8T6: 64K Flash, 20K RAM */
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K
  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 20K
}

/* 栈顶地址：RAM 的末尾 */
_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  /* 1. 向量表段 */
  .vector_table :
  {
    . = ALIGN(4);
    /* 第一项：初始栈指针 (MSP) */
    LONG(_estack);
    /* 第二项：复位向量地址 */
    KEEP(*(.vector_table.reset_vector));
    . = ALIGN(4);
  } > FLASH

  /* 2. 代码段 (.text) */
  .text :
  {
    *(.text .text.*);
    *(.glue_7);         /* ARM 到 Thumb 的胶水代码 */
    *(.glue_7t);        /* Thumb 到 ARM 的胶水代码 */
    . = ALIGN(4);
  } > FLASH

  /* 3. 只读数据段 (.rodata) */
  .rodata :
  {
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > FLASH

  /* 4. 数据段 (.data) - 加载在 Flash，运行在 RAM */
  /* _sidata 是 .data 在 Flash 中的加载地址 */
  _sidata = LOADADDR(.data);

  .data : AT (_sidata)
  {
    . = ALIGN(4);
    _sdata = .;         /* .data 段在 RAM 中的起始地址 */
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;         /* .data 段在 RAM 中的结束地址 */
  } > RAM

  /* 5. BSS 段 (.bss) - 未初始化数据，运行时清零 */
  .bss :
  {
    . = ALIGN(4);
    _sbss = .;          /* .bss 段起始地址 */
    *(.bss .bss.*);
    *(COMMON);          /* 未指定段的 COMMON 符号 */
    . = ALIGN(4);
    _ebss = .;          /* .bss 段结束地址 */
  } > RAM
}
