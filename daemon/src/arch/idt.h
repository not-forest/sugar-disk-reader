/* 
 *  This header provides definitions of IDT related management structures
 *  and functions.
 * */

#ifndef _IDT_h
#define _IDT_h

#define TASK_GATE   0x85
#define INT_GATE    0x8E
#define TRAP_GATE   0x8f

#include<stdint.h>

typedef union {
    void *handler;
    uint32_t bits[2];
} ISR_F;

// A special type for gate descriptor, which is an entry in the IDT.
typedef struct {
    uint16_t offset_0_15;
    uint16_t selector;  // Code segment selector in GDT.
    uint8_t reserved;   // Must always be set to zero
    uint8_t attr;       // Contain gate type, dpl and p fields. 
    uint16_t offset_16_31;
} __attribute__((packed)) GateDescriptor;

// static IDT table with all 256 entries.
__attribute__((aligned(0x10)))
static GateDescriptor IDT[256];

/* Sets the certain vector of IDT to some value */
void idt_set_descriptor(uint8_t vec, ISR_F isr, uint8_t flags, uint16_t selector) {
    GateDescriptor *desc = &IDT[vec];

    desc->offset_0_15 = isr.bits[0];
    desc->selector = selector;
    desc->reserved = 0;
    desc->attr = flags;
    desc->offset_16_31 = isr.bits[1];
}

/* Main function that initialize the whole IDT table */
void idt_init(void);

#endif
