[BITS 16]
[ORG 0x7C00]

mov bx, 0x9000
mov es, bx
mov bx, 0x0000

mov ah, 0x02       ; Read disk sector
mov al, 0x01       ; Number of sectors to read
mov ch, 0x00       ; Cylinder number
mov cl, 0x02       ; Sector number
mov dh, 0x00       ; Head number
mov dl, 0x00       ; Boot device (assume first floppy drive)

int 0x13           ; BIOS disk interrupt

cmp ah, 0x00       ; Check if read succeeded
jne error

jmp 0x9000:0x0000  ; Jump to kernel entry point

error:
hlt                ; Halt the CPU

times 510-($-$$) db 0
dw 0xAA55
