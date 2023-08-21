
        .equ FD_STDIN, 0
        .equ BUFFSIZE, 16000
        .equ LINE_END_SIZE, 400*8
        .equ N_COUNT, 10000

###
### Read only data
### 
        .section .rodata
p1_out_msg: .asciz "Score 1 is %ld\n"
p2_out_msg: .asciz "Score 2 is %ld\n"
dbg_msg: .asciz "[%d] %c - %d\n"
notfound_errmsg: .asciz "ERROR: no duplicate found on line %d\n"

###
### Initialized Data
### 
        .section .data
end_of_file:    .quad 0
num_lines:      .quad 0
        
###
### Uninitialized Data
### 
        .section .bss
        .lcomm in_buffer, BUFFSIZE
        .lcomm line_endings, LINE_END_SIZE
        .lcomm lookup_table, 64
        
###
### Code Section
### 
        .section .text

        .global _start
        .type _start, function
_start:
        mov %rsp, %rbp
        mov $FD_STDIN, %rdi
        mov $in_buffer, %rsi
        call read_file          # read the input file contents
        add $in_buffer, %rax
        mov %rax, end_of_file
        call get_line_endings   # Initialize line endings table
        mov $part1, %rdi        # now time part 1
        mov $N_COUNT, %rsi
        call perf_timer
        mov $p1_out_msg, %rdi
        mov %rax, %rsi
        mov $0, %rax
        call printf
        mov $part2, %rdi        # now time part 2
        mov $N_COUNT, %rsi
        call perf_timer
        mov $p2_out_msg, %rdi
        mov %rax, %rsi
        mov $0, %rax
        call printf
        mov $0, %edi
        call exit

### Find the common item in two halves of each line
### Returns the total score in %rax
        .type part1, function
part1:
        push %rbp
        push %r15
        push %r14
        push %r13
        push %r12
        push %rbx
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
        pop %rbx
        pop %r12
        pop %r13
        pop %r14
        pop %r15
        pop %rbp
        ret

### Find the single common item in a group of 3 lines
### Returns the total score in %rax
        .type part2, function
part2:
        push %rbp
        push %r15
        push %r14
        push %r13
        push %r12
        push %rbx
        mov %rsp, %rbp
        xor %sil, %sil          # initialize the lookup table
        xor %ecx, %ecx
        mov $in_buffer, %r14    # buffer ptr 
p2_ltbl_loop:
        cmp $64, %sil
        je p2_init_done
        mov $'A', %cl            # loop over letters covering A-z
        add %sil, %cl
        inc %sil
        mov %cl, %al                 # remember the character
        andb $63, %cl                # cl %= 64
        movb %al, lookup_table(,%ecx,) # save character to that offset in the lookup table
        jmp p2_ltbl_loop
p2_init_done:   
        xor %r15, %r15          # score
        mov $in_buffer, %r14    # buffer ptr 
        xor %r13, %r13          # line counter
p2_nextgroup:
        xor %rbx, %rbx                # 3-group counter
        mov $0xFFFFFFFFFFFFFFFF, %r11 # 3-group mask
p2_newline:
        cmp $3, %rbx
        je p2_groupdone
        mov line_endings(,%r13,8), %r12  # end-of-line ptr
        sub %r14, %r12                   # num characters on this line
        mov %r14, %rdi
        mov %r12, %rsi
        call get_mask_for_slice
        and %rax, %r11          # merge masks to identify common items
        inc %rbx
        inc %r13
        add %r12, %r14          # move to end of line
        inc %r14                # skip the newline character
        jmp p2_newline
p2_groupdone:
        bsf %r11, %rax          # bit-scan-forward - finds the index of the lowest set bit
        movb lookup_table(,%rax), %dil
        call get_priority
        add %rax, %r15
        cmp %r13, num_lines
        je p2_done
        jmp p2_nextgroup
p2_done:
        mov %r15, %rax
        pop %rbx
        pop %r12
        pop %r13
        pop %r14
        pop %r15
        pop %rbp
        ret

### Calculate the combined bitmask for the given line of items
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
        ## get_mask_for_char inlined here as it doubles the performance!
        andb $63, %cl           # cl = cl mod 64
        mov $1, %rax
        shl %cl, %rax           # shift to place in bit vector
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
        
