###
### Advent of code challenge 2022 day 01.
### Solution in x64 assembly for linux.
### 
### See https://adventofcode.com/2022/day/1
###
        
        .equ FD_STDIN, 0
        .equ FD_STDERR, 1
        .equ BUFFSIZE, 16000
        .equ N_COUNT, 10000

###
### Unnitialized data
### 
        .section .bss
        .lcomm in_buffer, BUFFSIZE

###
### Read only data
### 
        .section .rodata
max_fmtstr: .asciz "Max is: %ld\n"
top3_fmtstr: .asciz "Top 3 sum to: %ld\n"
invalid_args_msg: .asciz "USAGE: %s <filename>\n unexpected number of arguments: %d\n"

###
### Initialized data
### 
        .section .data
bytes_read: .quad 0
vals_array: .quad 0, 0, 0
        .equ IDX_0, 0
        .equ IDX_1, 8
        .equ IDX_2, 16
        
###
### Code section
### 
        .section .text
 
### Read the entire file contents into *in_buffer, then parse the contents,
### anaysing each block according to the rules of part1 or part 2.
        .type _start, function
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
        mov $0, %rax
        call dprintf
        mov $1, %rax
        call exit
args_valid:     
        ## read the input file 
        mov 16(%rbp), %rdi      # first program argument (=input filename)
        mov $in_buffer, %rsi
        mov $BUFFSIZE, %rdx
        call open_and_read_file
        mov %rax, bytes_read
        ## Time part 1
        mov $part1, %rdi
        mov $N_COUNT, %rsi
        call perf_timer
        mov $max_fmtstr, %rdi
        mov vals_array, %rsi
        mov $0, %rax
        call printf
        ## Time part 2:
        mov $part2, %rdi
        mov $N_COUNT, %rsi
        call perf_timer
        ## Sum the top 3 values:
        mov $vals_array, %rbx
        mov $0, %rax
        add (%rbx), %rax
        add 8(%rbx), %rax
        add 16(%rbx), %rax
        mov %rax, %rsi
        mov $top3_fmtstr, %rdi
        mov $0, %rax
        call printf
        mov $0, %edi
        call exit

### Part 1- find the maximum group sum
        .type part1, function
part1:
        mov $in_buffer, %rdi    # start of buffer
        mov $in_buffer, %rsi
        add bytes_read, %rsi    # end of buffer 
        mov $record_max, %rdx   # function pointer 
        call parse_file
        ret
        
### Part 2- find the top 3
        .type part2, function
part2:
        mov $vals_array, %rdx   # Zero the array for each run
        movq $0, IDX_0(%rdx)
        movq $0, IDX_1(%rdx)
        movq $0, IDX_2(%rdx)
        mov $in_buffer, %rdi    # start of buffer
        mov $in_buffer, %rsi
        add bytes_read, %rsi    # end of buffer
        mov $top3, %rdx         # function pointer 
        call parse_file
        ret
        
### Parse a file of numbers delimited by newlines
### with groups of numbers separated by blank lines.
### Call back the supplied function with the sum of each
### group of numbers
###   %rdi - input pointer
###   %rsi - end-of-input pointer
###   %rdx - callback function
        .type parse_file, function
parse_file:
        push %rbp
        mov %rsp, %rbp
        push %r15
        push %r14
        .equ STSIZE, 16
        .equ ST_CALLBACK, -16
        sub $STSIZE, %rsp
        mov %rdx, ST_CALLBACK(%rbp)
        mov %rdi, %r14          # input pointer
        mov %rsi, %r15          # end-of-input pointer
newbatch:                      
        xor %rcx, %rcx          # group accumulator
newline:
        xor %rax, %rax          # atoi accumulator
byte_parse_loop:
        cmp %r14, %r15
        je eof
        xor %rbx, %rbx
        movb (%r14), %bl        # read the current byte
        add $1, %r14            # increment buffer pointer
        cmpb $'\n', %bl         # check or end-of-line
        je eol
        mov $10, %r8d           # multiply current total by 10
        mull %r8d
        sub $'0', %bl           # convert ascii to number
        add %rbx, %rax          # add to the total
        jmp byte_parse_loop
eol:
        cmp $0, %rax            # empty line signifies end-of-group
        je savebatch
        add %rax, %rcx          # add this number to the group total
        jmp newline
savebatch:       
        mov %rcx, %rdi          # argument to callback is the group total
        call *ST_CALLBACK(%rbp)
        jmp newbatch
eof:
        add $STSIZE, %rsp
        pop %r14
        pop %r15
        pop %rbp
        ret

### Record the maximum value in the first element of vals_array
###   %rdi - the value
        .type record_max, function
record_max:
        cmp vals_array, %rdi
        jle recmax_done
        mov %rdi, vals_array
recmax_done:
        ret

### Remember the top 3 values of the number passed in
###   %rdi - the value
        .type top3, function
top3:
        push %rbp
        mov %rsp, %rbp
        mov $vals_array, %rdx   # pointer to the 3-array
        ## Keep the array sorted by comparing subsequent
        ## values and shuffling:
        cmp IDX_0(%rdx), %rdi 
        jle t3_done
        mov %rdi, IDX_0(%rdx)
        cmp IDX_1(%rdx), %rdi 
        jle t3_done
        mov IDX_1(%rdx), %rax   # shuffle down
        mov %rax, IDX_0(%rdx)
        mov %rdi, IDX_1(%rdx)
        cmp IDX_2(%rdx), %rdi
        jle t3_done
        mov IDX_2(%rdx), %rax   # shuffle down 
        mov %rax, IDX_1(%rdx)
        mov %rdi, IDX_2(%rdx)
t3_done:       
        leave
        ret

        
