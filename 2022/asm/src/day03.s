
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
        xor %r11, %r11          # score
        mov $in_buffer, %r10    # buffer ptr 
        xor %rsi, %rsi          # line counter
p1_newline:
        cmp %rsi, num_lines
        je p1_done
        mov line_endings(,%rsi,8), %rdx  # end-of-line ptr
        sub %r10, %rdx                # num characters on this line
        shr $1, %rdx                  # half num chars 
p1_first_half:
        xor %rdi, %rdi                # line index
        xor %r8, %r8                  # item mask
        xor %rcx, %rcx                # byte holder
p1_first_loop:                      
        cmp %rdx, %rdi
        je p1_second_half
        movb (%r10, %rdi), %cl  #read byte 
        inc %rdi
        andb $63, %cl           # cl = cl mod 64
        mov $1, %rax
        shl %cl, %rax           # shift to place in bit vector
        or %rax, %r8            # set bit in bit vector
        jmp p1_first_loop
p1_second_half:
        add %rdi, %r10          # move buffer to second half of line
        xor %rdi, %rdi          # reset line index
p1_second_loop:
        cmp %rdi, %rdx
        je p1_notfound
        movb (%r10, %rdi), %cl  # read byte 
        inc %rdi
        xor %r9, %r9
        movb %cl, %r9b          # copy of byte read
        andb $63, %cl
        mov $1, %rax
        shl %cl, %rax
        and %r8, %rax
        jz p1_second_loop
        add %rdx, %r10          # found it!. now move buffer to second half of line 
        inc %r10                # skip newline
        mov %r9, %rdi
        call get_priority
        add %rax, %r11           # add to score
        push %rdi
        push %rsi
        push %rdx
        push %rcx
        push %r10
        push %r11
        mov $dbg_msg, %rdi
        mov %r9, %rdx
        mov %rax, %rcx
        call printf
        pop %r11
        pop %r10
        pop %rcx
        pop %rdx
        pop %rsi
        pop %rdi
        inc %rsi
        jmp p1_newline
p1_notfound:
        mov $notfound_errmsg, %rdi # rsi is already set to line count
        mov $0, %rax
        call printf
        mov $1, %rdi
        call exit
p1_done:
        mov %r11, %rax
        pop %rbp
        ret

        .section .rodata
dbg_msg:        .asciz "[%d] %c - %d\n"
        .section .text
        
        
        
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
        
