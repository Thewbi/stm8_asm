    .386
    .model flat, stdcall
    .stack 4096

ExitProcess PROTO, dwExitCode:DWORD

    .code
main PROC
    push ebp ; save base of current stack frame to restore it later
    mov ebp, esp ; set new base of new stack frame (to current stack pointer)
    sub esp, 4 ; save space on stack for all local variables
    mov dword ptr [ebp-4], 10
    sub dword ptr [ebp-4], 3
    mov eax, dword ptr [ebp-4]
    add esp, 4 ; restore stack pointer to old stack frame top
    pop ebp ; restore old base pointer of old stack frame
    ret ; pops the return address from the top of the stack into the instruction pointer (EIP/RIP)
    INVOKE ExitProcess, eax
main ENDP

END main ; specify the program's entry point
