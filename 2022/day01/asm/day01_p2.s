
        .section .rodata
        .equ FD_STDIN, 0
        .equ BUFFSIZE, 16000
        .equ N_COUNT, 1000000000

        .section .data
.t0:    .quad 0,0           
.t1:    .quad 0,0           

.out_msg: .asciz "Top 3 sum to: %ld\n"

        .section .data
max_vals: .quad 0, 0, 0
        
        .section .text
        .global _start
        .type _start, function
_start:
        .equ STSIZE, 2*8 /* keep this 16-byte aligned when calling C functions */
        mov %rsp, %rbp
        sub $STSIZE, %rsp
        mov $FD_STDIN, %rdi
        mov $in_buffer, %rsi
        call read_file

        push %rax
        push %rdi
        mov $.t0, %rdi
        call record_time
        pop %rdi
        pop %rax

        add $in_buffer, %rax
        mov %rax, %rsi
        mov $in_buffer, %rdi
        mov $top3, %rdx
        mov $N_COUNT, %r15
.l0:    cmp $0, %r15
        je .l0d
        call parse_file
        dec %r15
        jmp .l0
.l0d:        
        mov $.t1, %edi
        call record_time
        mov $.t0, %edi
        mov $.t1, %esi
        call print_elapsed

        mov $max_vals, %rbx
        mov $0, %rsi
        mov $0, %rdi
.loop:        
        add (%rbx, %rdi, 8), %rsi
        inc %rdi
        cmp $3, %rdi
        jne .loop
        mov $.out_msg, %rdi
        mov $0, %rax
        call printf
        add $STSIZE, %rsp
        mov $0, %edi
        call exit


        .type top3, function
        .global top3

top3:
        .equ IDX_1, 8
        .equ IDX_2, 16
        push %rbp
        mov %rsp, %rbp
/* rdi has value
   rdx will point to array
*/
        mov $max_vals, %rdx
        cmp (%rdx), %rdi 
        jle .done
        mov %rdi, (%rdx)
        /* sort array */
        cmp IDX_1(%rdx), %rdi 
        jle .done
        mov IDX_1(%rdx), %rax
        mov %rdi, IDX_1(%rdx)
        mov %rax, (%rdx)
        cmp IDX_2(%rdx), %rdi
        jle .done
        mov IDX_2(%rdx), %rax
        mov %rdi, IDX_2(%rdx)
        mov %rax, IDX_1(%rdx)
.done:       
        leave
        ret

        
        .section .bss
        .lcomm in_buffer, BUFFSIZE
