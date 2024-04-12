;            Here all disk operations are defined
;
;   This part must be executed in real mode, because it uses the functionality
;   of BIOS to read the disk in an easy way. In order to read data from disk, 
;   we need to specify where to start reading, how much to read, and where to 
;   store the data in memory.

section .text
bits 16
; This function provides an interface to load data from the disk via BIOS.
_load_disk:
    pusha       
    push dx

    mov ah, 0x02           ; Read mode
    mov al, dh             ; Read DH sectors

    mov ch, 0x00           ; Cylinder 0
    mov cl, 0x02           ; Start from the second one
    mov dh, 0x00           ; Head 0

    int 13h        ; BIOS interrupt for disk op.
    jc _disk_error ; Checking the carry bit for error

    pop dx
    cmp al, dh     ; If the number of sectors that BIOS read is not right.
    jne _sectors_error
    popa
    ret

; TODO
_disk_error:
    jmp $

; TODO
_sectors_error:
    jmp $
