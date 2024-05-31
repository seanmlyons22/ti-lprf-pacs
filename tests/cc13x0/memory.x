MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* Application is stored in and executes from internal flash */
  FLASH (RX) : ORIGIN = 0x0, LENGTH = 0x1FFA8
  /* Customer Configuration Area (CCFG) */
  FLASH_CCFG (RX) : ORIGIN = 0x1FFA8, LENGTH = 88
  RAM (RWX) : ORIGIN = 0x20000000, LENGTH = 0x5000
}

SECTIONS
{
    .ccfg :
    {
        KEEP(*(.ccfg));
    } > FLASH_CCFG
}
