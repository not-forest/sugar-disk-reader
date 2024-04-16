/* 
 * Defines functions related to PIC controller.
 * */

#include<stdint.h>
#include"arch/pic.h"
#include"arch/cpu.h"

/* 
 * Small function that remaps PIC's int vectors, so that it will send interrupts
 * to the right entries within the IDT that won't collide with each other and CPU
 * exceptions. Both master and slave are mapped together in a chained way.
 * */
void remap_pic(uint8_t master_offset) {
    uint8_t mask1, mask2;

    mask1 = inb(PIC1_DATA);                             // save masks
	mask2 = inb(PIC2_DATA);
 
	outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);          // starts the initialization sequence (in cascade mode)
	outb(POST, 0);                                      // Delay.
	outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
	outb(POST, 0);
	outb(PIC1_DATA, master_offset);                     // ICW2: Master PIC vector offset
    outb(POST, 0); 
	outb(PIC2_DATA, master_offset + 8);                 // ICW2: Slave PIC vector offset
	outb(PIC1_DATA, 4);                                 // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
	outb(PIC2_DATA, 2);                                 // ICW3: tell Slave PIC its cascade identity (0000 0010)
 
	outb(PIC1_DATA, ICW4_8086);                         // ICW4: have the PICs use 8086 mode (and not 8080 mode)
	outb(PIC2_DATA, ICW4_8086);
 
	outb(PIC1_DATA, mask1);                             // restore saved masks.
	outb(PIC2_DATA, mask2);
}

/* 
 * Tells one of the controllers, based on the provided port, that it's interrupt
 * is done and he shall start send more. The port must be either PIC1_COMMAND or
 * PIC2_COMMAND, ports.
 * */
__attribute__((no_caller_saved_registers)) 
void end_of_interrupt(uint16_t port) {
    outb(port, PIC_EOI); // OCW2: telling the chosen PIC.
}
