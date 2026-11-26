
    .data


    .code
extern CreateFileA: proc
extern OpenFile: proc
extern WriteFile: proc
extern ReadFile: proc
extern ExitProcess: proc
extern putchar: proc
extern getchar: proc
extern fopen: proc
extern malloc: proc

print_unsigned_char_numeric_decimal PROC

    ; prelude - create stack frame and shadow area
    push rbp                            ; save base of current stack frame to restore it later
    mov rbp, rsp                        ; set new base of new stack frame (to current stack pointer)

    ; Function Preamble - save space on stack for all local variables - [AsmAstConversionVisitor::visit_tacky_function()]
    ; Updated in AsmAstFixupVisitor::visit_asm_ast_function()
    sub rsp, 256




    ;; parameter: unsigned char data
    mov byte ptr [rbp-1], cl            ; asm_ast_instruction.id: '2'    ; INSIDE_FUNCTION. Moving Reg-To-MEM: Parameter 0) Argument { name: "userdef_var.2", data_type: DataTypeUnsignedChar } is located in register CX

    ;; int text_len = 0;
    mov dword ptr [rbp-5], 0            ; asm_ast_instruction.id: '3'

while_95_continue_label:

    ;mov r12b, byte ptr [rbp-1]         ; DEBUG
    ;mov r12d, dword ptr [rbp-5]        ; DEBUG

    ;; while (data > 0) {
    mov dword ptr [rbp-9], 0            ; asm_ast_instruction.id: '6'    ; binary_relational (3) - initialize temporary with zero
    cmp byte ptr [rbp-1], 0             ; binary_relational (2), ZR flag needs to be 0
    ;setg byte ptr [rbp-9]              ; set if greater (ONLY SIGNED!!!!!!)
    seta byte ptr [rbp-9]
    ;mov r12b, byte ptr [rbp-9]          ; DEBUG
    cmp dword ptr [rbp-9], 0            ; JumpIfZero, ZR flag needs to be 1
    ;mov r12d, dword ptr [rbp-9]         ; DEBUG
    je while_95_end_label

    ;; buffer[text_len] = data % 10;
    xor rax, rax
    mov al, byte ptr [rbp-1]            ; asm_ast_instruction.id: '10'
    cdq
    mov ebx, 10                         ; asm_ast_instruction.id: '13'
    xor edx, edx
    div ebx                             ; dividend in EDX:EAX
    mov dword ptr [rbp-13], edx         ; remainder
    mov eax, dword ptr [rbp-5]          ; asm_ast_instruction.id: '15'    ; [visit_tacky_subscript_dst() - Subscript 1] Mov Index/Offset into RAX
    mov ecx, dword ptr [rbp-13]         ; asm_ast_instruction.id: '17'    ; [visit_tacky_subscript_dst() - Subscript 4] mov src variable's value into dst!
    mov byte ptr [rbp-16+rax*1], cl     ; asm_ast_instruction.id: '16'    ; [visit_tacky_subscript_dst() - Subscript 2] for subscript (applying an index to an array)

    ;; text_len = text_len + 1;
    ;mov r12d, dword ptr [rbp-5]         ; DEBUG
    add dword ptr [rbp-5], 1
    ;mov r12d, dword ptr [rbp-5]         ; DEBUG

    ;; data = data / 10;
    xor rax, rax
    mov al, byte ptr [rbp-1]            ; asm_ast_instruction.id: '19'
    cdq
    mov ebx, 10                         ; asm_ast_instruction.id: '22'
    xor edx, edx
    div ebx
    ;mov dword ptr [rbp-1], eax          ; quotient
    mov byte ptr [rbp-1], al          ; quotient

    ;; if (data == 0) { break; }
    ;mov r12b, byte ptr [rbp-1]          ; DEBUG
    cmp byte ptr [rbp-1], 0             ; binary_relational (2)
    mov dword ptr [rbp-20], 0           ; asm_ast_instruction.id: '25'    ; binary_relational (3) - initialize temporary with zero
    sete byte ptr [rbp-20]              ; set if equal (does work for both signed and unsigned)
    cmp dword ptr [rbp-20], 0           ; JumpIfZero
    je if_89_end_label
    jmp while_95_end_label

if_89_end_label:
    jmp while_95_continue_label

while_95_end_label:
    ;; text_len = text_len - 1;
    ;mov r12d, dword ptr [rbp-5]         ; DEBUG
    sub dword ptr [rbp-5], 1
    ;mov r12d, dword ptr [rbp-5]         ; DEBUG

    ;; int i = 123;
    ;mov dword ptr [rbp-24], 123         ; DEBUG

    ;; for (i = text_len;
    mov r10d, dword ptr [rbp-5]         ; asm_ast_instruction.id: '35' ; Added by FixupVistor - clone 1
    mov dword ptr [rbp-24], r10d       ; asm_ast_instruction.id: '35'

for_160_start_label:
    cmp dword ptr [rbp-24], 0          ; binary_relational (2)

    mov dword ptr [rbp-28], 0           ; place comparison result into [rbp-28]
    setge byte ptr [rbp-28]

    cmp dword ptr [rbp-28], 0           ; JumpIfZero

    je for_160_end_label
    xor rax, rax
    mov eax, dword ptr [rbp-24]        ; asm_ast_instruction.id: '43'    ; [visit_tacky_subscript_src() - Subscript 1] Mov Index/Offset into RAX
    mov al, byte ptr [rbp-16+rax*1]     ; asm_ast_instruction.id: '44'    ; [visit_tacky_subscript_src() - Subscript 2] for subscript (applying an index to an array)
    mov byte ptr [rbp-32], al           ; asm_ast_instruction.id: '45'    ; [visit_tacky_subscript_src() - Subscript 3] mov_2
    mov r10d, dword ptr [rbp-32]       ; asm_ast_instruction.id: '47'	; case 1 - (both srces are not destination) move operand src into dst ; Added by FixupVistor - clone 1
    mov dword ptr [rbp-36], r10d       ; asm_ast_instruction.id: '47'	; case 1 - (both srces are not destination) move operand src into dst
    add dword ptr [rbp-36], 48
    mov ecx, dword ptr [rbp-36]        ; asm_ast_instruction.id: '48'    ; [TARGET:REGISTER] push/mov fun_call argument 0
    call putchar
for_160_continue_label:
    mov r10d, dword ptr [rbp-24]       ; asm_ast_instruction.id: '51' ; Added by FixupVistor - clone 1
    mov dword ptr [rbp-24], r10d       ; asm_ast_instruction.id: '51'
    mov r10d, dword ptr [rbp-24]       ; asm_ast_instruction.id: '52' ; FIXUP Visitor
    dec dword ptr [rbp-24]
    jmp for_160_start_label
for_160_end_label:

    ; epilog - restore stack pointer
    mov rsp, rbp                        ; stack pointer now points to the pushed rbp from the beginning of the call
    pop rbp

    ; inverse to call - pops the return address from the top of the stack into the instruction pointer (EIP/RIP)
    ret

print_unsigned_char_numeric_decimal ENDP






main PROC

    ; prelude - create stack frame and shadow area
    push rbp                            ; save base of old stack frame to restore it later
    mov rbp, rsp                        ; set new base of new stack frame into rbp

    ; Function Preamble - save space on stack for all local variables - [AsmAstConversionVisitor::visit_tacky_function()]
    ; Updated in AsmAstFixupVisitor::visit_asm_ast_function()
    sub rsp, 72




    ;; unsigned char a = 213;
    mov byte ptr [rbp-1], 234           ; asm_ast_instruction.id: '57'

    xor rcx, rcx
    mov cl, byte ptr [rbp+-1]

    ;; mov ecx, dword ptr [rbp+-1]       ; asm_ast_instruction.id: '58'    ; [TARGET:REGISTER] push/mov fun_call argument 0

    call print_unsigned_char_numeric_decimal

    ;call getchar

    ;mov ecx, 0                          ; asm_ast_instruction.id: '60'    ; [TARGET:REGISTER] push/mov fun_call argument 0
    ;call ExitProcess

    mov eax, 0                          ; asm_ast_instruction.id: '62'    ; Generated by the Return keyword in TackyToIntermediateAsmConversionVisitor

    ; epilog - restore stack pointer
    mov rsp, rbp                        ; stack pointer now points to the pushed rbp from the beginning of the call
    pop rbp

    ; pops the return address from the top of the stack into the instruction pointer (EIP/RIP)
    ret

main ENDP

END
