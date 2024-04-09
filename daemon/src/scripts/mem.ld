/*  
 *  Memory area definitions for Daemon.
 *
 *  Memory areas are separated into two sections, the MBR bootloading part and the main
 *  application area. The first part only bootloads the second part, while the second
 *  part establishes connection with mobile's backend.
 *  */

MEMORY
{
    mbrrom (rx)  : ORIGIN = 0x7C00, LENGTH = 512   /* MBR */
    approm (rx)  : ORIGIN = 0x7E00, LENGTH = 5K    /* Main application's code */
    appram (rwx) : ORIGIN = 0x9200, LENGTH = 1K    /* Main application's stack */
}

_STACK_SIZE_ = 1K;

_MBRROM_ADDR_ = ABSOLUTE(ORIGIN(mbrrom));
_MBRROM_SIZE_ = ABSOLUTE(LENGTH(mbrrom));
_APPROM_ADDR_ = ABSOLUTE(ORIGIN(approm));
_APPROM_SIZE_ = ABSOLUTE(LENGTH(approm));
_APPRAM_ADDR_ = ABSOLUTE(ORIGIN(appram));
