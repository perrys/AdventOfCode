
        .section .rodata
        .equ FD_STDIN, 0
        .equ BUFFSIZE, 16000
        .equ N_COUNT, 10000

        .section .bss
        .lcomm in_buffer, BUFFSIZE

        .section .data
bytes_read: .quad 0
vals_array: .quad 0, 0, 0
max_fmtstr: .asciz "Max is: %ld\n"
top3_fmtstr: .asciz "Top 3 sum to: %ld\n"
        
        .section .text


/**        
  Advent of code 2022 day. Read the entire file contents into *in_buffer, then
  parse the contents, anaysing each block according to the rules of part1 or
  part 2.
*/
        .type _start, function
        .global _start
_start:
        .equ STSIZE, 2*8 /* keep this 16-byte aligned when calling C functions */
        mov %rsp, %rbp
        sub $STSIZE, %rsp

        /* read the input file */
        mov $FD_STDIN, %rdi
        mov $in_buffer, %rsi
        mov $BUFFSIZE, %rdx
        call read_file
        mov %rax, bytes_read

        /* part 1 */
        mov $part1, %rdi
        mov $N_COUNT, %rsi
        call perf_timer
//        call part1 
        mov $max_fmtstr, %rdi
        mov vals_array, %rsi
        mov $0, %rax
        call printf
        
        /* part 2 */
        mov $0, %rax
        mov %rax, vals_array
        mov %rax, vals_array+8
        mov %rax, vals_array+16
        mov $part2, %rdi
        mov $N_COUNT, %rsi
//        call perf_timer
        call part2
        mov $vals_array, %rbx
        mov $0, %rsi
        mov $0, %rdi
.loop_max_vals_sum:        
        add (%rbx, %rdi, 8), %rsi
        inc %rdi
        cmp $3, %rdi
        jne .loop_max_vals_sum

        mov $top3_fmtstr, %rdi
        mov $0, %rax
        call printf
        add $STSIZE, %rsp
        mov $0, %edi
        call exit

/**
  Part 1- find the maximum group sum
*/
        .type part1, function
part1:
        mov $in_buffer, %rdi   /* start of buffer */
        mov $in_buffer, %rsi
        add bytes_read, %rsi   /* end of buffer */
        mov $record_max, %rdx  /* function pointer */
        call parse_file
        ret
        
/**
  Part 2- find the top 3
*/
        .type part2, function
part2:
        mov $in_buffer, %rdi /* start of buffer */
        mov $in_buffer, %rsi
        add bytes_read, %rsi /* end of buffer */
        mov $top3, %rdx      /* function pointer */
        call parse_file
        ret
        

        .type parse_file, function
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

/**
  Record the maximum value in the first element of vals_array
*/        
        .type record_max, function
record_max:
        cmp vals_array, %rdi
        jle .recmax_done
        mov %rdi, vals_array
.recmax_done:
        ret

/**
 Remember the top 3 values of the number passed in
  %rdi - the value
*/
        .type top3, function
        .global top3
top3:
        push %rbp
        mov %rsp, %rbp
        mov $vals_array, %rdx /* pointer to the 3-array */
        .equ IDX_0, 0
        .equ IDX_1, 8
        .equ IDX_2, 16
        /* sort array */
        cmp IDX_0(%rdx), %rdi 
        jle .done
        mov %rdi, IDX_0(%rdx)
        cmp IDX_1(%rdx), %rdi 
        jle .done
        mov IDX_1(%rdx), %rax /* shuffle down */
        mov %rax, IDX_0(%rdx)
        mov %rdi, IDX_1(%rdx)
        cmp IDX_2(%rdx), %rdi
        jle .done
        mov IDX_2(%rdx), %rax /* shuffle down */
        mov %rax, IDX_1(%rdx)
        mov %rdi, IDX_2(%rdx)
.done:       
        leave
        ret

        
