global _start

BUFFER_SIZE equ 32768
COLUMNS equ 12
LINES equ 1000
H_LINES equ 1000 / 2

section .data
    char: db 0
    i:    dd 0
    j:    dd 0
    buf:   times BUFFER_SIZE db 0  ; buffer to hold the input
    ones:  times COLUMNS dd 0      ; counters

; byte read to: char
; number of bytes read: eax.  0 is end of file.
read_byte:
    mov eax, 3    ; read
    mov ebx, 0    ; stdin
    mov ecx, char ; where to read to
    mov edx, 1    ; bytes to read
    int 0x80
    ret

; byte write to: char
write_byte:
    mov eax, 4    ; write
    mov ebx, 1    ; stdout
    mov ecx, char
    mov edx, 1
    int 0x80
    ret

; ebx: line
; ecx: column
; ret: eax
buf_get_char:
    mov eax, COLUMNS + 1
    mul ebx
    add eax, ecx
    mov al, byte[buf+eax]
    ret

exit:
    mov ebx, 0 ; value for exit()
    mov	eax, 1 ; exit
    int	0x80

process:
    mov dword[i], 0

outer_loop_start:
    mov dword[j], 0

inner_loop_start:
            mov ecx, [i]
            mov ebx, [j]
            call buf_get_char

            ; mov byte[char], al
            ; call write_byte

            cmp al, '0'
            je skip

            ; one
            mov eax, [i]
            mov edx, 4
            mul edx
            mov edx, ones
            add eax, edx
            mov ebx, eax
            mov eax, dword[ebx]
            add eax, 1
            mov dword[ebx], eax
skip:

            mov eax, [j]
            add eax, 1
            mov [j], eax

            cmp eax, LINES
            je inner_loop_end
            jmp inner_loop_start
inner_loop_end:

        mov eax, [i]
        add eax, 1
        mov [i], eax

        cmp eax, COLUMNS
        je outer_loop_end
        jmp outer_loop_start
outer_loop_end:

    mov dword[i], 0

comp_loop_start:
    mov eax, [i]
    mov edx, 4
    mul edx
    mov edx, ones
    add eax, edx
    mov ebx, eax
    mov eax, dword[ebx]

    ; mov byte[char], al
    ; call write_byte

    cmp eax, H_LINES
    jge is_one
    mov byte[char], '0'
    call write_byte
    jmp skip_is_one
is_one:
    mov byte[char], '1'
    call write_byte
skip_is_one:

    mov eax, [i]
    add eax, 1
    mov [i], eax

    cmp eax, COLUMNS
    je comp_loop_end

    jmp comp_loop_start
comp_loop_end:

    mov byte[char], 0x0a
    call write_byte

    ret

_start:
    mov dword[i], 0

read_loop_start:
	call read_byte
    cmp eax, 0         ; EOF?
    je read_loop_end

	cmp byte[char], 0x0a
	jne end_repl_n_nul
	mov byte[char], 0x00
end_repl_n_nul:

    ; save in buffer
    mov eax, [i]
    add eax, buf
    mov bl, byte[char]
    mov byte[eax], bl

    mov eax, [i]
    add eax, 1
    mov [i], eax

    jmp read_loop_start
read_loop_end:

    ; we are done reading input

    call process

    jmp exit

; In the end get the output and, say 010, invert it (101) and multiply them (e.g. in python with `0b101 * 0b010`)
