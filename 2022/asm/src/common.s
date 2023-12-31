
        .equ SYS_READ, 0
        .equ SYS_OPEN, 2
        .equ SYS_CLOSE, 3
        .equ O_RDONLY, 0
        .equ FD_STDIN, 0
        .equ FD_STDERR, 2

###
### Read only data
### 
        .section .rodata
outmsg: .asciz "Elapsed time- min: %dns, avg: %.0f, stddev: %.0f\n"
input_errmsg: .asciz "ERROR: Unable to read input\n"
open_failed_errmsg: .asciz "ERROR: Could not open file \"%s\"\n"
close_failed_errmsg: .asciz "ERROR: Could not close file \"%s\"\n"

###
### Initialized data
### 
        .section .data
t0_mark: .quad 0,0           
t1_mark: .quad 0,0           
        ## Offsets into the stats struct:
        .equ SUM, 0
        .equ SUMSQ, 8
        .equ COUNT, 16
        .equ MIN, 24
timer_stats: .quad 0,0,0,0

###
### Code Section
### 
        .section .text
        
### Return the absolute value of a (signed) 64-bit integer
### using abs(x) = (x XOR y) - y, where y = x >> 63
###   %rdi - the value
        .type abs, function
        .global abs
abs:
        mov %rdi, %rsi  # rdi = x, rsi will be y 
        sar $63, %rsi   # y = all 1s if rdi was negative, all zeros if positive 
        xor %rsi, %rdi  # let rdi = rdi xor rsi 
        sub %rsi, %rdi  # let rdi = rdi - rsi 
        mov %rdi, %rax
        ret

### Open the given filename and read the contents into a buffer, aborting
### on failure
###   %rdi - char pointer to file name
###   %rsi - pointer to buffer
###   %rdx - size of buffer
        .type open_and_read_file, function
        .global open_and_read_file
open_and_read_file:
        .equ STSIZE, 5*8
        .equ ST_BYTESREAD, -1*8
        .equ ST_FNAME,   -2*8
        .equ ST_BUFFER,  -3*8
        .equ ST_BUFSIZE, -4*8
        .equ ST_FHANDLE, -5*8
        push %rbp
        mov %rsp, %rbp
        sub $STSIZE, %rbp
        mov %rdi, ST_FNAME(%rbp)
        mov %rsi, ST_BUFFER(%rbp)
        mov %rdx, ST_BUFSIZE(%rbp)
        ## open the file read-only:
        mov $O_RDONLY, %rsi
        mov $SYS_OPEN, %rax
        syscall
        mov %rax, ST_FHANDLE(%rbp)
        ## Test if open was successful:
        cmp $0, %rax
        jge opened_ok
        ## Open failed, print error and exit
        mov %rdi, %rdx
        mov $open_failed_errmsg, %rsi
        mov $FD_STDERR, %rdi
        mov $0, %rax
        call dprintf
        mov $1, %rdi
        call exit
opened_ok:      
        mov ST_FHANDLE(%rbp), %rdi 
        mov ST_BUFFER(%rbp), %rsi 
        mov ST_BUFSIZE(%rbp), %rdx 
        call read_file
        mov %rax, ST_BYTESREAD(%rbp)
        ## Now close the file
        mov $SYS_CLOSE, %rax
        mov ST_FHANDLE(%rbp), %rdi 
        syscall
        cmp $0, %rax
        jge closed_ok
        ## Close failed, something is wrong, best to abort:
        mov ST_FNAME(%rbp), %rdx 
        mov $close_failed_errmsg, %rsi
        mov $FD_STDERR, %rdi
        mov $0, %rax
        call dprintf
        mov $1, %rdi
        call exit
closed_ok:
        mov ST_BYTESREAD(%rbp), %rax
        add $STSIZE, %rbp
        pop %rbp
        ret
        
### Read the entire contents of a file into a buffer. Abort if read fails.
###   %rdi - file descriptor
###   %rsi - pointer to buffer
###   %rdx - size of buffer
        .type read_file, function
        .global read_file
read_file:
        .equ ST_BYTESREAD, -1*8
        .equ STSIZE, 2*8
        push %rbp
        mov %rsp, %rbp
        sub $STSIZE, %rsp
        mov $SYS_READ, %rax
        syscall
        mov %rax, ST_BYTESREAD(%rbp)
        cmp $0, %rax
        jg read_success
        ## Error handler:
        mov $FD_STDERR, %rdi
        mov $input_errmsg, %rsi
        mov $0, %rax
        call dprintf
        mov $1, %rax
        call exit
read_success:
        mov ST_BYTESREAD(%rbp), %rax
        add $STSIZE, %rsp
        pop %rbp
        ret

        
### Call a function in a loop to time it repteatedly, and print out its performance
###  %rdi - callback function
###  %rsi - number of times to loop
        .type perf_timer, function
        .global perf_timer
perf_timer:     
        .equ ST_SIZE, 2*8
        .equ ST_FUNCTION, -1*8
        .equ ST_COUNTER, -2*8
        .equ CLOCK_MONOTONIC, 1
        push %rbp
        mov %rsp, %rbp
        push %r15
        push %r14
        push %r13
        sub $ST_SIZE, %rsp
        mov %rdi, ST_FUNCTION(%rbp)
        mov %rsi, ST_COUNTER(%rbp)
        mov $0, %rax
        mov %rax, timer_stats+SUM
        mov %rax, timer_stats+SUMSQ
        mov %rax, timer_stats+COUNT
        mov $0xffffffffffffffff, %rax
        mov %rax, timer_stats+MIN
        
 .l0:   mov ST_COUNTER(%rbp), %r15 # loop counter to decrement 
        cmp $0, %r15
        je .l0d

        mov $CLOCK_MONOTONIC, %rdi
        mov $t0_mark, %rsi
        call clock_gettime

        call * ST_FUNCTION(%rbp)
        mov %rax, %r14

        mov $CLOCK_MONOTONIC, %rdi
        mov $t1_mark, %rsi
        call clock_gettime

        mov $t0_mark, %rdi
        mov $t1_mark, %rsi
        call mark_time

        mov ST_COUNTER(%rbp), %rax
        dec %rax
        mov %rax, ST_COUNTER(%rbp)
        jmp .l0
.l0d:        
        call print_stats

        mov %r14, %rax
        add $ST_SIZE, %rsp
        pop %r13
        pop %r14
        pop %r15
        pop %rbp
        ret
        

### Calculate elapsed time and update statistics
###  %rdi - pointer to T0 struct
###  %rsi - pointer to T1 struct
        .type mark_time, function
mark_time:
        call calc_elapsed
        cvtsi2sd %eax, %xmm0
        movsd timer_stats+SUM, %xmm1
        addsd %xmm0, %xmm1
        movsd %xmm1, timer_stats+SUM
        mulsd %xmm0, %xmm0
        movsd timer_stats+SUMSQ, %xmm1
        addsd %xmm0, %xmm1
        movsd %xmm1, timer_stats+SUMSQ
        cmpq timer_stats+MIN, %rax
        jae .notmin
        mov %rax, timer_stats+MIN
        .notmin:
        mov timer_stats+COUNT, %rax
        inc %rax
        mov %rax, timer_stats+COUNT
        ret

### Calculate the elapsed time between to timer points in nanoseconds
###   rdi - pointer to T0 struct
###   rsi - pointer to T1 struct
        .type calc_elapsed, function
calc_elapsed:
        mov (%rsi), %rax 
        sub (%rdi), %rax        # subtract the seconds part 
        .equ NS_PER_SEC, 1000000000
        mov $NS_PER_SEC, %rcx
        mul %rcx
        ## now offset by the nanoseconds
         
        add 8(%rsi), %rax
        sub 8(%rdi), %rax
        ret
        
        .type print_stats, function
print_stats:
        
        mov $outmsg, %rdi
        mov timer_stats+MIN, %rsi
        movsd timer_stats+SUM, %xmm0
        mov timer_stats+COUNT, %rax
        cvtsi2sd %rax, %xmm3
        divsd %xmm3, %xmm0      # xmm0 has average
        movsd timer_stats+SUMSQ, %xmm1
        divsd %xmm3, %xmm1      # xmm1 has sumsq/N 
        movsd %xmm0, %xmm3
        mulsd %xmm0, %xmm3      # xmm3 - avg*avg
        subsd %xmm3, %xmm1      # xmm2 - Var = sumsq/N - avg^2 
        sqrtsd %xmm1, %xmm1     # xmm2 - stddev = sqrt(Var) 
        mov $2, %rax
        call printf
        ret

