MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */  
  FLASH : ORIGIN = 0x00004000, LENGTH = 240K
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
}

SECTIONS
{
  .text :
  {
    *(.text*)
    *(.rodata*)
  } > FLASH

  .data :
  {
    *(.data*)
  } > RAM AT > FLASH

  .bss :
  {
    *(.bss*)
    *(COMMON)
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*)
  }
}
