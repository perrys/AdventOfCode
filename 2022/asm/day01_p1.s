
        .section .rodata
        .equ FD_STDIN, 0
        .equ BUFFSIZE, 16000

.out_msg: .asciz "Score 1 is %ld\n"

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


        .type get_throw,  function
get_throw:      
        /* Parameters:
        %rsi - character
        %rdi - offset
        returns - 1=Rock, 2=Paper, 3=Scissors
        */
        sub %rdi, %rsi
        mov %rsi, %rax
        inc %rax
        ret

get_score:
        /* %rsi - their throw
           %rdi - your throw
        */
        cmp %rsi, %rdi
        je .draw
        cmp $1, %rdi
        jne .paper
        
        .draw:
        mov $6, %rax
        jmp .end

        .end:
        ret
        
        .section .bss
        .lcomm in_buffer, BUFFSIZE
