; Extern functions
extern main


; Text section for main boot
section .text
    global _start
; Entry point after BIOS.
bits 16
_start:
    cld                     ; Clear the direction flag  

    xor ax, ax              ; AX = 0
    mov es, ax              ; Clear the es segment
    mov ss, ax              ; Zeroing the stack segment

    mov sp, _MBRROM_ADDR_   ; Setup stack to grow below the bootloader.

    mov [_BOOT_DRIVE_], dl  ; BIOS sets the boot drive in 'dl' reg.

    mov al, 0x03            ; Clearing the screen while we are still in real mode.
    int 10h                 ; Make BIOS to clean the screen.

    ; The daemon must be loaded like any other kernel as it is a small 32bit OS.   
    call _load_daemon
    ; Switching to protected mode here, as almost all kernels are running in
    ; this mode. The TAIL also runs in protected mode.
    call _switch_protected

    jmp $

; Includes
%include "gdt.asm"
%include "disk.asm"

; Reads the location of the second stage and loads it as a small kernel.
_load_daemon:
    ; Preparing data for BIOS. The BIOS needs to know where to start reading,
    ; how much to read, and where to store the data in memory.
    mov bx, _APPROM_ADDR_  ; Tail address
    mov dh, _SECTORS_LEN_           ; Amount of sectors to read.
    mov dl, [_BOOT_DRIVE_] ; Making sure the dl has the disk info from BIOS.

    call _load_disk         ; With register set, calling the disk operation.
    ret

; Switches to protected mode by creating a flat memory model setup in GDT
_switch_protected:
    cli
    lgdt [_gdt_descriptor]
    mov eax, cr0                    ; Read the control reg
    or eax, 0x1                     ; Enable the protected mode
    mov cr0, eax                    ; Write the new control reg
    jmp _CODE_SEG_:_init_protected  ; Perform a far jump.

bits 32
_init_protected:
    mov ax, _DATA_SEG_     ; Update segment registers with data from gdt.
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov esp, _STACK_TOP_ ; Setting up a new stack

    mov eax, [_BOOT_DRIVE_]  ; Getting a selected drive info.
    push eax

    call main                ; Jump to the main application space.
    jmp $

_BOOT_DRIVE_ db 0

_SECTORS_LEN_ equ 3

; Extern variables
extern _MBRROM_ADDR_ 
extern _APPROM_ADDR_ 
extern _STACK_TOP_
