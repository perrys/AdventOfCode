        .section .rodata
        .equ FD_STDIN, 0
        .equ SYS_READ, 0
        .equ BUFFSIZE, 16000
.input_err: .asciz "Unable to read input\n"

        .section .text
        .type read_file, function
        .global read_file

/**
  Read the entire contents of a file into a buffer.
  Inputs:
    rdi - file descriptor
    rsi - pointer to buffer
*/
read_file:
        .equ ST_BYTESREAD, -1*8
        .equ STSIZE, 1*8 /* keep this 16-byte aligned when calling C functions */
        push %rbp
        mov %rsp, %rbp
        sub $STSIZE, %rsp

        mov $SYS_READ, %rax
        mov $BUFFSIZE, %rdx
        syscall
        mov %rax, ST_BYTESREAD(%rbp)

        cmp $0, %rax
        jg .read_success
        mov $.input_err, %rdi
        mov $0, %rax
        call printf
        mov $1, %rax
        call exit

.read_success:
        mov ST_BYTESREAD(%rbp), %rax
        add $STSIZE, %rsp
        pop %rbp
        ret

        .type parse_file, function
        .global parse_file
/**
  Parse a file of numbers delimited by newlines
  with groups of numbers separated by blank lines.
  Call back the supplied function with the sum of each
  group of numbers
Inputs: 
        rdi - input pointer
        rsi - end-of-input pointer
        rdx - callback function
*/
parse_file:
        
        push %rbp
        mov %rsp, %rbp
        push %rbx
        .equ STSIZE, 16
        .equ ST_CALLBACK, -16
        sub $STSIZE, %rsp
        mov %rdx, ST_CALLBACK(%rbp)
/* Registers:
   rax - atoi accumulator
   rbx - byte read (in rl)
   rcx - batch accumulator
*/
.newbatch:                      
        xor %rcx, %rcx
.newline:
        xor %rax, %rax
.byte_parse_loop:
        cmp %rdi, %rsi
        je .eof
        xor %rbx, %rbx
        movb (%rdi), %bl
        add $1, %rdi
        cmpb $'\n', %bl
        je .eol
        mov $10, %r8
        mull %r8d
        sub $'0', %bl
        add %rbx, %rax
        jmp .byte_parse_loop
.eol:
        cmp $0, %rax
        je .savebatch
        add %rax, %rcx
        jmp .newline
.savebatch:       
        push %rdi
        push %rsi
        mov %rcx, %rdi
        mov ST_CALLBACK(%rbp), %rdx
        call *%rdx
        pop %rsi
        pop %rdi
        jmp .newbatch
        
.eof:
        
        add $STSIZE, %rsp
        pop %rbx
        pop %rbp
        ret

        .type record_time, function
        .global record_time

record_time:
        .equ CLOCK_MONOTONIC, 1
        mov %rdi, %rsi
        mov $CLOCK_MONOTONIC, %rdi
        call clock_gettime
        ret

        .section .data
.stats: .quad 0,0,0,0


        .equ SUM, 0
        .equ SUMSQ, 8
        .equ MIN, 16
        .equ COUNT, 24

        .section .text
        .type mark_time, function
        .global mark_time

mark_time:
/* Registers        
   rdi - pointer to T0 struct
   rsi - pointer to T1 struct
        */
        call calc_elapsed
        cvtsi2sd %eax, %xmm0
        movsd .stats+SUM, %xmm1
        addsd %xmm0, %xmm1
        movsd %xmm1, .stats+SUM
        mulsd %xmm0, %xmm0
        movsd .stats+SUMSQ, %xmm1
        addsd %xmm0, %xmm1
        movsd %xmm1, .stats+SUMSQ
        cmpq .stats+MIN, %rax
        jle .notmin
        mov %rax, .stats+MIN
        .notmin:
        mov .stats+COUNT, %rax
        inc %rax
        mov %rax, .stats+COUNT
        ret
        
calc_elapsed:
/* Registers:
   rdi - pointer to T0 struct
   rsi - pointer to T1 struct
        */
        mov (%rsi), %rax
        sub (%rdi), %rax
        .equ NS_PER_SEC, 1000000000
        mov $NS_PER_SEC, %rcx
        mul %rcx
        add 8(%rsi), %rax
        sub 8(%rdi), %rax
        ret
        
        .type print_elapsed, function
        .global print_elapsed
print_elapsed:
        
        mov .stats+MIN, %rsi
        mov $outmsg, %rdi
        mov $0, %eax
        call printf
        ret

outmsg: .asciz "Min elapsed time: %dns\n"
