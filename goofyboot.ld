SECTIONS
{
  . = 0x1000;  /* Start address */

  .text : {
    *(.text)    /* Include all .text sections from input files */
  }

  .data : {
    *(.data)    /* Include all .data sections */
  }

  .bss : {
    *(.bss)
  }
}
