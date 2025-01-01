### AoC 2024
### Day 1: Historian Hysteria

        .equ FD_STDERR, 2
        .equ FD_STDOUT, 1
        .equ BUFFSIZE, 16000

###
### Uninitialized data
### 
        .section .bss
        .lcomm in_buffer, BUFFSIZE
        .lcomm list1, 4000
        .lcomm list2, 4000

###
### Read only data
### 
        .section .rodata
bytes_read_msg: .asciz "Read: %ld bytes from input file\n"
invalid_args_msg: .asciz "USAGE: %s <filename>\n unexpected number of arguments: %d\n"
part1_msg: .asciz "Part1 total is %d\n"


###
### Code section 
### 

        .section .text
        
        .type _start function
        .global _start

_start:
        mov %rsp, %rbp
        mov (%rbp), %rdx          # number of args
        cmp $2, %rdx
        je .L_args_valid
        mov $invalid_args_msg, %rsi
        mov $FD_STDERR, %rdi
        mov 8(%rbp), %rdx       # program name
        mov (%rbp), %rcx        # number of args (incl prog name)
        dec %rcx
        mov $0, %rax
        call dprintf
        mov $1, %rax
        call exit

.L_args_valid:
        ## Read the input file
        mov 16(%rbp), %rdi
        mov $in_buffer, %rsi
        mov $BUFFSIZE, %rdx
        call open_and_read_file
        mov $FD_STDOUT, %rdi
        mov $bytes_read_msg, %rsi
        mov %rax, %rdx
        mov %rax, %r15
        mov $0, %rax
        call dprintf

        ## Parse the input file data into two lists of ints
        mov $in_buffer, %rdi
        mov %r15, %rsi
        mov $list1, %rdx
        mov $list2, %rcx
        call parse_input
        mov %rax, %r15          # number of lines

        ## Sort the first list
        mov $list1, %rdi
        mov %r15, %rsi
        call gnome_sort

        ## Sort the second list
        mov $list2, %rdi
        mov %r15, %rsi
        call gnome_sort

        ## Calculate the total distance
        mov $0, %rax            # part1 total
        mov $0, %rdi            # loop index
        mov $list1, %r8
        mov $list2, %r9
1:
        cmp %rdi, %r15
        je .L_print_total
        mov (%r8, %rdi, 4), %r10d
        mov (%r9, %rdi, 4), %r11d
        sub %r10d, %r11d
        jge .L_nonnegative
        imul $-1, %r11d
.L_nonnegative:
        add %r11d, %eax
        inc %rdi
        jmp 1b
        
.L_print_total:
        mov $FD_STDOUT, %rdi
        mov $part1_msg, %rsi
        mov %rax, %rdx
        mov $0, %rax
        call dprintf

        mov $0, %rdi
        call exit


### Parse the given input buffer into lists 1 and 2
### %rdi = buffer address
### %rsi = data size
### %rdx = list1 address
### %rcx = list2 address

        .type parse_input function
parse_input:
        push %rbp
        mov %rsp, %rbp
        mov %rdi, %r8           # buffer address
        mov %rsi, %r9           # data size
        mov %rdx, %r10          # list1 address
        mov %rcx, %r11          # list2 address
        mov $0, %rdi            # buffer index
        mov $0, %rsi            # line index
        mov $0, %rax            # atoi accumulator
        mov $0, %rdx            # data byte
1:      
        cmp %rdi, %r9
        je .L_pi_done
        mov (%r8,%rdi,1), %dl   # read next input byte
        cmpb $0x20,%dl          # space?
        jne .L_pi_notspace       
        cmp $0,%rax
        je .L_pi_next           # if accumulator is zero, just a repeated space
        mov %eax, (%r10,%rsi,4) # populate list1
        mov $0, %rax            # reset accumulator
        jmp .L_pi_next
.L_pi_notspace:
        cmpb $0x0A,%dl          # newline?
        jne .L_pi_atoi
        mov %eax, (%r11,%rsi,4) # populate list2
        mov $0, %rax            # reset accumulator
        inc %rsi                # next line
        jmp .L_pi_next
.L_pi_atoi:
        imul $10,%eax           # shift accumulator left (in decimal)
        subb $'0',%dl           # numeric value of ascii digit
        add %edx, %eax          # update accumulator
.L_pi_next:
        inc %rdi                # move to next input byte
        jmp 1b
.L_pi_done:
        mov %rsi, %rax
        pop %rbp
        ret
        

### Sort the given buffer in place
### %rdi = buffer address
### %rsi = data size      
        
        .type gnome_sort function
gnome_sort:
        push %rbp
        mov %rsp, %rbp
        mov %rdi, %rax          # buffer address
        mov %rsi, %rdx          # number of bytes
        mov $0, %rdi            # index var
1:
        cmp %rdx, %rdi          # test for end of data
        je .L_gs_done            
        cmp $0, %rdi            # test for first datapoint
        je .L_gs_fwd
        mov (%rax,%rdi,4), %r9d  # data[i]
        mov -4(%rax,%rdi,4), %r8d # data[i-1]
        cmp %r8d, %r9d            # compare prev to this
        jge .L_gs_fwd             # if destination operand is >=
        ##  wrong order, so swap and move backwards
        mov %r8d,(%rax,%rdi,4)  # swap data[i-1]
        mov %r9d,-4(%rax,%rdi,4) # with data[i]
        dec %rdi
        jmp 1b
.L_gs_fwd:
        inc %rdi
        jmp 1b
.L_gs_done:
        pop %rbp
        ret
        
