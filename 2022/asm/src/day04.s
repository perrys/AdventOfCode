        .equ FD_STDERR, 1
        .equ BUFFSIZE, 16000
        .equ LINE_END_SIZE, 400*8
        .equ N_COUNT, 10000

###
### Read only data
### 
        .section .rodata
p1_out_msg: .asciz "Score 1 is %ld\n"
p2_out_msg: .asciz "Score 2 is %ld\n"
invalid_args_msg: .asciz "USAGE: %s <filename>\n unexpected number of arguments: %d\n"

###
### Initialized Data
### 
        .section .data
bytes_read: .quad 0
        
###
### Uninitialized Data
### 
        .section .bss
        .lcomm in_buffer, BUFFSIZE

###
### Code Section
### 
        .section .text

        .global _start
_start:
        mov %rsp, %rbp
        ## Validate program arguments:
        mov (%rbp), %rdx        # number of program args
        cmp $2, %rdx
        je args_valid
        ## Invalid arguments, print error and exit
        mov $FD_STDERR, %rdi
        mov $invalid_args_msg, %rsi
        mov 8(%rbp), %rdx       # prgram name
        mov (%rbp), %rcx        # number of args (incl prog name)
        dec %rcx
        mov $0, %rax            # no floating point args
        call dprintf
        mov $1, %rax
        call exit
args_valid:     
        .equ STSIZE, 2*8
        sub $STSIZE, %rsp
        mov 16(%rbp), %rdi      # first program argument (=input filename)
        mov $in_buffer, %rsi
        mov $BUFFSIZE, %rdx
        call open_and_read_file
        mov %rax, bytes_read
        ## time part1:
        mov $part1, %rdi
        mov $N_COUNT, %rsi
        call perf_timer
        mov $p1_out_msg, %rdi
        mov %rax, %rsi
        mov $0, %rax
        call printf
        ## time part2:
        mov $part2, %rdi
        mov $N_COUNT, %rsi
        call perf_timer
        mov $p2_out_msg, %rdi
        mov %rax, %rsi
        mov $0, %rax
        call printf
        mov $0, %edi
        call exit

### Part1 closure
        .type part1, function
part1:  
        mov $in_buffer, %rdi
        mov bytes_read, %rsi
        mov $part1_func, %rdx
        call parse_file
        ret

### Part2 closure
        .type part2, function
part2:  
        mov $in_buffer, %rdi
        mov bytes_read, %rsi
        mov $part2_func, %rdx
        call parse_file
        ret

### Parse a buffer of newline-separated lines of the form %d-%d,%d-%d
### and call the provided callback function with an array pointer to four integers
###   %rdi - pointer to the buffer
###   %rsi - size of the buffer
###   %rdx - callback function for each line
        .type parse_file, function
parse_file:
        push %rbp
        mov %rsp, %rbp
        .equ STSIZE, 6*8
        .equ ST_ARRAY, -8*4     # stack-allocated array
        .equ ST_FUNC, -8*5      # stack-allocated callback function
        .equ ST_TOTAL, -8*6     # stack-allocated total score
        sub $STSIZE, %rsp       # make room for above variables on the stack
        movq $0, ST_TOTAL(%rbp) # let total score = 0
        mov %rdi, %r14          # start of buffer pointer (won't change)
        ## note - because line order is not relevant, we will parse the file backwards
        mov %rsi, %r15          # buffer index set to end of buffer
        mov %rdx, ST_FUNC(%rbp)
        dec %r15
nextline:
        xor %r13, %r13          # line token indexer
nexttoken:      
        xor %r12, %r12          # let token sum = 0
        mov $1, %r11d           # let decimal multiplier = 1
atoi_loop:      
        xor %rax, %rax          
        movb (%r14,%r15), %al   # get the byte
        dec %r15                # move backward 1 character
        cmp $0, %r15
        jl processline
        ## first check if we've hit a delimiter:
        cmpb $'-', %al           
        je savetoken
        cmpb $',', %al
        je savetoken
        cmpb $'\n', %al
        je processline
        ## not a delimiter so convert char to decimal and add to total for the token
        sub $'0', %al
        mul %r11d               # multiply %rax by the decimal multiplier
        add %rax, %r12          # add to token total
        mov %r11d, %eax         # scale the decimal multiplier
        mov $10, %edx
        mul %edx
        mov %eax, %r11d
        jmp atoi_loop
savetoken:
        inc %r13
        mov $-8, %rax           # calculate array offset
        mul %r13
        mov %r12, (%rbp,%rax)   # let array[3-i] = token value
        jmp nexttoken
processline:
        cmp $0, %r13            # check if this is the final newline
        je nexttoken
        inc %r13
        mov $-8, %rax           # calculate array offset
        mul %r13
        mov %r12, (%rbp,%rax)   # let array[3-i] = token value
        lea ST_ARRAY(%rbp), %rdi
        call *ST_FUNC(%rbp)
        add %rax, ST_TOTAL(%rbp)
        cmp $0, %r15
        jl parse_done
        jmp nextline
parse_done:
        mov ST_TOTAL(%rbp), %rax # set the return value
        add $STSIZE, %rsp       # restore the stack
        pop %rbp
        ret
        
        
        
### Test if one of the ranges is fully contained by the other
###   %rdi - pointer to a 4-array of qword integers
        .type part1_func, function
part1_func:
        .equ RANGE1_LO, 0
        .equ RANGE1_HI, 8
        .equ RANGE2_LO, 16 
        .equ RANGE2_HI, 24
        ## Note - a handy gdb printf function:
        ## printf "[%d, %d] [%d, %d]\n", *($rdi), *($rdi+8), *($rdi+16), *($rdi+24)
        mov RANGE1_LO(%rdi), %rax
        cmp RANGE2_LO(%rdi), %rax
        je p1_affirmative
        jl p1_testneg
        ## r1[0] > r2[0]
        mov RANGE1_HI(%rdi), %rax
        cmp RANGE2_HI(%rdi), %rax
        jle p1_affirmative
        jmp p1_negative
p1_testneg:
        ## r1[0] < r2[0]
        mov RANGE1_HI(%rdi), %rax
        cmp RANGE2_HI(%rdi), %rax
        jge p1_affirmative
p1_negative:
        mov $0, %rax
        ret
p1_affirmative:
        mov $1, %rax
        ret
        

### Test if one of the ranges is partially contained by the other
###   %rdi - pointer to a 4-array of qword integers
        .type part2_func, function
part2_func:
        .equ RANGE1_LO, 0
        .equ RANGE1_HI, 8
        .equ RANGE2_LO, 16 
        .equ RANGE2_HI, 24
        mov RANGE1_LO(%rdi), %rax
        cmp RANGE2_LO(%rdi), %rax
        je p2_affirmative
        jl p2_testneg
        ## r1[0] > r2[0]
        mov RANGE1_LO(%rdi), %rax
        cmp RANGE2_HI(%rdi), %rax
        jle p2_affirmative
        jmp p2_negative
p2_testneg:
        ## r1[0] < r2[0]
        mov RANGE1_HI(%rdi), %rax
        cmp RANGE2_LO(%rdi), %rax
        jge p2_affirmative
p2_negative:
        mov $0, %rax
        ret
p2_affirmative:
        mov $1, %rax
        ret
        
