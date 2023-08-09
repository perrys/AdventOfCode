        .section .rodata
.input_err: .asciz "ERROR: Unable to read input\n"

        .section .data
.t0:    .quad 0,0           
.t1:    .quad 0,0           
.stats: .quad 0,0,0,0
        .equ SUM, 0
        .equ SUMSQ, 8
        .equ COUNT, 16
        .equ MIN, 24

        .section .text
        
/**
  Read the entire contents of a file into a buffer. Abort if read fails.
    rdi - file descriptor
    rsi - pointer to buffer
    rdx - size of buffer
*/
        .type read_file, function
        .global read_file
read_file:
        .equ ST_BYTESREAD, -1*8
        .equ STSIZE, 1*8 /* keep this 16-byte aligned when calling C functions */
        push %rbp
        mov %rsp, %rbp
        sub $STSIZE, %rsp

        .equ SYS_READ, 0
        mov $SYS_READ, %rax
        syscall
        mov %rax, ST_BYTESREAD(%rbp)

        cmp $0, %rax
        jg .read_success
        .equ FD_STDERR, 1
        mov $FD_STDERR, %rdi
        mov $.input_err, %rsi
        mov $0, %rax
        call fprintf
        mov $1, %rax
        call exit

.read_success:
        mov ST_BYTESREAD(%rbp), %rax
        add $STSIZE, %rsp
        pop %rbp
        ret

        
/**
  Call a function in a loop to time it repteatedly, and print out its performance
   rdi - callback function
   rsi - number of times to loop
*/
        .type perf_timer, function
        .global perf_timer
perf_timer:     
        push %rbp
        mov %rsp, %rbp
        push %rdi
        push %rsi
        push %rsi /* for stack alignment */
        .equ ST_FUNCTION, -1*8
        .equ ST_COUNTER, -2*8
        .equ CLOCK_MONOTONIC, 1
        mov $0, %rax
        mov %rax, .stats+SUM
        mov %rax, .stats+SUMSQ
        mov %rax, .stats+COUNT
        mov $0xffffffffffffffff, %rax
        mov %rax, .stats+MIN
        
 .l0:   mov ST_COUNTER(%rbp), %r15   /* loop counter to decrement */
        cmp $0, %r15
        je .l0d

        mov $CLOCK_MONOTONIC, %rdi
        mov $.t0, %rsi
        call clock_gettime

        call * ST_FUNCTION(%rbp)

        mov $CLOCK_MONOTONIC, %rdi
        mov $.t1, %rsi
        call clock_gettime

        mov $.t0, %rdi
        mov $.t1, %rsi
        call mark_time

        mov ST_COUNTER(%rbp), %rax
        dec %rax
        mov %rax, ST_COUNTER(%rbp)
        jmp .l0
.l0d:        
        call print_stats

        pop %rsi
        pop %rsi
        pop %rdi
        pop %rbp
        ret
        

/** 
  Calculate elapsed time and update statistics
   rdi - pointer to T0 struct
   rsi - pointer to T1 struct
*/
        .type mark_time, function
mark_time:
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
        jae .notmin
        mov %rax, .stats+MIN
        .notmin:
        mov .stats+COUNT, %rax
        inc %rax
        mov %rax, .stats+COUNT
        ret


/**
  Calculate the elapsed time between to timer points in nanoseconds
    rdi - pointer to T0 struct
    rsi - pointer to T1 struct
*/
        .type calc_elapsed, function
calc_elapsed:
        mov (%rsi), %rax 
        sub (%rdi), %rax /* subtract the seconds part */
        .equ NS_PER_SEC, 1000000000
        mov $NS_PER_SEC, %rcx
        mul %rcx
        /* now offset by the nanoseconds */
        add 8(%rsi), %rax
        sub 8(%rdi), %rax
        ret
        
        .type print_stats, function
print_stats:
        
        mov $outmsg, %rdi
        mov .stats+MIN, %rsi
        movsd .stats+SUM, %xmm0
        mov .stats+COUNT, %rax
        cvtsi2sd %rax, %xmm3
        divsd %xmm3, %xmm0 /* xmm0 has average */
        movsd .stats+SUMSQ, %xmm1
        divsd %xmm3, %xmm1 /* xmm1 has sumsq/N */
        movsd %xmm0, %xmm3
        mulsd %xmm0, %xmm3 /* xmm3 - avg*avg */
        subsd %xmm3, %xmm1 /* xmm2 - Var = sumsq/N - avg^2 */
        sqrtsd %xmm1, %xmm1       /* xmm2 - stddev = sqrt(Var) */
        mov $2, %rax
        call printf
        ret

        .section .rodata
outmsg: .asciz "Elapsed time- min: %dns, avg: %.0f, stddev: %.0f\n"
