/* 
 *  This is the main entry point of 'Sugar Daemon'.
 *
 *  The main binary works in 32-bit protected mode with flat memory model and provides a connection between
 *  the backend service on mobile side. The binary takes commands from the mobile and provides output based
 *  on obtained query. The binary is not acknowledged in any file formats and can only separate them for further
 *  transmission to the mobile device. All data parsing and format recognition is done on the backend side.
 *
 *  Daemon will by all means cause no harm to the target device and only provide a way to create a word-by-word
 *  deep copy channel between a target device and the mobile backend.
 *
 *  The Daemon can be run only in RAM, so it takes no disk space of the target device nor it has any intentions
 *  to hijack any data it the disk space.
 * */

#include <stdint.h>

/* Main daemon entry point. */
void main(uint16_t boot_drive) {
    for(;;);
}
