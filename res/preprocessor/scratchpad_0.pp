// res\C\samples\c_samples\fopen_1.c

int main() {

__asm {

    ; https://learn.microsoft.com/de-de/windows/win32/api/fileapi/nf-fileapi-createfilea
    ;
    ; HANDLE CreateFileA(
    ;  [in]           LPCSTR                lpFileName,
    ;  [in]           DWORD                 dwDesiredAccess,
    ;  [in]           DWORD                 dwShareMode,
    ;  [in, optional] LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    ;  [in]           DWORD                 dwCreationDisposition,
    ;  [in]           DWORD                 dwFlagsAndAttributes,
    ;  [in, optional] HANDLE                hTemplateFile
    ; );

    ; https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention?view=msvc-170
    ;
    ; By default, the x64 calling convention passes the first four arguments
    ; to a function in registers.
    ; The registers used for these arguments depend on the position and type of the argument.
    ; - Integer arguments are passed in registers RCX, RDX, R8, and R9.
    ; - Floating point arguments are passed in XMM0L, XMM1L, XMM2L, and XMM3L.
    ; Remaining arguments are passed on the stack in right-to-left order.
    ; Any parameters beyond the first four must be stored on the stack after
    ; the shadow store before the call.

    ; shadow space
    sub rsp, 8 * 32     ; 32 byte

    ; reserve stack space for the parameters to CreateFileA
    sub rsp, 8 * 4

    ; parameter 1 - filename
    mov rcx, OFFSET filename

    ; parameter 2 - dwDesiredAccess
    ;mov rdx, 2                  ; read and write rights. See https://learn.microsoft.com/de-de/windows/win32/fileio/file-access-rights-constants
    mov rdx, 10000000h

    ; parameter 3 - dwShareMode
    mov r8, 0

    ; parameter 4 - lpSecurityAttributes (0x00 - may be null. If it is null,
    ; file handled cannot be passed to sub-processes!)
    mov r9, 0

    ;push 1
    ;push 80h
    ;push 0
    ;mov QWORD PTR [rsp + 20h], 4           ; push fifth argument dwCreationDisposition
    ;mov QWORD PTR [rsp + 28h], 80h         ; push sixth argument dwFlagsAndAttributes
    ;mov QWORD PTR [rsp + 30h], 0           ; push seventh argument hTemplateFile

    mov QWORD PTR [rsp + 20h + 18h], 00h    ; temp 0 (maybe for propert stack alignment ???)
    mov QWORD PTR [rsp + 20h + 10h], 00h    ; push seventh argument hTemplateFile (may be 0. 0x00)
    mov QWORD PTR [rsp + 20h + 08h], 80h    ; push sixth argument dwFlagsAndAttributes (0x80 == FILE_ATTRIBUTE_NORMAL)
    mov QWORD PTR [rsp + 20h + 00h], 04h    ; push fifth argument dwCreationDisposition (4 == OPEN_ALWAYS)

    call CreateFileA

    ; Rueckgabewert (befindet sich in rax)
    ; Wenn die Funktion erfolgreich ist, ist der Rueckgabewert ein geoeffnetes
    ; Handle fuer die angegebene Datei, das Geraet, das benannte Pipe oder den E-Mail-Steckplatz.
    ; Wenn die Funktion fehlschlaegt, ist der Rueckgabewert INVALID_HANDLE_VALUE.
    ; Rufen Sie GetLastErrorauf, um erweiterte Fehlerinformationen zu erhalten.

    ;pop rdx
    ;pop rdx
    ;pop rdx

    ; remove stack space for the parameters to CreateFileA
    add rsp, 8 * 4

    ; shadow space
    add rsp, 8 * 32

}

__asm {

    ;
    ; ExitProcess
    ;

    ; shadow space
    sub rsp, 8 * 32

    mov rax, 0h
    call ExitProcess

    ; remove shadow space
    add rsp, 8 * 32

}

    return 0;

}