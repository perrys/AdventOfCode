
        .section .rodata
        .equ FD_STDIN, 0
        .equ BUFFSIZE, 16000

.out_msg: .asciz "Max is %ld\n"

        .section .data
max_val: .quad 0
        
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

        add $in_buffer, %rax
        mov %rax, %rsi
        mov $in_buffer, %rdi
        mov $save_max, %rdx
        call parse_file
        
        mov $.out_msg, %rdi
        mov max_val, %rsi
        mov $0, %rax
        call printf
        add $STSIZE, %rsp
        mov $0, %edi
        call exit


        .type save_max, function
save_max:
        cmp max_val, %rdi
        jle .ignore
        movq %rdi, max_val
.ignore:
        ret
        
        .section .bss
        .lcomm in_buffer, BUFFSIZE
