;       General purpose assembly function for daemon
;
;   All defined functions within this module serve only as
;   convenient low level assembly bridge to not overruse C
;   inline assembly statement. For the sake of smaller binary
;   footprint, those functions are stacked in MBR sector with
;   other assembly.

section .text
    global update_cursor
    global enable_cursor
    global disable_cursor
bits 32

; Updates the VGA cursor location based on a row and column value.
update_cursor:
    ; Inputs:
    ;   rdi:  row (uint8_t)
    ;   rsi:  col (uint8_t)
    ; Outputs: None
    ; Clobbers: dx, al

    ; Calculate the position
    movzx ebx, byte [edi]   ; Load row into ebx
    movzx ecx, byte [esi]   ; Load col into ecx
    mov ax, 0x07            ; Load attribute
    mul cx                  ; Multiply row by BUFFER_WIDTH
    add bx, ax              ; Add col
    mov dx, 0x03d4          ; DX = VGA port
    mov al, 0x0f            ; Low cursor byte command
    out dx, al              ; Send command
    inc dx                  ; DX points to high cursor byte port
    mov al, bl              ; Move low cursor byte
    out dx, al              ; Send byte to port
    dec dx                  ; DX points back to low cursor byte port
    mov al, 0x0e            ; High cursor byte command
    out dx, al              ; Send command
    inc dx                  ; DX points to high cursor byte port
    mov al, bh              ; Move high cursor byte
    out dx, al              ; Send byte to port
    ret

disable_cursor:
    ; Outputs: None
    ; Clobbers: dx, al
    mov dx, 0x3d4       ; DX = VGA port
    mov al, 0xa         ; Cursor start register
    out dx, al          ; Send command

    inc dx              ; DX points to cursor end register
    mov al, 0x20        ; Cursor end value (disable)
    out dx, al          ; Send byte to port

    ret

enable_cursor:
    ; Inputs:
    ;   cursor_start (uint8_t)
    ;   cursor_end (uint8_t)
    ; Outputs: None
    ; Clobbers: dx, al

    ; Setting up the cursor's start
    mov dx, 0x3d4       ; DX = VGA port
    mov al, 0xa         ; Cursor start register
    out dx, al          ; Send command

    inc dx              ; DX points to cursor start data port
    in al, dx           ; Read current cursor start value
    and al, 0xc0        ; Clear the cursor start bits
    or al, dl           ; Set the cursor start bits with cursor_start value
    out dx, al          ; Write updated cursor start value

    ; Setting up the cursor's end
    mov dx, 0x3d4       ; DX = VGA port
    mov al, 0xb         ; Cursor end register
    out dx, al          ; Send command

    inc dx              ; DX points to cursor end data port
    in al, dx           ; Read current cursor end value
    and al, 0xe0        ; Clear the cursor end bits
    or al, al           ; Set the cursor end bits with cursor_end value
    out dx, al          ; Write updated cursor end value

    ret
