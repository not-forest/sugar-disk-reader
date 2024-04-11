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

extern void *IDT_TABLE[];       // IDT TABLE defined in idt.asm 

#include <stdint.h>
#include "arch/idt.h"
#include "vga.h"

#define L_WARN COLOR_YELLOW
#define L_OK   COLOR_GREEN
#define L_INFO COLOR_WHITE

#define OK "[OK]"
#define FD "[FAILED]"

// A global static buffer.
static volatile VGABuffer LOGGER = {.row = 0, .col = 0};

/* Main daemon entry point. */
void main(uint16_t boot_drive) {
    disable_cursor(); 

#if __RELEASE__ == 0
    prints("Initializing... ", L_INFO, &LOGGER); 
#endif

    /* Beginning of daemon post initialization */
    
    idt_init();

    /* End of initialization */
    
#if __RELEASE__ == 0
    println(OK, L_OK, &LOGGER);
#endif

    for(;;);
}

/* Initializing the IDT and putting required handler functions within */
void idt_init() {
    uint16_t cs;
    uint8_t vec;

    // Reading the cs segment.
    __asm__("mov %%cs, %w0\n":"=a"(cs)::"memory");

    // Providing exceptions,
    for (vec = 0; vec < 32; vec++) {
        idt_set_descriptor(vec, IDT_TABLE[vec], TRAP_GATE, cs);
    }

    for (; vec < 255; vec++) {
        idt_set_descriptor(vec, IDT_TABLE[vec], INT_GATE, cs);
    }

    // Creating a pointer for lidt instruction
    struct {
        uint16_t length;
        void*    base;
    } __attribute__((packed)) IDTR = { .length = sizeof(IDT) - 1, &IDT };
 
    __asm__ ( "lidt %0" : : "m"(IDTR) );
}
