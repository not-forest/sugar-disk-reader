/* 
 *  This module defines all interrupt handler function needed for proper
 *  daemon execution. Those handler functions are primarly implemented to
 *  communicate with the backend on mobile side.
 *
 *  All defined functions from this module are automatically linked to IDT
 *  defined in assembly initialization part.
 * */

#include "arch/cpu.h"
#include "arch/pic.h"
#include "vga.h"

extern VGABuffer LOGGER;

/* Halts the whole application completely. */
void GENERAL_HANDLER(struct Iframe *frame) {
#if !__RELEASE__
    println("Error general handler invoked!", COLOR_RED, &LOGGER);
#endif
    // Debugging logic... 

    __asm__ volatile ("cli; hlt");
}

#if !__RELEASE__
/* Debug handler. (Debug build only)
 *
 * 
 * */
void BREAKPOINT_HANDLER(struct Iframe *frame) {
    
}
#endif

/* Handles most of IO events by checking the buffered data obtained from 
 * mobile's backend. */
void SOFTWARE_TIMER_HANDLER(struct Iframe *frame) { 
#if !__RELEASE__
    printc('A', COLOR_CYAN, &LOGGER);
#endif

    end_of_interrupt(PIC1_COMMAND);
}

void SOFTWARE_KEYBOARD_HANDLER(struct Iframe *frame) {
    // IO logic...

    end_of_interrupt(PIC1_COMMAND);
}

// Aliasing out undefined handlers to GENERAL_HANDLER.
#define __aliased(name) \
    void name(struct Iframe *frame) __attribute__((interrupt, alias("GENERAL_HANDLER")));

/* All defined handles are commented out */

__aliased(DIVISION_ERROR_HANDLER);
__aliased(DEBUG_HANDLER);
__aliased(NMI_HANDLER);

#if __RELEASE__
__aliased(BREAKPOINT_HANDLER);
#endif

__aliased(OVERFLOW_HANDLER);
__aliased(BRE_HANDLER);
__aliased(INVALID_OPCODE_HANDLER);
__aliased(DEVICE_NOT_AVAILABLE_HANDLER);
__aliased(DOUBLE_FAULT_HANDLER);
__aliased(INVALID_TSS_HANDLER);
__aliased(SEGMENT_NOT_PRESENT_HANDLER);
__aliased(STACK_SEGMENT_FAULT_HANDLER);
__aliased(GENERAL_PROTECTION_FAULT_HANDLER);
__aliased(X87_FP_EXCEPTION_HANDLER);
__aliased(ALIGNMENT_CHECK_HANDLER);
__aliased(MACHINE_CHECK_HANDLER);
__aliased(SIMD_FP_EXCEPTION_HANDLER);
__aliased(VIRTUALIZATION_EXCEPTION_HANDLER);
__aliased(CONTROL_PROTECTION_HANDLER);
__aliased(HIE_HANDLER);
__aliased(VMMC_EXCEPTION_HANDLER);
__aliased(SECURITY_EXCEPTION_HANDLER);


//__aliased(SOFTWARE_TIMER_HANDLER);
//__aliased(SOFTWARE_KEYBOARD_HANDLER);
