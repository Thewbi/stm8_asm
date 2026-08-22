# x86-64 Registers in Windows 10

|  Register |  Hardware         | Software: x64 Calling Convention |
|:----------|:------------------|:---------------------------------|
|   RAX     |Default accumulator|Return value, volatile            |
|   _RBX_   |Index              |Nonvolatile                       |
|   RCX     |Loop counter       |1st integer argument, volatile, usually exit status code|
|   RDX     |                   |2nd integer argument, volatile    |
|   _RSI_   |Source index       |Nonvolatile                       |
|   _RDI_   |Destination index  |Nonvolatile                       |
|   _RBP_   |Base pointer       |Nonvolatile                       |
|   _RSP_   |Stack pointer      |Nonvolatile                       |
|   R8, R9  |                   |3rd/4th integer argument, volatile|
|   R10, R11|                   |Volatile                          |
|  _R12-R15_|                   |Nonvolatile                       |




## Show Memory

Debuggen > Fenster > Arbeitsspeicher > Arbeitsspeicher 1 (Strg + Alt + M, 1)




# x64 Assembly


## .code vs. SEGMENT text

In Microsoft Macro Assembler (MASM), the core difference is that .code is a simplified segment directive that automatically manages segment properties, while SEGMENT is a full-segment directive that requires you to manually define all segment attributes, alignments, and pairings

## extrn ExitProcess: PROC vs. EXTERN GetForegroundWindow: PROC vs. ExitProcess PROTO vs externdef

https://stackoverflow.com/questions/74738084/when-to-use-externdef-with-abs-in-masm

```
extrn ExitProcess: PROC
EXTERN ExitProcess: PROC
ExitProcess PROTO, dwExitCode:DWORD
externdef symbol:type {optional_list_of_symbol:type_pairs}
```

How should you include windows functions into assembly code?
Should you use extrn+PROC or EXTERN+PROC or PROTO or externdef?
What is the difference?

The Art of 64-Bit Assembly (Randall Hyde) page 24: "MASM has two other directives (next to externdef) extrn and extern, that could also be used. This book uses the externdef directive because it is the most general directive."

I tested these two options:

```
ExitProcess PROTO
EXTERN ExitProcess: PROC
```

both worked.

## Specify the entry point after the END keyword or not? END vs. END main

Is it true that the main entry point to the application can be inserted after the END keyword?
This seems to be optional.

```
END
```

vs.

```
END main
```



### Write "Hello World" to the console (x64)

This example uses .code (instead of SEGMENT).
This code calls ExitProcess without crashing.

```
; ---------------------------------------------
; Hello World for Win64 Intel x64 Assembly
;
; by fruel (https://github.com/fruel)
; 13 June 2016
; ---------------------------------------------

GetStdHandle PROTO
ExitProcess PROTO
WriteConsoleA PROTO

.data
msg BYTE "Hello World",0
bytesWritten DWORD ?

.code
main PROC
    sub rsp, 5 * 8

    mov rcx, -11
    call GetStdHandle

    mov  rcx, rax
    lea  rdx, msg
    mov  r8, LENGTHOF msg - 1
    lea  r9, bytesWritten  
    mov  QWORD PTR [rsp + 4 * SIZEOF QWORD], 0
    call WriteConsoleA

    mov rcx, 0      
    call ExitProcess
main ENDP

END
```

### ExitProcess (x64)

Uses segments.
Just calls ExitProcess()

```
ExitProcess PROTO
;EXTERN ExitProcess: PROC

PUBLIC main

_TEXT SEGMENT

main PROC

    ; prelude - build stack frame
	push rbp ; save frame pointer
	mov rbp, rsp ; fix stack pointer
	sub rsp, 8 * (4 + 2) ; allocate shadow register area + 2 QWORDs for stack alignment
	
    ; call to ExitProcess
	mov eax, 0  
	call ExitProcess

	; epilog - restore stack pointer
	mov rsp, rbp
	pop rbp

	ret
main ENDP

_TEXT ENDS

END
```

### MessageBox (x64)

Using SEGMENT instead of .text
Calls ExitProcess before removing the stack pointer.

```
GetForegroundWindow PROTO
;EXTERN GetForegroundWindow: PROC

MessageBoxA PROTO
;EXTERN MessageBoxA: PROC

ExitProcess PROTO
;EXTERN ExitProcess: PROC

PUBLIC main


_DATA SEGMENT
hello_msg db "Hello world", 0
info_msg  db "Info", 0
_DATA ENDS


_TEXT SEGMENT

main PROC

	push rbp ; save frame pointer
	mov rbp, rsp ; fix stack pointer
	sub rsp, 8 * (4 + 2) ; allocate shadow register area + 2 QWORDs for stack alignment

	; Get a window handle
	call GetForegroundWindow
	mov rcx, rax

	; WINUSERAPI int WINAPI MessageBoxA(
	;  RCX =>  _In_opt_ HWND hWnd,
	;  RDX =>  _In_opt_ LPCSTR lpText,
	;  R8  =>  _In_opt_ LPCSTR lpCaption,
	;  R9  =>  _In_ UINT uType);

	mov rdx, offset hello_msg
	mov r8, offset info_msg
	mov r9, 0 ; MB_OK

	and rsp, not 8 ; align stack to 16 bytes prior to API call
	call MessageBoxA

    ; Exit Process
	mov eax, 0  
	call ExitProcess

	; epilog. restore stack pointer
	mov rsp, rbp
	pop rbp	

	ret
main ENDP

_TEXT ENDS

END
```






# x32 Assembly





## Console Output on Windows with MASM (32 bit)

https://en.wikibooks.org/wiki/X86_Assembly/Print_Version

```
	.386
	.MODEL flat, stdcall
STD_OUTPUT_HANDLE EQU -11 
GetStdHandle PROTO, nStdHandle: DWORD 
WriteConsoleA PROTO, handle: DWORD, lpBuffer:PTR BYTE, nNumberOfBytesToWrite:DWORD, lpNumberOfBytesWritten:PTR DWORD, lpReserved:DWORD
ExitProcess PROTO, dwExitCode: DWORD 

.data
consoleOutHandle dd ? 
bytesWritten dd ? 
message db "Hello World",13,10
lmessage dd 13

.code
main PROC
	INVOKE GetStdHandle, STD_OUTPUT_HANDLE
	mov consoleOutHandle, eax 
	mov edx, offset message 
	pushad    
	mov eax, lmessage
	INVOKE WriteConsoleA, consoleOutHandle, edx, eax, offset bytesWritten, 0
	popad
	INVOKE ExitProcess, 0 
main ENDP

END main
```



# Iterate over byte buffer

https://stackoverflow.com/questions/7592115/iterate-through-memory-editing-each-byte

```
    mov cl, 0           ; cl is the counter register, set it to
                        ; zero (the first character in the string)

start:                  ; Beginning of loop
    mov al, bytes[cl]   ; Read the next byte from memory

    cmp al, 0           ; Compare the byte to null (the terminator)
    je end              ; If the byte is null, jump out of the loop

    sub al, 20h         ; Convert to upper case
                        ; A better solution would be: and al, 0DFh

    ; Output the character in al

    add cl, 1           ; Move to the next byte in the string
    jmp start           ; Loop
end:
```


## Replace String with upper case A characters

```
	.386
	.MODEL flat, stdcall
STD_OUTPUT_HANDLE EQU -11

GetStdHandle PROTO, nStdHandle: DWORD 
WriteConsoleA PROTO, handle: DWORD, lpBuffer:PTR BYTE, nNumberOfBytesToWrite:DWORD, lpNumberOfBytesWritten:PTR DWORD, lpReserved:DWORD
ExitProcess PROTO, dwExitCode: DWORD 

.data
consoleOutHandle dd ? 
bytesWritten dd ? 
message db "Hello World",13,10
lmessage dd 13

.code
main PROC
	mov edx, offset message			; data goes into the data register EDX
	mov ecx, 0						; counter goes into counter register ECX
lc:
	mov ebx, 65
	mov BYTE PTR [edx+ecx], bl

	mov ebx, 1
	add ecx, ebx
	cmp ecx, lmessage
	je print
	jmp lc
	
print:
	INVOKE GetStdHandle, STD_OUTPUT_HANDLE
	mov consoleOutHandle, eax

	mov edx, offset message 
	pushad    
	mov eax, lmessage
	INVOKE WriteConsoleA, consoleOutHandle, edx, eax, offset bytesWritten, 0
	popad
	
	INVOKE ExitProcess, 0 
main ENDP

END main
```

##

```
	.386
	.MODEL flat, stdcall
STD_OUTPUT_HANDLE EQU -11

GetStdHandle PROTO, nStdHandle: DWORD 
WriteConsoleA PROTO, handle: DWORD, lpBuffer:PTR BYTE, nNumberOfBytesToWrite:DWORD, lpNumberOfBytesWritten:PTR DWORD, lpReserved:DWORD
ExitProcess PROTO, dwExitCode: DWORD 

.data
consoleOutHandle dd ? 
bytesWritten dd ? 

;message db "Hello World",13,10
;lmessage dd 13

;message db "helloworld",13,10
message db "HELLOWORLD",13,10
lmessage dd 12
lmessage_text dd 10


.code
main PROC
	mov edx, offset message			; data goes into the data register EDX
	mov ecx, 0						; counter goes into counter register ECX
lc:
	;mov ebx, 65
	;mov BYTE PTR [edx+ecx], bl

	mov bl, BYTE PTR [edx+ecx]
	;sub bl, 20h						; to uppercase
	add bl, 20h						; to lowercase
	mov BYTE PTR [edx+ecx], bl

	mov ebx, 1
	add ecx, ebx
	cmp ecx, lmessage_text
	je print
	jmp lc
	
print:
	INVOKE GetStdHandle, STD_OUTPUT_HANDLE
	mov consoleOutHandle, eax

	mov edx, offset message 
	pushad    
	mov eax, lmessage
	INVOKE WriteConsoleA, consoleOutHandle, edx, eax, offset bytesWritten, 0
	popad
	
	INVOKE ExitProcess, 0 
main ENDP

END main
```