###
### Advent of code challenge 2022 day 02.
### Solution in x64 assembly for linux.
### 
### See https://adventofcode.com/2022/day/2
###

        .equ FD_STDIN, 0
        .equ FD_STDERR, 1
        .equ BUFFSIZE, 16000
        .equ N_COUNT, 10000

###
### Read only data
### 
        .section .rodata
part1_msg: .asciz "Part 1 score is %ld\n"
part2_msg: .asciz "Part 2 score is %ld\n"
invalid_file_msg: .asciz "ERROR: data file invalid - should be multiple of 4 bytes long"
invalid_args_msg: .asciz "USAGE: %s <filename>\n unexpected number of arguments: %d\n"

###
### Initialized data
### 
        .section .data
end_of_buffer:  .quad 0
processor_func:  .quad 0
score:  .quad 0

###
### Code section
### 
        .section .text
        .global _start
        .type _start, function
_start:
        .equ STSIZE, 2*8
        .equ BYTES_READ, -1*8
        mov %rsp, %rbp
        sub $STSIZE, %rsp

        mov (%rbp), %rdx        # number of program args
        cmp $2, %rdx
        je .L_args_valid
        ## Invalid arguments, print error and exit
        mov $FD_STDERR, %rdi
        mov $invalid_args_msg, %rsi
        mov 8(%rbp), %rdx       # prgram name
        mov (%rbp), %rcx        # number of args (incl prog name)
        dec %rcx
        mov $0, %rax
        call dprintf
        mov $1, %rax
        call exit
.L_args_valid:     
        ## read the input file 
        mov 16(%rbp), %rdi      # first program argument (=input filename)
        mov $in_buffer, %rsi
        mov $BUFFSIZE, %rdx
        call open_and_read_file
        mov %rax, %rsi          # number of bytes read 
        add $in_buffer, %rsi
        mov %rsi, end_of_buffer
        and $0b11, %rax         # file length must be a multiple of 4
        jz .L_file_validated
        mov $invalid_file_msg, %rdi 
        mov $0, %rax
        call printf
        mov $1, %rdi
        call exit
.L_file_validated:        
        ## time part 1:
        mov $part1, %rax
        mov %rax, processor_func
        mov $parse_file, %rdi
        mov $N_COUNT, %rsi
        call perf_timer
        ## output part 1:
        mov $part1_msg, %rdi
        mov score, %rsi
        mov $0, %rax
        call printf
        ## time part 2:
        mov $part2, %rax
        mov %rax, processor_func
        mov $parse_file, %rdi
        mov $N_COUNT, %rsi
        call perf_timer
        ## output part 2:
        mov $part2_msg, %rdi
        mov score, %rsi
        mov $0, %rax
        call printf
        mov $0, %edi
        call exit

###  Parse the data file. Each line must be of the form "A X\n"
###  where A and X are characters.
###  Returns the final score
        .type parse_file, function
parse_file:
        push %rbp
        mov %rsp, %rbp
        push %r12
        push %r13
        push %r14
        push %r15
        mov processor_func, %r12 # r12 = function ptr 
        mov $in_buffer, %r13     # r13 = current datum ptr 
        mov end_of_buffer, %r14  # r14 = eof ptr 
        xor %r15, %r15           # r15 = points total 
.L_newline:
        xor %rdi, %rdi
        xor %rsi, %rsi
        movb (%r13), %dil       # read the first char into %rdi
        movb 2(%r13), %sil      # read the second char into %rsi
        call *%r12              # call processing function
        add %rax, %r15          # add score to total 
        add $4, %r13            # jump 4 chars to next line
        cmp %r13, %r14
        je .L_eof
        jmp .L_newline
.L_eof:
        mov %r15, score
        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %rbp
        ret

### Part1 - the characters represent their throw and my throw respectively
###        %rdi - first character (theirs)
###        %rsi - second character (mine)
### Return - score for the round
        .type part1, function
part1:
        push %r12
        push %r13
        xor %rax, %rax
        mov %rsi, %r12   # r12 - my char 
        mov $'A', %rsi
        call normalize_throw
        mov %rax, %r13   # r13 - their throw 
        mov %r12, %rdi   # rdi - my char 
        mov $'X', %rsi
        call normalize_throw
        mov %rax, %r12   # r12 - my throw 
        mov %rax, %rdi   # my throw 
        mov %r13, %rsi   # their throw 
        call get_score
        add %r12, %rax   # add my throw to total 
        pop %r13
        pop %r12
        ret
        
### Part2 - the characters represent their throw and the result respectively
###        %rdi - first character (theirs)
###        %rsi - second character (result)
### Return - score for the round
        .type part1, function
part2:
        push %r12
        push %r13
        xor %rax, %rax
        mov %rsi, %r12   # r12 - result char 
        mov $'A', %rsi
        call normalize_throw
        mov %rax, %r13   # r13 - their throw 
        mov %r12, %rdi   # rdi - result char 
        mov $'X', %rsi
        call normalize_result
        mov %rax, %r12   # r12 - result 
        mov %rax, %rdi   # result 
        mov %r13, %rsi   # their throw 
        call get_my_throw
        mov %rax, %r13   # r13 - score for throw 
        mov %r12, %rdi
        call get_score_for_result
        add %r13, %rax   # rax = result points + throw points 
        pop %r13
        pop %r12
        ret
        

### Get throw type from the given character, with the given offset:
###        %rsi - character
###        %rdi - offset
### Returns: 1=Rock, 2=Paper, 3=Scissors
### volatile registers are rdi, rsi and rax
        .type normalize_throw,  function
normalize_throw:      
        sub %rsi, %rdi
        mov %rdi, %rax
        inc %rax
        ret


### Get result from the given character, with the given offset:
###        %rsi - character
###        %rdi - offset
### Returns: -1:Lose, 0:Draw, 1:Win
### volatile registers are rdi, rsi and rax
        .type normalize_result,  function
normalize_result:      
        sub %rsi, %rdi
        mov %rdi, %rax
        dec %rax
        ret


### Get the throw required for the given result
###        %rdi - required result
###        %rsi - their throw
### Returns: my throw Rock=1 etc
        .type get_my_throw, function
get_my_throw:
        ## in order to do this without branching, we will calculate the result as
        ## (x+T), where x is their throw and T is 2 for lose, 3 for draw and 4 for win
        ## then normalize the result back to 1<=x<=3 
        add $3, %rdi     # rdi is T 
        add %rsi, %rdi   # rdi now has range 3-7, so normalize it 
        mov $3, %rsi     # this will be our multiplier 
        mov %rdi, %rax   # pass 1 
        shr $2, %rax
        and $0b1, %rax # rax is now 1 if rdi is greater than 3, 0 otherwise 
        mul %rsi       # rax = 0 or 3 
        sub %rax, %rdi
        mov %rdi, %rax   # pass 2 
        shr $2, %rax
        and $0b1, %rax # rax is now 1 if rdi is greater than 3, 0 otherwise 
        mul %rsi       # rax = 0 or 3 
        sub %rax, %rdi
        mov %rdi, %rax
        ret
       
        

### Get the result of my throw and their throw. 
###        %rdi - my throw
###        %rsi - their throw
### Returns:
###  -1 = lose
###   0 = draw
###   1 = win
        .type get_result, function
get_result:     
        ## so each throw beats the one directly below it in the
        ## scoring hierarchy (i.e. paper beats rock, sciscors
        ## beats paper, except in the rock vs sciscors case where it
        ## wraps around 
        sub %rsi, %rdi # %rdi = mine - theirs 
        ## it is faster to omit this early exit in the case of a draw; this 
        ## is almost certainly due to the branch causing pipeline stalls 
        ## jnz .L_notdraw
        ## mov %rdi, %rax
        ## ret
.L_notdraw:
        ## so rdi has the correct result except for R vs S (it is -2, should be 1)
        ## and S vs R (it is 2, should be -1). So first check if abs(rdi) is 2 
        push %rdi
        call abs         # rax = abs(rdi) 
        pop %rdi
        shr $1, %rax     # rax is now 1 if abs(result) was 2, 0 otherwise 
        mov %rax, %rcx   # so it is a boolean for the wraparound case  
        sar %cl, %rdi    # divide by 2 for the wraparound case 
        mov %rax, %rsi   # now multiply by -1 for wraparound (using 2's complement).. 
        shl $63, %rsi    # these two shifts fill rsi with 1s in the wraparound.. 
        sar $63, %rsi    # ..case, or with 0s otherwise 
        xor %rsi, %rdi   # one's complement when wraparound, nop otherwise 
        add %rdi, %rax   # then adjust to two's complement when wraparound 
        ret


### Get the score for the given result.
###        %rdi - result -1=lose, 0=draw, 1=win
### Returns: 0 for loss, 3 for draw, 6 for win
        .type get_score_for_result, function
get_score_for_result:
        inc %rdi
        mov $3, %rax
        mul %rdi
        ret

        

### Return a score for the given throws.
###        %rdi - my throw
###        %rsi - their throw
### Returns: 0 for loss, 3 for draw, 6 for win
        .type get_score, function
get_score:
        call get_result
        mov %rax, %rdi
        call get_score_for_result
        ret

        .section .bss
        .lcomm in_buffer, BUFFSIZE
