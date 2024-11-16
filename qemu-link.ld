/* Memory layout of the STM32F405RGT6 microcontroller */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 192K
}

ENTRY(Reset);
EXTERN(RESET_VECTOR);
EXTERN(_EXCEPTIONS);

SECTIONS
{
  PROVIDE(_sstack = ORIGIN(RAM) + LENGTH(RAM));

  .vector_table ORIGIN(FLASH) :
  {
    _vector_table = .;
    
    /* initial Stack Pointer */
    LONG(_sstack);

    /* Reset vector */
    KEEP(*(.vector_table.reset_vector));

    /* Exceptions vectors */
    KEEP(*(.vector_table.exceptions))
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  .rodata :
  {
    . = ALIGN(4);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > FLASH

  .bss :
  {
    . = ALIGN(4);
    _sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    . = ALIGN(4);
    _sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > RAM

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.* .ARM.attributes);
  }
}