/*  
 *  Main linker script.
 *
 *  This script defines kernel sections of daemon binary. Daemon is a relatively small binary
 *  application, so it does not require too much kernel sections and modules, yet it has to
 *  enter to 32-bit protected mode and bootload the main C code.
 * */

/* Daemon entry point */
ENTRY(_start)

/* Memory areas definitions */
INCLUDE mem.ld

/* Separated assembly imports. */
INPUT(mbr.o gdt.o disk.o) 

SECTIONS
{
    . = ORIGIN(mbrrom);

    .text : AT(ORIGIN(mbrrom))
    {
        mbr.o*(.text)
        gdt.o*(.text)
        disk.o(.text)
    } > mbrrom

    .signature : AT(0x7DFE)
    {
        /* The boot signature needed by BIOS. */
        SHORT(0xaa55)
    }

    .text : AT(ORIGIN(approm))
    {
        KEEP(*(.vectors*))
        *(.text*)
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
        _STACK_TOP_ = .;
    } > appram

    /DISCARD/ : 
    {
        *(COMMON)
    }

    _DAEMON_END_ = .;
}

