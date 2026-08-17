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



## Console Output on Windows with MASM

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