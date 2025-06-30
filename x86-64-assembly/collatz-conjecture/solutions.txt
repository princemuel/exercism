section .text
global steps
; Function: steps(int number)
; Calculates the number of steps to reach 1 using the Collatz Conjecture
; Input: edi contains the input number (32-bit int)
; Output: eax contains the step count, or -1 for invalid input
; Registers used: eax (step counter), edi (current number), esi (temporary storage)

steps:
    ; Initialize step counter to 0
    ; Using XOR is faster than MOV eax, 0
    xor eax, eax

    ; Input validation and base case handling
    cmp edi, 1                  ; Compare input with 1
    jl .failed                  ; If input < 1 (zero or negative), return failed
    je .done                    ; If input == 1, return 0 steps (we're already done)

.loop:
    ; Increment step counter for each iteration
    inc eax

    ; Pre-calculate the "odd case" result (3n + 1)
    ; LEA is used as a fast arithmetic instruction: esi = edi * 3 + 1
    ; We calculate this speculatively before knowing if the number is odd
    lea esi, [rdi*3+1]

    ; Always perform the "even case" operation (divide by 2)
    ; SHR shifts bits right by 1, equivalent to dividing by 2
    ; CRUCIAL: If the original number was odd, the LSB (1) shifts into carry flag
    ; If the original number was even, the LSB (0) clears the carry flag
    shr edi, 1

    ; Conditional move based on carry flag (our even/odd detector)
    ; If carry flag is set (original number was odd), use the pre-calculated 3n+1 value
    ; If carry flag is clear (original number was even), keep the shifted (n/2) value
    ; This replaces a traditional if/else branch with a single conditional move
    cmovc edi, esi

    ; Check if we've reached 1 (the end condition)
    cmp edi, 1
    jne .loop                   ; If not 1 yet, continue looping

    ; Fall through to .done if we've reached 1

.done:
    ; Return with step count in eax
    ret

.failed:
    ; Return -1 for invalid input
    ; Since eax was initialized to 0, NOT flips all bits: 0 becomes 0xFFFFFFFF (-1)
    not eax
    ret

; Required for 64-bit ELF format to mark stack as non-executable
%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
