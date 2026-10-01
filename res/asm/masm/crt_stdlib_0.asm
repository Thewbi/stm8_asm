; https://stackoverflow.com/questions/76751044/using-printf-scanf-in-x64-masm
;
; Compiling:
;
; Open x64 native tools command prompt
;
; cd C:\Users\lapto\dev\rust\stm8_asm\res\asm\masm
;
; ml64.exe /c crt_stdlib_0.asm
;
; link.exe /SUBSYSTEM:console /LARGEADDRESSAWARE:NO crt_stdlib_0.obj
;

option casemap:none

includelib kernel32.lib
includelib user32.lib
includelib libcmt.lib
includelib legacy_stdio_definitions.lib

extern printf:proc
extern scanf:proc


; Section 11.9 Complete Program


        .data
msg1fmt   byte "%s",0
msg2fmt   byte 0Ah,"%s",0Ah,0Ah,0
msg3fmt   byte "   %lld", 0Ah,0Ah,0
in1fmt    byte "%lld",0
msg2      byte "Enter an integer: ",0
msg3      byte "Reversed",0
n         sqword 5
arry      sqword 5 dup(?)
        .code
main      proc
        mov rcx,n                     ; initialize rcx to n
        mov rbx,0                     ; initialize rbx to 0
for01:    nop
        push rcx                      ; save rcx
        lea rcx, msg1fmt
        lea rdx, msg2
        sub rsp, 40
        CALL printf
        add rsp, 40
        lea rcx, in1fmt
        lea rdx,arry[rbx]
        sub rsp,40
        call scanf
        add rsp,40
        pop rcx                       ; restore rcx
        add rbx,8                     ; increment rbx by 8
        loop for01
endfor01: nop

        lea rcx, msg2fmt
        lea rdx, msg3
        sub rsp, 40
        CALL printf
        add rsp, 40
        mov rcx,n                     ; initialize rcx to n
        sub rbx,8                     ; subtract 8 from rbx

for02:    nop
        push rcx                      ; save rcx
        lea rcx, msg3fmt
        mov rdx, arry[rbx]
        sub rsp, 40
        CALL printf
        add rsp, 40
        pop rcx                       ; restore rcx
        sub rbx,8                     ; decrement rbx by 8
        loop for02
endfor02: nop

        ret
main      endp
        end