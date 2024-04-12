/* 
 * A small module that defines constants related to PIC 8259 controller.
 * 
 * The amount of defines are enough to implement the most important part
 * of its use, which allows for future interrupts.
 * */

#ifndef __PIC__h
#define __PIC__h

#include<stdint.h>

#define PIC1		    0x20		// IO base address for master PIC 
#define PIC2		    0xA0		// IO base address for slave PIC
#define PIC1_COMMAND	PIC1
#define PIC1_DATA	(PIC1+1)
#define PIC2_COMMAND	PIC2
#define PIC2_DATA	(PIC2+1)
#define PIC_EOI		    0x20		// End-of-interrupt command code

#define ICW1_ICW4	    0x01		// Indicates that ICW4 will be present
#define ICW1_SINGLE	    0x02		// Single (cascade) mode
#define ICW1_INTERVAL4	0x04		// Call address interval 4 (8)
#define ICW1_LEVEL	    0x08		// Level triggered (edge) mode
#define ICW1_INIT	    0x10		// Initialization - required!
 
#define ICW4_8086	    0x01		// 8086/88 (MCS-80/85) mode
#define ICW4_AUTO	    0x02		// Auto (normal) EOI
#define ICW4_BUF_SLAVE	0x08		// Buffered mode/slave
#define ICW4_BUF_MASTER	0x0C		// Buffered mode/master
#define ICW4_SFNM	    0x10		// Special fully nested (not)

#define POST            0x80        // Port for sending data to a debug board (used for small delays.)

/* 
 * Small function that remaps PIC's int vectors, so that it will send interrupts
 * to the right entries within the IDT that won't collide with each other and CPU
 * exceptions. Both master and slave are mapped together in a chained way.
 * */
extern void remap_pic(uint8_t master_offset);
/* 
 * Tells one of the controllers, based on the provided port, that it's interrupt
 * is done and he shall start send more. The port must be either PIC1_COMMAND or
 * PIC2_COMMAND, ports.
 * */
__attribute__((no_caller_saved_registers)) 
extern void end_of_interrupt(uint16_t port);


#endif
