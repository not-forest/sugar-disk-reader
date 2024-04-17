/*  
 *  Main linker script.
 *
 *  This script defines kernel sections of daemon binary. Daemon is a relatively small binary
 *  application, so it does not require too much kernel sections and modules, yet it has to
 *  enter to 32-bit protected mode and bootloads the main C code.
 * */

/* Daemon entry point */
ENTRY(_start)

/* Memory areas definitions */
INCLUDE mem.ld

/* Separated assembly imports. */
INPUT(mbr.o gdt.o disk.o idt.o insc.o) 

SECTIONS
{
    . = ORIGIN(mbrrom);

    .text : AT(ORIGIN(mbrrom))
    {
        /* Assembly */
        mbr.o*(.text)
        gdt.o*(.text)
        disk.o*(.text)

        insc.o*(.text)
    } > mbrrom

    .signature : AT(0x7DFE)
    {
        /* The boot signature needed by BIOS. */
        SHORT(0xaa55)
    }

    .text : AT(ORIGIN(approm))
    {
        /* Main application space. Starting in the second disk sector. */
        KEEP(*(.vectors*))
        *(.text*)
        /* Amount of sectors to copy from the disk. */
        _SECTORS_AMOUNT_ = (. + 511) / 512 - _APPROM_ADDR_;
        *(.rodata*)
        *(.got*)
    } > approm

    .bss (NOLOAD) :
    {
        *(.bss*)
        *(COMMON)
    } > approm

    .data :
    {
        *(.data*)
    } > approm

    .stack (NOLOAD) : AT(ORIGIN(appram))
    {
        . =  ALIGN(8);
        . += _STACK_SIZE_;
        . =  ALIGN(8);
        _STACK_TOP_ = .;                        /* Stack top address. */
    } > appram

    /DISCARD/ : 
    {
        *(COMMON)                               /* Removing all repeated symbols. */
    }

    _DAEMON_END_ = .;                           /* Symbol which defines the end of daemon. */
}
