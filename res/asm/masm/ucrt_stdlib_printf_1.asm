; https://www.reddit.com/r/Assembly_language/comments/17vleha/printf_not_printing_masm_64_bit_windows_visual/
;
; ucrt is the universal C runtime library. It is a second C runtime library available on windows.
; See: https://stackoverflow.com/questions/45029909/c-standard-library-for-win32-win64-other-than-mscrt
; "These days there is also a choice between which Windows' underlying libraries to use:
; MSVCRT or UCRT. The former is the one used since the early days of win32, the latter is
; for more recent Windows versions
; (see https://learn.microsoft.com/en-us/cpp/porting/upgrade-your-code-to-the-universal-crt)."
;
; Compiling:
;
; Open x64 native tools command prompt
; or
; Open normal cmd prompt (not powershell) and execute:
; "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
;
; cd C:\Users\lapto\dev\rust\stm8_asm\res\asm\masm
;
; ml64.exe /nologo /c ucrt_stdlib_printf_1.asm
; link.exe /SUBSYSTEM:console /LARGEADDRESSAWARE:NO ucrt_stdlib_printf_1.obj
; ucrt_stdlib_printf_1.exe
;

includelib ucrt.lib
includelib legacy_stdio_definitions.lib

EXTERN printf: PROC

    .data
fmt_str byte 'test: %d', 13, 10, 0
;fmt_str db "hello world!", 13, 10, 0

    .code
mainCRTStartup PROC
    sub rsp, 56             ; shadow space, home space

    ;call _CRT_INIT

    ; https://stackoverflow.com/questions/79519237/windows-masm-x64-calling-convention-and-stack-setup
    ;
    ; * First 4 parameters passed in RCX, RDX, R8, R9
    ; * Following parameters are passed on stack
    ; * 32 byte shadow space must also be passed to stack
    ;   (This value needs to be increased for some function e.g. printf needs 56 bytes!?!)
    ; * Stack must be 16 byte aligned before "CALL fun" is executed so padding is needed in some scenarios
    ;
    ; Order:
    ; 1. Push padding if necessary.
    ; 2. parameter k, parameter k-1, ..., parameter5.
    ; 3. 32 byte shadow (16 byte aligned at this point).
    ; 4. CALL pushes 8 byte return address.
    lea rcx, fmt_str
    mov rdx, 100
    mov r8, 100
    mov r9, 100
    call printf

    add rsp, 56
mainCRTStartup ENDP

END