
        .section .rodata
        .equ FD_STDIN, 0
        .equ BUFFSIZE, 16000

.part1_msg: .asciz "Part 1 score is %ld\n"
.part2_msg: .asciz "Part 2 score is %ld\n"
.invalid_file_msg: .asciz "ERROR: data file invalid - should be multiple of 4 bytes long"

        .section .text
        .global _start
        .type _start, function
_start:
        .equ STSIZE, 1*8 /* keep this 16-byte aligned when calling C functions */
        .equ BYTES_READ, -1*8
        push %rbp
        mov %rsp, %rbp
        sub $STSIZE, %rsp

        mov $FD_STDIN, %rdi
        mov $in_buffer, %rsi
        call read_file
        mov %rax, BYTES_READ(%rbp)
        
        mov $in_buffer, %rdi /* start of buffer */
        mov %rax, %rsi
        add %rdi, %rsi       /* end of buffer */
        mov $part1, %rdx
        call parse_file
        
        mov $.part1_msg, %rdi
        mov %rax, %rsi
        mov $0, %rax
        call printf

        mov $in_buffer, %rdi /* start of buffer */
        mov BYTES_READ(%rbp), %rsi
        add %rdi, %rsi       /* end of buffer */
        mov $part2, %rdx
        call parse_file
        
        mov $.part2_msg, %rdi
        mov %rax, %rsi
        mov $0, %rax
        call printf

        mov $0, %edi
        call exit

/**
 Parse the data file. Each line must be of the form "A X\n"
 where A and X are characters.
        %rdi - input data start addr
        %rsi end addr
        %rdx - score calculation function pointer      
 Returns the final score
*/
        .type parse_file, function
parse_file:     
        push %rbp
        mov %rsp, %rbp
        push %r12
        push %r13
        push %r14
        push %r15
        mov %rdx, %r12 /* r12 = function ptr */
        mov %rdi, %r13 /* r13 = current datum ptr */
        mov %rsi, %r14 /* r14 = eof ptr */
        xor %r15, %r15 /* r15 = points total */
        sub %rdi, %rsi
        and $0b11, %rsi
        jz .validated
        mov $.invalid_file_msg, %rdi 
        mov $0, %rax
        call printf
        mov $1, %rdi
        call exit
.validated:        
        
.newline:
        xor %rdi, %rdi
        xor %rsi, %rsi
        movb (%r13), %dil
        movb 2(%r13), %sil
        call *%r12
        add %rax, %r15 /* add score to total */
        add $4, %r13
        cmp %r13, %r14
        je .eof
        jmp .newline
.eof:
        mov %r15, %rax
        pop %r15
        pop %r14
        pop %r13
        pop %r12
        pop %rbp
        ret

/**
 Part1 - the characters represent their throw and my throw respectively
        %rdi - first character (theirs)
        %rsi - second character (mine)
 Return - score for the round
*/        
        .type part1, function
part1:
        push %r12
        push %r13
        xor %rax, %rax
        mov %rsi, %r12 /* r12 - my char */
        mov $'A', %rsi
        call normalize_throw
        mov %rax, %r13 /* r13 - their throw */
        mov %r12, %rdi /* rdi - my char */
        mov $'X', %rsi
        call normalize_throw
        mov %rax, %r12 /* r12 - my throw */
        mov %rax, %rdi /* my throw */
        mov %r13, %rsi /* their throw */
        call get_score
        add %r12, %rax /* add my throw to total */
        pop %r13
        pop %r12
        ret
        
/**
 Part2 - the characters represent their throw and the result respectively
        %rdi - first character (theirs)
        %rsi - second character (result)
 Return - score for the round
*/        
        .type part1, function
part2:
        push %r12
        push %r13
        xor %rax, %rax
        mov %rsi, %r12 /* r12 - result char */
        mov $'A', %rsi
        call normalize_throw
        mov %rax, %r13 /* r13 - their throw */
        mov %r12, %rdi /* rdi - result char */
        mov $'X', %rsi
        call normalize_result
        mov %rax, %r12 /* r12 - result */
        mov %rax, %rdi /* result */
        mov %r13, %rsi /* their throw */
        call get_my_throw
        mov %rax, %r13 /* r13 - score for throw */
        mov %r12, %rdi
        call get_score_for_result
        add %r13, %rax /* rax = result points + throw points */
        pop %r13
        pop %r12
        ret
        
/**
 Get throw type from the given character, with the given offset:
        %rsi - character
        %rdi - offset
 Returns: 1=Rock, 2=Paper, 3=Scissors
 volatile registers are rdi, rsi and rax
*/
        .type normalize_throw,  function
normalize_throw:      
        sub %rsi, %rdi
        mov %rdi, %rax
        inc %rax
        ret

/**
 Get result from the given character, with the given offset:
        %rsi - character
        %rdi - offset
 Returns: -1:Lose, 0:Draw, 1:Win
 volatile registers are rdi, rsi and rax
*/
        .type normalize_result,  function
normalize_result:      
        sub %rsi, %rdi
        mov %rdi, %rax
        dec %rax
        ret

/**
 Get the throw required for the given result
        %rdi - required result
        %rsi - their throw
 Returns: my throw Rock=1 etc
*/
        .type get_my_throw, function
get_my_throw:
        /* in order to do this without branching, we will calculate the result as
        (x+T), where x is their throw and T is 2 for lose, 3 for draw and 4 for win
        then normalize the result back to 1<=x<=3 */
        add $3, %rdi /* rdi is T */
        add %rsi, %rdi /* rdi now has range 3-7, so normalize it */
        mov $3, %rsi /* this will be our multiplier */
        mov %rdi, %rax /* pass 1 */
        shr $2, %rax
        and $0b1, %rax /* rax is now 1 if rdi is greater than 3, 0 otherwise */
        mul %rsi /* rax = 0 or 3 */
        sub %rax, %rdi
        mov %rdi, %rax /* pass 2 */
        shr $2, %rax
        and $0b1, %rax /* rax is now 1 if rdi is greater than 3, 0 otherwise */
        mul %rsi /* rax = 0 or 3 */
        sub %rax, %rdi
        mov %rdi, %rax
        ret
       
        
/**
 Get the result of my throw and their throw. 
        %rdi - my throw
        %rsi - their throw
 Returns:
  -1 = lose
   0 = draw
   1 = win
*/
        .type get_result, function
get_result:     
        sub %rsi, %rdi /* %rdi = mine - theirs */
        jnz .notdraw
        mov %rdi, %rax
        ret
.notdraw:
        push %rdi
        call abs /* rax = abs(rdi) */
        pop %rdi
        shr $1, %rax   /* rax = 1 if abs(result) was 2, 0 otherwise */
        mov %rax, %rcx
        sar %cl, %rdi /* divide by 2 if rax is 1 */
        mov %rax, %rsi /* now multiply by -1 if rax is 1, using 2's complement */
        shl $63, %rsi  /* make rsi all 1s if.. */
        sar $63, %rsi  /* rax is 1, all 0s otherwise */
        xor %rsi, %rdi /* one's complement if rax is 1 */
        add %rdi, %rax /* then adjust to two's complement */
        ret

/**
 Get the score for the given result.
        %rdi - result -1=lose, 0=draw, 1=win
 Returns: 0 for loss, 3 for draw, 6 for win
*/
        .type get_score_for_result, function
get_score_for_result:
        inc %rdi
        mov $3, %rax
        mul %rdi
        ret

        
/**
 Return a score for the given throws.
        %rdi - my throw
        %rsi - their throw
 Returns: 0 for loss, 3 for draw, 6 for win
*/
        .type get_score, function
get_score:
        call get_result
        mov %rax, %rdi
        call get_score_for_result
        ret

        .section .bss
        .lcomm in_buffer, BUFFSIZE
