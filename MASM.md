# Visual Studio

Create a MASM project: See [VISUAL_STUDIO_AND_MASM](VISUAL_STUDIO_AND_MASM.md)




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



| 64-Bit | 32-Bit | 16-bit | 8bit/8Bit | Description                            |
| ------ | ------ | ------ | --------- | -------------------------------------- |
| RAX    | EAX    | AX     | AH,  AL   | Accumulator                            |
| RBX    | EBX    | BX     | BH,  BL   | Base                                   |
| RCX    | ECX    | CX     | CH,  CL   | Counter                                |
| RDX    | EDX    | DX     | DH,  DL   | Data (commonly extends the A register) |
| RSP    | ESP    | SP     | N/A, SPL  | Stack Pointer                          |
| RBP    | EBP    | BP     | N/A, BPL  | Stack Base Pointer                     |
| RSI    | ESI    | SI     | N/A, SIL  |                                        |
| RDI    | EDI    | DI     | N/A, DIL  |                                        |
| R8     | R8D    | R8W    | N/A, R8B  |                                        |
| R9     | R9D    | R9W    | N/A, R9B  |                                        |
| R10    | R10D   | R10W   | N/A, R10B |                                        |
| R11    | R11D   | R11W   | N/A, R11B |                                        |
| R12    | R12D   | R12W   | N/A, R12B |                                        |
| R13    | R13D   | R13W   | N/A, R13B |                                        |
| R14    | R14D   | R14W   | N/A, R14B |                                        |
| R15    | R15D   | R15W   | N/A, R15B |                                        |

## Show Memory

Debuggen > Fenster > Arbeitsspeicher > Arbeitsspeicher 1 (Strg + Alt + M, 1)



# Building

; Naming confusion: https://stackoverflow.com/questions/28414915/what-is-the-difference-between-masm-exe-and-ml-exe

; Open x64 native tools command prompt
;
; cd C:\Users\lapto\dev\masm\helloworld
;
; /link passes “the remainder of the command line to LINK”. That is, anything after /link gets passed as options to the linker.
;
; del main.obj main.exe *.lnk *.ilk *.pdb
; ml64.exe main.asm /link /DEBUG /subsystem:console /entry:main
;
; Two steps:
; del main.obj main.exe *.lnk
; ml64 /c main.asm
; link main.obj kernel32.lib /subsystem:console /entry:main /DEBUG

; Automatically tell the linker to look inside kernel32.lib
includelib kernel32.lib





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

Somehow END is used to mark the end of the file. This is because, when removing END from a file, then the assembler outputs this error message: error A2088: END directive required at end of file
The assembler just wants the end of the file (EOF) to be marked! The label might be purely optional.

```
END
```

vs.

```
END main
```



# Splitting a project into several files

In Visual Studio, the main.asm should include a .inc file.
The .inc file lists function prototypes from other .asm files (other than main.asm)
Visual Studio will feed all .asm files to the MASM assembler to produce object files and it will automatically specify those object files when linking the executable.

This means, create a .inc file (here: external.inc) with the following content:
```
itoa PROTO
```
then include the .inc file in main.asm at the very top of the file before other statements.
```
INCLUDE external.inc
```
This will pull the itoa function into main.asm.
The itoa function can be defined in some .asm file. Which .asm it is does not really matter as long as that .asm file is added to the Visual Studio project.

As an example lets assume that itoa is defined in the itoa.asm file.

```
PUBLIC itoa

.code

itoa PROC
    ... code goes here

    RET ; end of itoa
itoa ENDP

END ; END OF FILE (EOF)
```

itoa needs to be public so that the assembler exports that public symbol and the linker can find it in the .obj file. As MASM makes functions public by default, the very first line (PUBLIC itoa) is optional and almost a little bit confusing. Maybe remove it you want. No PRIVATE keyword exists, so I do not now how to prevent a function from being exported.


# Working with Arrays

Arrays can be defined in the .data section:

```
.data
    array DWORD 17,2,11,3,4,99,27
```

It is possible to store the address of the array into a register using the OFFSET keyword.

```
mov edx, OFFSET array           ; base address
```

edx now contains the array's base address which is the address of the first element.
The other elements are aligned in memory consecutively. This is the promise that the assembler makes
for arrays.

Instead of the OFFSET keyword, it is also possible to use the LEA (load effective address) instruction
to transfer the address of the array into a register. Using OFFSET might make your source code less
portable as the OFFSET keyword may not be available in other assemblers.

```
lea edx, array
```

To access the elements, indirect addressing with offset can be used.

```
mov eax, [edx + esi * 4]
```

This mov instruction copies the value stored at index esi from the array into the eax register.
It is not actually a mov but a copy meaning that the original value in the array is not erased but copied.
esi needs to contain the index. e.g. when esi has the value 3, then the fourth element is copied.
The literal 4 is used because each element of the array has 4 byte as the array was declared with DWORD
type and DWORD is a double word which is 4 byte in size.

To iterate over the array, initialize esi with zero for example and then loop incrementing esi:

```
    mov edx, OFFSET array               ; base address

    mov r12, 0                          ; reset loop counter variable (xor r12, r12 also sets zero)
    mov esi, 0                          ; reset array index to zero (xor esi, esi also sets zero)

print_array_loop_label:

    ... loop body ...
    mov eax, [edx + esi * 4]            ; now the array element's value is available in EAX
    ; move eax, DWORD PTR [edx + esi * 4] ; specifying DWORD PTR seems to be optional when the assembler can deduce the datatype itself
    ... loop body ...

    inc esi                             ; increment array index

    inc r12                             ; increment loop counter
    cmp r12, 7                          ; array has 7 elements
    jne print_array_loop_label
```

The loop counter variable r12 is used to abort the loop and prevent infinite loops.
esi is used as the array indexer and is incremented as the loop iterates.

See (https://stackoverflow.com/questions/37753811/when-can-i-address-variables-without-using-the-ptr-keyword)
Do you need to use DWORD PTR [...] or is it ok to just use [...]?
The DWORD PTR part is similar to a cast in C.
This part is optional, if the assembler can infer the data type itself when the type
is unambiguous.

## Length of an Array

Instead of counting the elements of an array manually, MASM provides the LENGTHOF operator.

```
array DWORD 4, 7, 4, 4, 8, 5, 4, 8, 4, 6, 6, 7, 4

mov ecx, LENGTHOF array
```

It works unless your array is split accross several lines which is where LENGTHOF only processes
the first line of the array definition and produces an incorrect result.

The solution in the multiline-case is to make use of the $ operator and a computation
on the first line after the array definition!

```
; https://stackoverflow.com/questions/65174740/masm-lengthof-array
array            DWORD 4, 7, 4, 4, 8, 5, 4, 8, 4, 6, 6, 7, 4
                DWORD 7, 5, 3, 4, 4, 6, 5, 4, 6, 5, 8, 7, 4, 5, 7
                DWORD 6, 5, 7, 8, 7, 6, 5, 4, 5, 7, 5, 3, 5, 3, 6
                DWORD 9, 6, 4, 7, 5, 4, 9
array_len = ($ - array) / TYPE array
```

As the Assembler counts the bytes in memory using this technique, the result is independent
of the lines in the source file.

## Amount of bytes in an Array

If the amount of memory is of interest instead of the amount of elements, use the SIZEOF
operator.

```
.data
myArray WORD 10, 20, 30, 40  ; 4 elements, each 2 bytes (WORD)

.code
mov ecx, LENGTHOF myArray   ; ECX becomes 4
mov eax, SIZEOF myArray     ; EAX becomes 8 (4 elements * 2 bytes)
```



# Procedures, Functions and Function Calls

In MASM a procedure is wrapped in <name> PROC and <name> ENDP.

```
.code

main PROC
    <function_body>
    ret
main ENDP
```

MASM does not generate any instructions for PROC and ENDP.
The advantage that PROC and ENDP have is that labels used within the wrapped source code
are local and will not conflict with labels of the exact same name used in other procedures!

The ret instruction is the counter part to the call instruction. Every path out of a procedures
needs the ret! If there is no ret, then the stack is in an invalid state (see later) and there
will most likely be an exception. Also the function will not jump back to the initial call instruction
but execute the code that comes after the function which may or may not be incorrect.

To call a function, use the call instruction. To exit from a function use the ret instruction.
call and ret need to be used in pairs as they modify the instruction pointer (eip/rip).
call pushes the address of the instruction following the call instruction on the stack and places
the address of the first instruction of the procedure into eip/rip so that the CPU "jumps" to the
called procedure. The ret instruction pops a value from the stack and treats it as an address and
writes that address into eip/rip. So that the CPU "jumps" back to the instruction after the call
instruction.

Since call has pushed the address of the instruction after the call instruction, this means that ret
makes the CPU "jump" to the instruction following the call instruction. This also means if you omit
ret, then the CPU will just continue executing (may be what you want to happen or may not be) and
also the stack now contains an address that nobody will ever remove from the stack!

Also note that call and ret do not take care of stack frames for function calls!
Call and ret also do not affect the base pointer register EBP/RBP!
Call and ret do affect the stack pointer ESP/RSP since call pushes the address of the instruction after
the call instruction. This push indirectly moves the ESP/RSP register value. ret will perform a pop
instruction which also affects the ESP/RSP register value.

Stack frames and their layout as well as the registers used for the Application Binary Interface (ABI)
for parameter passing are not part of the call/ret pair's functionality. call and ret merely use one
cell on the stack for the return address and they modify the eip/rip register and the ESP/RSP register
due to push and pop, nothing more.

## Stack Frames

In pure assembly applications, before using a register, the value of the register is pushed so that
the register can be restored by a pop of the original value later. Another use of the stack is to use
it to define variables. Variables are nothing but memory locations. As the stack is located in memory,
it can be used to define variables. The downside is that the variable defined via the stack are only
valid while the function/procedure call is ongoing. As such stack variables are local or temporary.
Global, long lived variables need to go somewhere else in memory, maybe the .data or .bss section.

High Level programming languages let the user declare variables. local variables in a programming language
will most likely implemented as space reserved on the stack. The space is reserved on the stack by
the stack frame. The Stack Frame is a section of the stack that is created when the function is entered
and removed when the function is exited before the ret instruction.

The start of the stack frame is marked by the EBP/RBP register which contains the base of the stack frame.
the end of the stack frame is marked by the ESP/RSP, the stack pointer. Local variables are reserved by
the stack frame in the area between EBP/RBP and ESP/RSP.

To create the stack frame, a compiler will take notes on which local variables are used in a function, it
will then create a layout for a stack frame so it knows which local variable lives where within the stack
frame and it will emit a set of instructions to produce and also to remove the stack frame, when the
function starts and ends respectively.

When stack frames are used, nested function calls get more complicated and the compiler has to emit
some extra code to keep the callers stack frame alive in addition to constructing the stack frame of
the callee.

The caller has set up their own EBP/RBP and ESP/RSP to mark the start and end of their stack frame.
These values need to kept exactly that way. The callee has the responsibility to restore the old
values back to their original state once the callee is returning via a ret call.

To do that, the callee could just push the EBP/RBP to the stack and the ESP/RSP to the stack.
These are two push instructions, but there is room for optimization.

The actual sequence used commonly is to push EBP/RBP to the stack (which moves ESP/RSP down one
stack cell), then to move the current value of ESP/RSP into EBP/RBP, making point both registers to
the same stack cell for a short time (that cell contains the original EBP/RBP value).
After that, stack space is reserved for the local variables which moves ESP/RSP downwords.

To remove the stack frame, first the value of EBP/RBP is moved into ESP/RSP. Now ESP/RSP still does
not point to the original stack cell but up two cells to where the original EBP/RBP value is stored.
Two cells because the call instruction has pushed the return address on the stack also!
A pop of the original EBP/RBP value into the EBP/RBP register will move EBP/RBP to it's original
value. ESP/RSP is moved up one cell since the EBP/RBP pop moves ESP/RSP up one cell!

Now ESP/RSP points to the return address.
When ret executes, it pops the return address from the stack and writes it to EIP/RIP.
The ret pop makes ESP/RSP again move up one cell and now points to the original end of the callers stack frame. Finally at this point the original stack frame of the caller is restored.

This is very complicated since the concept of call/ret and stackframes with EBP/RBP and ESP/RSP are mixed.

Here is code that performs these steps:

```
.code

main PROC
    ; prelude - create stack frame and shadow area

    ; create stack frame start by setting a new value into rbp
    push rbp ; save base of current stack frame to restore it later
    mov rbp, rsp ; set new base of new stack frame (to current stack pointer)

    ; create stack frame end by moving rsp down the stack to reserve space for all local variables
    ; Here, two QWORD shadow space are reserved plus space for 3 local QWORD variables!
    ; 32 byte shadow space must also be passed to stack (When integrating with Windows API functions)
    ;
    ; 8 * (3 + 2) = 8 * 5 = 40 ---- The stack frame is 40 bytes in size!
    sub rsp, 8 * (3 + 2) ; allocate shadow register area + 2 QWORDs for stack alignment
                         ; Function Preamble - save space on stack for all local variables - [AsmAstConversionVisitor::visit_tacky_function()]
                         ; + Updated in AsmAstFixupVisitor::visit_asm_ast_function()

    ; assign values to the local variables according to the stack frame layout
    ; which only the compiler knows (Here it is an array with three QWORD elements)
    mov qword ptr [rbp-40+0], 1    ; mov for CopyToOffset(src, dst, offset)
    mov qword ptr [rbp-40+8], 2    ; mov for CopyToOffset(src, dst, offset)
    mov qword ptr [rbp-40+16], 3    ; mov for CopyToOffset(src, dst, offset)


    lea rbx, qword ptr [rbp-40+0]   ; base pointer to array (int *my_pointer = my_array;)
    mov qword ptr [rbp-40+24], rbx
    mov rax, qword ptr [rbp-40+24]    ; mov generated for TACKY Load()
    ;
    ;;
    ;;mov r10, qword ptr [rax+0]
    ;
    mov rax, qword ptr [rax+0]
    ;
    ;; place return value into rax
    ;;mov qword ptr [rbp-40], r10
    ;;mov rax, qword ptr [rbp-40]    ; Generated by the Return keyword in asm_ast_conversion_visitor
    ;
    ;;mov rax, r10

    ; epilog - restore stack pointer
    mov rsp, rbp
    pop rbp

    ; pops the return address from the top of the stack into the instruction pointer (EIP/RIP)
    ret

    main ENDP

END
```

## Stack Frame Alignment

https://stackoverflow.com/questions/79519237/windows-masm-x64-calling-convention-and-stack-setup

[Stack must be 16 byte aligned before "CALL fun" is executed so padding is needed in some scenarios]




# Sample Code

## Write "Hello World" to the console (x64)

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
msg BYTE "Hello World",0        ; The string "Hello World" with a trailing zero for zero-termination
bytesWritten DWORD ?            ; space (DWORD) to store the amount of bytesWritten

.code
main PROC

    sub rsp, 5 * 8              ; shadow frame

    mov rcx, -11                ; -11 is std out handle ; STD_OUTPUT_HANDLE
    call GetStdHandle           ; Get std out into rax

    mov  rcx, rax               ; store std out handle from rax into rcx where WriteConsoleA expects it
    lea  rdx, msg               ; rdx contains the message to write
    mov  r8, LENGTHOF msg - 1   ; store amount of bytes to write into r8
    lea  r9, bytesWritten       ; load effective address of the byteWritten variable into r9
                                ; This is effectively a pointer to byteWritten
                                ; WriteConsoleA will write into the variable using the pointer in r9
    ; mov r9, 0                 ; Optional: nullptr for the number of bytes written if we are not
                                ; interested in the bytes written
    ;mov  QWORD PTR [rsp + 4 * SIZEOF QWORD], 0  ; ???
    call WriteConsoleA

    ; add rsp, 5 * 8              ; Clean up stack - causes a crash

    mov rcx, 0                  ; Return zero
    call ExitProcess
main ENDP

END
```

Error: LINK : error LNK2001: unresolved external symbol mainCRTStartup

In the properties of the project go to

```
Configuration Properties >> Linker >> Advanced
```

In Advanced at the top should be Entry Point. Type in main.

## ExitProcess (x64)

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

## MessageBox (x64)

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


# Int to String (itoa)

```
GetStdHandle PROTO
ExitProcess PROTO
WriteConsoleA PROTO


.DATA?                              ; .bss section, zero-initialized
    itoa_revers_buffer db 100 dup(?)
    itoa_buffer db 100 dup(?)

.data
    msg BYTE "Hello World",0        ; The string "Hello World" with a trailing zero for zero-termination
    bytesWritten DWORD ?            ; space (DWORD) to store the amount of bytesWritten
    ;test_data DWORD 1234            ; test integer to itoa convert
    test_data DWORD 12345678

.code
main PROC

    ;mov r10, 0                      ; initialize offset
    ;lea rdi, itoa_revers_buffer
    ;mov BYTE PTR [rdi + r10], 65    ; 'A'
    ;inc r10
    ;mov BYTE PTR [rdi + r10], 66    ; 'B'
    ;inc r10
    ;mov BYTE PTR [rdi + r10], 67    ; 'C'
    ;inc r10

    mov r10, 0                      ; initialize offset
    lea rdi, itoa_revers_buffer     ; store ptr to itoa_revers_buffer into RDI

    mov eax, test_data              ; place the integer to convert to a string into EAX
loop_lbl:
    mov rcx, 10                     ; the divisor
    mov rdx, 0                      ; clear register
    div rcx                         ; div places the division result into RAX and the modulo into RDX

    add rdx, 30h                    ; convert from digit to ASCII number by adding 30h = 48dec
    mov BYTE PTR [rdi + r10], dl    ; dl contains the byte that is the modulo result of the division by 10
    inc r10                         ; increment next buffer index. r10 also symbolizes string len

                                    ; check if the entiere integer has dissapeared due to repeated div
    cmp rax, 0                      ; rax contains the div part of the division (rdx contains the modulo)
    jne loop_lbl                    ; repeat the loop

                                    ; as the character string is currently stored in the buffer in
                                    ; reverse order, the last step inverts the buffer
    mov r9, r10                     ; save the string length by using a copy in r9 for the inversion
    ;dec r9
    mov r11, 0
    lea r8, itoa_buffer
    mov rbx, 0
invert_label:
    mov bl, BYTE PTR [rdi + r11]
    dec r9
    mov BYTE PTR [r8 + r9], bl

    inc r11

    cmp r9, 0

    jne invert_label

                                    ; end of itoa

    sub rsp, 5 * 8              ; shadow frame

    mov rcx, -11                ; -11 is std out handle ; STD_OUTPUT_HANDLE
    call GetStdHandle           ; Get std out into rax

    mov  rcx, rax               ; store std out handle from rax into rcx where WriteConsoleA expects it

    ;lea  rdx, msg               ; rdx contains the message to write
    lea rdx, itoa_buffer

    ;mov  r8, LENGTHOF msg - 1   ; store amount of bytes to write into r8
    mov r8, r10

    lea  r9, bytesWritten       ; load effective address of the byteWritten variable into r9
                                ; This is effectively a pointer to byteWritten
                                ; WriteConsoleA will write into the variable using the pointer in r9
    ; mov r9, 0                 ; Optional: nullptr for the number of bytes written if we are not
                                ; interested in the bytes written
    ;mov  QWORD PTR [rsp + 4 * SIZEOF QWORD], 0  ; ???
    call WriteConsoleA

    ;add rsp, 5 * 8              ; Clean up stack - causes a crash

    mov rcx, 0                  ; Return zero
    call ExitProcess
main ENDP

END
```