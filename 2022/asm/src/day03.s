
        .section .rodata
        .equ FD_STDIN, 0
        .equ BUFFSIZE, 16000
        .equ LINE_END_SIZE, 400*8
out_msg: .asciz "Score 1 is %ld\n"
notfound_errmsg: .asciz "ERROR: no duplicate found on line %d\n"

        .section .data
end_of_file:    .quad 0
num_lines:      .quad 0
        
        .section .text

        .global _start
        .type _start, function
_start:
        .equ STSIZE, 2*8 
        mov %rsp, %rbp
        sub $STSIZE, %rsp
        mov $FD_STDIN, %rdi
        mov $in_buffer, %rsi
        call read_file
        add $in_buffer, %rax
        mov %rax, end_of_file
        
        call get_line_endings
        call part1      

        mov $out_msg, %rdi
        mov %rax, %rsi
        mov $0, %rax
        call printf
        add $STSIZE, %rsp
        mov $0, %edi
        call exit

### Parse the buffer one line at a time
        .type part1, function
part1:
        push %rbp
        mov %rsp, %rbp
        xor %r15, %r15          # score
        mov $in_buffer, %r14    # buffer ptr 
        xor %r13, %r13          # line counter
p1_newline:
        cmp %r13, num_lines
        je p1_done
        mov line_endings(,%r13,8), %r12  # end-of-line ptr
        sub %r14, %r12                   # num characters on this line
        shr $1, %r12                     # half num chars 
        mov %r14, %rdi
        mov %r12, %rsi
        call get_mask_for_slice
        mov %rax, %rbx
        add %r12, %r14          # move buffer to second half of line
        xor %rdi, %rdi          # reset line index
p1_second_half_loop:
        cmp %rdi, %r12
        je p1_notfound
        movb (%r14, %rdi), %cl  # read byte 
        inc %rdi
        xor %r9, %r9
        movb %cl, %r9b          # copy of byte read
        call get_mask_for_char
        and %rbx, %rax          # test if this item is already present in the mask
        jz p1_second_half_loop  # if not, loop again
        add %r12, %r14          # found it! Now move buffer to next line
        inc %r14                # skip newline
        mov %r9, %rdi
        call get_priority
        add %rax, %r15           # add to score
        inc %r13
        jmp p1_newline
p1_notfound:
        mov $notfound_errmsg, %rdi
        mov %r13, %rsi
        mov $0, %rax
        call printf
        mov $1, %rdi
        call exit
p1_done:
        mov %r15, %rax
        pop %rbp
        ret

        .section .rodata
dbg_msg:        .asciz "[%d] %c - %d\n"
        .section .text
        
### Calculate the bitmask for the given line of items
###  %rdi - pointer to line
###  %rsi - number of items to read
### Returns the 64bit mask in %rax
        .type get_mask_for_slice, function
get_mask_for_slice:
        add %rdi, %rsi          # rsi is now pointer to last byte
        xor %rdx, %rdx          # zero the result
mask_loop:                      
        cmp %rsi, %rdi
        je mask_done
        movb (%rdi), %cl        # read byte
        inc %rdi
        call get_mask_for_char
        or %rax, %rdx           # set bit in bit vector
        jmp mask_loop
mask_done:
        mov %rdx, %rax
        ret
        
### Calculate the 64-bit mask for a single character
###  %cl - byte register should contain the character
### Returns the 64-bit mask in %rax
        .type get_mask_for_char, function
get_mask_for_char:
        andb $63, %cl           # cl = cl mod 64
        mov $1, %rax
        shl %cl, %rax           # shift to place in bit vector
        ret
        
        
### Get the score (aka prioirty) for this item code
###  %dil: the item code
### returns the score in %rax
        .type get_priority, function
get_priority:
        xor %rax, %rax
        cmpb $'a', %dil
        jge lowercase
        subb $'A'-27, %dil
        movb %dil, %al
        ret
lowercase:
        subb $'a'-1, %dil
        movb %dil, %al
        ret
        
### Parse the buffer to establish line ending positions

        .type get_line_endings, function #
get_line_endings:
        push %rbp
        mov %rsp, %rbp
        push %r15
        mov $in_buffer, %rdi    # buffer pointer 
        mov end_of_file, %rsi  # end-of-buffer pointer
        xor %rdx, %rdx          # character register 
        mov $line_endings, %rcx # end-of-line array
        mov $'\n', %al
next_chr:
        cmp %rdi, %rsi
        jz eof
        movb (%rdi), %dl
        cmpb %dl, %al
        jne not_newline
        mov %rdi, (%rcx)
        add $8, %rcx
not_newline:    
        inc %rdi
        jmp next_chr
eof:
        sub $line_endings, %rcx
        shr $3, %rcx            # nlines = diff / 8
        mov %rcx, num_lines
        pop %r15
        pop %rbp
        ret
        
        .section .bss
        .lcomm in_buffer, BUFFSIZE
        .lcomm line_endings, LINE_END_SIZE
        
