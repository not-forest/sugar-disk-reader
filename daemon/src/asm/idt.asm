;       Here the IDT is defined.
;
;   Handler functions are linked to symbols, which will be
;   provided by main C application daemon during compile time.
;   This assembly module loads the new IDT to the target before
;   jumping to main application space.

; All handler function defined in C files are linked with those symbols
extern GENERAL_HANDLER

; Exceptions
extern DIVISION_ERROR_HANDLER
extern DEBUG_HANDLER
extern NMI_HANDLER
extern BREAKPOINT_HANDLER
extern OVERFLOW_HANDLER
extern BRE_HANDLER
extern INVALID_OPCODE_HANDLER
extern DEVICE_NOT_AVAILABLE_HANDLER
extern DOUBLE_FAULT_HANDLER
extern INVALID_TSS_HANDLER
extern SEGMENT_NOT_PRESENT_HANDLER
extern STACK_SEGMENT_FAULT_HANDLER
extern GENERAL_PROTECTION_FAULT_HANDLER
extern X87_FP_EXCEPTION_HANDLER
extern ALIGNMENT_CHECK_HANDLER
extern MACHINE_CHECK_HANDLER
extern SIMD_FP_EXCEPTION_HANDLER
extern VIRTUALIZATION_EXCEPTION_HANDLER
extern CONTROL_PROTECTION_HANDLER
extern HIE_HANDLER
extern VMMC_EXCEPTION_HANDLER
extern SECURITY_EXCEPTION_HANDLER

; Interrupts
extern SOFTWARE_TIMER_HANDLER
extern SOFTWARE_KEYBOARD_HANDLER

.text
bits 32
global IDT_TABLE  ; Global IDT table which will be loaded during the boot.

;   Defines all IDT entries with the required handler function.
;
;   If a certain vector number does not have a handler yet, will provide it with
;   a general handler, that easily just halts the execution of the process.
IDT_TABLE:
%assign i 0
%rep 33
    dd _isr_%+i ; Writing all 256 entries of IDT table.  
%assign i i + 1
%endrep

; Defines the vector number of IDT table with it's handler function.
%macro _define_isr 2
_isr_%+%1:
    call %2 ; Calling the handler function.
    iret
%endmacro

; Setting handler functions manually for each exception and interrupt
_define_isr 0,  DIVISION_ERROR_HANDLER
_define_isr 1,  DEBUG_HANDLER
_define_isr 2,  NMI_HANDLER
_define_isr 3,  BREAKPOINT_HANDLER
_define_isr 4,  OVERFLOW_HANDLER
_define_isr 5,  BRE_HANDLER
_define_isr 6,  INVALID_OPCODE_HANDLER
_define_isr 7,  DEVICE_NOT_AVAILABLE_HANDLER
_define_isr 8,  DOUBLE_FAULT_HANDLER
_define_isr 9,  GENERAL_HANDLER
_define_isr 10, INVALID_TSS_HANDLER
_define_isr 11, SEGMENT_NOT_PRESENT_HANDLER
_define_isr 12, STACK_SEGMENT_FAULT_HANDLER
_define_isr 13, GENERAL_PROTECTION_FAULT_HANDLER
_define_isr 14, GENERAL_HANDLER
_define_isr 15, GENERAL_HANDLER
_define_isr 16, X87_FP_EXCEPTION_HANDLER
_define_isr 17, ALIGNMENT_CHECK_HANDLER
_define_isr 18, MACHINE_CHECK_HANDLER
_define_isr 19, VIRTUALIZATION_EXCEPTION_HANDLER
_define_isr 20, CONTROL_PROTECTION_HANDLER
_define_isr 21, GENERAL_HANDLER
_define_isr 22, GENERAL_HANDLER
_define_isr 23, GENERAL_HANDLER
_define_isr 24, GENERAL_HANDLER
_define_isr 25, GENERAL_HANDLER
_define_isr 26, GENERAL_HANDLER
_define_isr 27, GENERAL_HANDLER
_define_isr 28, HIE_HANDLER
_define_isr 29, VMMC_EXCEPTION_HANDLER
_define_isr 30, SECURITY_EXCEPTION_HANDLER
_define_isr 31, GENERAL_HANDLER

_define_isr 32, SOFTWARE_TIMER_HANDLER
_define_isr 33, SOFTWARE_KEYBOARD_HANDLER
