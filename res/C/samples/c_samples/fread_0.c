// res\C\samples\c_samples\fread_0.c

int CreateFileA(char *lpFileName,
    int dwDesiredAccess,
    int dwShareMode,
    int lpSecurityAttributes,
    int dwCreationDisposition,
    int dwFlagsAndAttributes,
    int hTemplateFile);

//int OpenFile(char *lpFileName,
//)

int WriteFile(
    int hFile,
    char *lpBuffer,
    int nNumberOfBytesToWrite,
    long lpNumberOfBytesWritten,
    long lpOverlapped);

int ReadFile(
    int hFile,
    char *lpBuffer,
    int nNumberOfBytesToRead,
    long lpNumberOfBytesRead,
    long lpOverlapped);

int main() {

/*
    // https://learn.microsoft.com/de-de/windows/win32/api/fileapi/nf-fileapi-createfilea
    //
    // HANDLE CreateFileA(
    //  [in]           LPCSTR                lpFileName,
    //  [in]           DWORD                 dwDesiredAccess,
    //  [in]           DWORD                 dwShareMode,
    //  [in, optional] LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    //  [in]           DWORD                 dwCreationDisposition,
    //  [in]           DWORD                 dwFlagsAndAttributes,
    //  [in, optional] HANDLE                hTemplateFile
    // );
    //
    // Return Value:
    // The constant INVALID_HANDLE_VALUE is 0xFFFFFFFFFFFFFFFF. It is returned
    // if the operation failed. The reason can be retrieved using GetLastError().

    int file_handle = CreateFileA(
        //"C:\\Users\\lapto\\source\\repos\\x64_test\\x64\\Debug\\test.txt", // lpFileName
        //"C:\\Users\\lapto\\temp\\fopen_2_test.txt",
        "C:\\users\\lapto\\Downloads\\bgbw64",
        0x10000000, // dwDesiredAccess
        0,          // dwShareMode
        0,          // lpSecurityAttributes
        0,          // dwCreationDisposition
        0x80,       // dwFlagsAndAttributes
        4           // hTemplateFile
    );
*/

    int file_handle = CreateFileA(
        //"C:\\Users\\lapto\\source\\repos\\x64_test\\x64\\Debug\\test.txt", // lpFileName
        //"C:\\Users\\lapto\\temp\\fopen_2_test.txt",
        "C:\\users\\lapto\\Downloads\\bgbw64\\bgbtest.gb",
        0x80000000, // dwDesiredAccess (GENERIC_READ: 0x80000000)
        1,          // dwShareMode (FILE_SHARE_READ: 0x00000001)
        0,          // lpSecurityAttributes
        3,          // dwCreationDisposition (OPEN_EXISTING: 3)
        0x80,       // dwFlagsAndAttributes (FILE_ATTRIBUTE_READONLY: 1, FILE_ATTRIBUTE_NORMAL: 0x80)
        3           // hTemplateFile
    );


/*
    // https://learn.microsoft.com/de-de/windows/win32/api/fileapi/nf-fileapi-writefile
    //
    // BOOL WriteFile(
    //  [in]                    HANDLE              hFile,
    //  [in]                    LPCVOID             lpBuffer,
    //  [in]                    DWORD               nNumberOfBytesToWrite,
    //  [out, optional]         LPWORD              lpNumberOfBytesWritten,
    //  [in, out, optional]     DWORD               lpOverlapped,
    // );

    WriteFile(file_handle,
        "test",
        4,
        0,
        0
    );
*/

    // https://learn.microsoft.com/de-de/windows/win32/api/fileapi/nf-fileapi-readfile
    //
    // BOOL ReadFile(
    //  [in]                    HANDLE              hFile,
    //  [out]                   LPVOID              lpBuffer,
    //  [in]                    DWORD               nNumberOfBytesToRead,
    //  [out, optional]         LPWORD              lpNumberOfBytesRead,
    //  [in, out, optional]     DWORD               lpOverlapped,
    // );

    char buffer[100];

    ReadFile(file_handle,
        buffer,
        100,
        0,
        0
    );

    //return buffer[0x38];
    return buffer[56];
}





/*

    .data
_1180898821999259727 DB 'C:\\users\\lapto\\Downloads\\bgbw64\\bgbtest.gb', 00h



    .code
extern CreateFileA: proc
extern OpenFile: proc
extern WriteFile: proc
extern ReadFile: proc
extern ExitProcess: proc
extern putchar: proc
extern fopen: proc

main PROC


    ; prelude - create stack frame and shadow area
    push rbp ; save base of current stack frame to restore it later
    mov rbp, rsp ; set new base of new stack frame (to current stack pointer)

    ; Function Preamble - save space on stack for all local variables - [AsmAstConversionVisitor::visit_tacky_function()]    ; + Updated in AsmAstFixupVisitor::visit_asm_ast_function()

    sub rsp, 32+128 ; has to be 16-byte aligned
    lea rbx,  [_1180898821999259727]     ;     ; lea for GetAddress()
    mov qword ptr [rbp+-8], rbx    ; asm_ast_instruction.id: '6'; lea cannot load into mem, so it loads into reg. This move loads reg into dest mem

    ; FunctionCall - Allocate space for parameters that need to go onto the stack according to the ABI - [AsmAstConversionVisitor::visit_tacky_fun_call_system_v_abi()]
    sub rsp, 48
    mov ecx, dword ptr [rbp+-8]    ; asm_ast_instruction.id: '8'    ; [TARGET:REGISTER] push/mov fun_call argument 0
    mov edx, 2147483648    ; asm_ast_instruction.id: '9'    ; [TARGET:REGISTER] push/mov fun_call argument 1
    mov r8d, 1    ; asm_ast_instruction.id: '10'    ; [TARGET:REGISTER] push/mov fun_call argument 2
    mov r9d, 0    ; asm_ast_instruction.id: '11'    ; [TARGET:REGISTER] push/mov fun_call argument 3
    mov dword ptr [rsp+40], 3    ; asm_ast_instruction.id: '12'    ; [TARGET:STACK] push/mov fun_call argument 4
    mov dword ptr [rsp+36], 128    ; asm_ast_instruction.id: '13'    ; [TARGET:STACK] push/mov fun_call argument 5
    mov dword ptr [rsp+32], 3    ; asm_ast_instruction.id: '14'    ; [TARGET:STACK] push/mov fun_call argument 6
    call CreateFileA
    add rsp, 48  ; remove stack frame

    mov dword ptr [rbp-12], eax    ; asm_ast_instruction.id: '17'    ; mov EAX into TACKY dst variable Variable(userdef_var.17)

    ;lea rbx,  [_7397688135890305232]     ;     ; lea for GetAddress()
    ;mov qword ptr [rbp+-20], rbx    ; asm_ast_instruction.id: '22'; lea cannot load into mem, so it loads into reg. This move loads reg into dest mem

    ; FunctionCall - Allocate space for parameters that need to go onto the stack according to the ABI - [AsmAstConversionVisitor::visit_tacky_fun_call_system_v_abi()]
    sub rsp, 48
    xor rcx, rcx
    mov ecx, eax    ; asm_ast_instruction.id: '24'    ; [TARGET:REGISTER] push/mov fun_call argument 0
    xor rdx, rdx
    ; load effective address of byte buffer into rdx!
    lea rdx, [rbp-112]    ; asm_ast_instruction.id: '25'    ; [TARGET:REGISTER] push/mov fun_call argument 1
    mov r8, 100    ; asm_ast_instruction.id: '26'    ; [TARGET:REGISTER] push/mov fun_call argument 2
    mov r9, 0    ; asm_ast_instruction.id: '27'    ; [TARGET:REGISTER] push/mov fun_call argument 3
    mov qword ptr [rsp+32], 0    ; asm_ast_instruction.id: '28'    ; [TARGET:STACK] push/mov fun_call argument 4
    call ReadFile
    add rsp, 48  ; remove stack frame

    xor rax, rax
    mov eax, 56    ; asm_ast_instruction.id: '33'    ; [visit_tacky_subscript_src() - Subscript 1] Mov Index/Offset into RAX
    mov al, byte ptr [rbp-112+rax*1]    ; asm_ast_instruction.id: '34'    ; [visit_tacky_subscript_src() - Subscript 2] for subscript (applying an index to an array)
    mov byte ptr [rbp-124], al    ; asm_ast_instruction.id: '35'    ; [visit_tacky_subscript_src() - Subscript 3] mov_2
    mov eax, dword ptr [rbp+-124]    ; asm_ast_instruction.id: '36'    ; Generated by the Return keyword in TackyToIntermediateAsmConversionVisitor

    ; epilog - restore stack pointer
    mov rsp, rbp
    pop rbp

    ; pops the return address from the top of the stack into the instruction pointer (EIP/RIP)
    ret
main ENDP

END


*/