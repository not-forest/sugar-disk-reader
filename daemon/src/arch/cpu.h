/*
 *  Header file that defines CPU related features and structures.
 * */

#ifndef __CPU__h
#define __CPU__h

#include<stdint.h>

/* 
 *  A special struct that is being pushed to the stack by CPU when some exception or interrupt occurs.
 *
 *  Daemon is running in 32-bit protected mode to support or types of devices and to not cause overhead
 *  of long mode. Because of that only three registers will be pushed onto the stack during an interrupt.
 * */
struct Iframe {
    uint16_t eip, cs, eflags;
}; 

// Writes a byte to the chosen port.
__attribute__((no_caller_saved_registers)) 
static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile ( "outb %b0, %w1" : : "a"(val), "Nd"(port) : "memory");
}

// Reads the data from the provided port.
__attribute__((no_caller_saved_registers)) 
static inline uint8_t inb(uint16_t port) {
    uint8_t val;
    __asm__ volatile ( "inb %w1, %b0" : "=a"(val) : "Nd"(port) : "memory");
    return val;
}

//////// CPU DEFINED INTERRUPTS AND EXCEPTIONS /////////

#define general_handler(name) \
    void name(struct Iframe *frame) __attribute__((weak));

/* * * * * * * * * * * * * * * * * * * *
 *  EXCEPTION HANDLERS (vecn: 0 - 31)
 *
 *  Defined by CPU as exceptions and shall be provided with handler function.
 * * * * * * * * * * * * * * * * * * * */

/*
 *  GENERAL HANDLER
 *
 *  Will be aplied to all undefined handler functions to prevent double fault. 
 * */
general_handler(GENERAL_HANDLER)
general_handler(DIVISION_ERROR_HANDLER);
general_handler(DEBUG_HANDLER);
general_handler(NMI_HANDLER);
general_handler(BREAKPOINT_HANDLER);
general_handler(OVERFLOW_HANDLER);
general_handler(BRE_HANDLER);
general_handler(INVALID_OPCODE_HANDLER);
general_handler(DEVICE_NOT_AVAILABLE_HANDLER);
general_handler(DOUBLE_FAULT_HANDLER);
general_handler(INVALID_TSS_HANDLER);
general_handler(SEGMENT_NOT_PRESENT_HANDLER);
general_handler(STACK_SEGMENT_FAULT_HANDLER);
general_handler(GENERAL_PROTECTION_FAULT_HANDLER);
general_handler(X87_FP_EXCEPTION_HANDLER);
general_handler(ALIGNMENT_CHECK_HANDLER);
general_handler(MACHINE_CHECK_HANDLER);
general_handler(SIMD_FP_EXCEPTION_HANDLER);
general_handler(VIRTUALIZATION_EXCEPTION_HANDLER);
general_handler(CONTROL_PROTECTION_HANDLER);
general_handler(HIE_HANDLER);
general_handler(VMMC_EXCEPTION_HANDLER);
general_handler(SECURITY_EXCEPTION_HANDLER);


/* * * * * * * * * * * * * * * * * * * *
 *  SOFTWARE INTERRUPTS (vecn: 32 - 255)
 *
 *  Defined by this daemon and can only be cause via certain pheripherals like PIC or
 *  via software part itself.
 * * * * * * * * * * * * * * * * * * * */
general_handler(SOFTWARE_TIMER_HANDLER);
general_handler(SOFTWARE_KEYBOARD_HANDLER);

#endif
