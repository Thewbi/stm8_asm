# Links

https://cs.lmu.edu/~ray/notes/nasmtutorial/

https://scot.tg/2023/05/02/debugging-with-pdbs/
https://github.com/microsoft/microsoft-pdb

# Installation

https://www.nasm.us/pub/nasm/releasebuilds/

https://www.nasm.us/pub/nasm/releasebuilds/3.02rc13/win64/nasm-3.02rc13-installer-x64.exe

# Building

NASM uses intel syntax but differs slightly from Microsofts MASM.

```
C:\Users\lapto\AppData\Local\bin\NASM\nasm.exe
```

Open "x64 Native Tools Command Prompt for VS...." from the start menu

```
set PATH=%PATH%;C:\Users\lapto\AppData\Local\bin\NASM

cd C:\Users\lapto\dev\nasm\hello_world
del main.obj main.exe

nasm -f win64 -o main.obj main.asm

link main.obj /subsystem:console /out:main.exe kernel32.lib legacy_stdio_definitions.lib msvcrt.lib
```





# Trying to debug (MINGW MSYS2)

There are two ways to use NASM on Windows.

The first is to use NASM to output win64 output (nasm -f win64 -o main.obj main.asm) form (object file will be in COFF format) together with the Microsoft Linker link to build a .exe file in the x64 Native Tools Command prompt window. Then the last step is to debug the .exe in a debugger such as dbgrs.

The second is to install the GNU toolchain using MingW MSYS2. Then, in the MSYS2 MINGW64 command prompt window, use the GNU toolchain by assembling with NASM using the DWARD and ELF format (nasm -g -F dwarf -f elf64 -l main.lst main.asm), then linking with the GNU linker ld. The final step is to debug the .exe file using the GNU debugger gdb.

There is an important fact to be aware of here. NASM is not able to create debug information for the Windows COFF and PE formats! This means that a native windows debugger such as dbgrs will not find debug information inside the .exe file!

NASM is able to create debug information when using option 2 for the GNU formats. This means that it is possible to debug the resulting .exe in the gdb debugger!

There is this post on stack overflow: https://stackoverflow.com/questions/274566/how-can-i-debug-a-mingw-exe-with-the-microsoft-visual-c-debugger

It says that on a windows native debugger, debug symbols are read from a separate .pdb file instead of from within the .exe file. The post says that one way to get debug symbols out of NASM on windows is to assemble using NASM with the .elf and DWARF formats inside MSYS2 MINGW64 and then using a tool called cv2pdb to extract a .pdb file from the .exe. Then it should be possible for a native windows debugger to debug the .exe file using the .pdb file for debug symbols. I tried this approach and it did not work.

```
; ; For this command to work, the .asm file needs to contain segment instructions
; ; instead of section instructions!
;
; Open "x64 Native Tools Command Prompt for VS...." from the start menu
;
; set PATH=%PATH%;C:\Users\lapto\AppData\Local\bin\NASM
; cd C:\Users\lapto\dev\nasm\hello_world
;
; del main.obj main.o main.pdb main.lst main.exe
;
; nasm -f win64 -o main.obj main.asm
;
; ; For this command to work, the .asm file needs to contain segment instructions
; ; instead of section instructions!
; link main.obj /subsystem:console /out:main.exe kernel32.lib legacy_stdio_definitions.lib msvcrt.lib
;
; main.exe
;
;
; https://stackoverflow.com/questions/33324452/nasm-debug-symbols-for-windows
; Link via .elf format because NASM only outputs debug symbols in the .elf format
; (no debug symbols for COFF files with NASM)
;
; Open MSYS2 MINGW64 console
; cd /c/Users/lapto/dev/nasm/hello_world
;
; rm main.obj main.o main.pdb main.lst main.exe
;
; /c/Users/lapto/AppData/Local/bin/NASM/nasm -f elf -g main.asm -l main.lst
; /c/Users/lapto/AppData/Local/bin/NASM/nasm -g -F dwarf -f elf64 -l main.lst main.asm
;
; In the GNU linker (ld), the -s (or --strip-all) flag is used to completely strip all
; symbol information and debugging symbols from the final output executable or library file.
; [1] (https://manpages.debian.org/testing/binutils-common/ld.1.en.html)
; [2] (https://stackoverflow.com/questions/5244509/no-debugging-symbols-found-when-using-gdb)
;
; GNU ld (GNU Binutils) 2.28
;
; ld -lc -mi386pe -o main.exe main.o
; ld -mi386pe /lib/crt0.o -o main.exe main.o
; ld -o output /lib/crt0.o main.o -lc
; ld main.o -b elf64-x86-64 -lmsvcrt -entry=main -subsystem=console -o main.exe
; ld main.o -b elf64-x86-64 -lmsvcrt -lkernel32 --enable-stdcall-fixup %SYSTEMROOT%/system32/kernel32.dll -entry=main -subsystem=console -o main.exe
; ld -lmsvcrt -lkernel32 --enable-stdcall-fixup %SYSTEMROOT%/system32/kernel32.dll -entry=main -subsystem=console -o main.exe main.o
;
; This works:
; https://blog.code-cop.org/2015/07/hello-world-windows-32-assembly.html
; This works: (use /c/Users/lapto/AppData/Local/bin/NASM/nasm -f elf64 -g main.asm -l main.lst to build the object file)
; ld main.o -b elf64-x86-64 -lmsvcrt -lkernel32 --enable-stdcall-fixup /C/Windows/System32/kernel32.dll -entry=_main -subsystem=console -o main.exe
; ld main.o -b elf64-x86-64 -lmsvcrt -lkernel32 /C/Windows/System32/kernel32.dll -subsystem=console -o main.exe
; ld main.o -lmsvcrt -lkernel32 /C/Windows/System32/kernel32.dll -subsystem=console -o main.exe
;
; Read this:
; https://stackoverflow.com/questions/274566/how-can-i-debug-a-mingw-exe-with-the-microsoft-visual-c-debugger
;
; dump symbols from the object file:
; objdump --syms main.o
; objdump --syms main.exe
;
; https://stackoverflow.com/questions/274566/how-can-i-debug-a-mingw-exe-with-the-microsoft-visual-c-debugger
; /c/Users/lapto/Downloads/cv2pdb-0.54/cv2pdb64.exe main.exe
; THIS IS A DESTRUCTIVE OPERATION! DEBUGGING SYMBOLS ARE REMOVED FROM THE EXE AND INSERTED INTO THE PDB!
; DEBUGGING WITH GDB WILL NOT BE EASY ANY MORE AFTER THIS COMMAND!
;
; ./main.exe
;
; gdb ./main.exe
;
; (gdb) b start
; Breakpoint 1 at 0x400080: file hello.asm, line 7.
;
; (gdb) run
;
; (gdb) x/10i $pc
;
; For debug symbols: https://stackoverflow.com/questions/72268576/gdb-no-symbol-files-found-in-nasm-assembled-file
; Adding section .text before main solves the problem. --> It does not! This did not work for me!
```

# Tutorial

## Registers

https://wiki.osdev.org/CPU_Registers_x86-64

The x64 CPU has 16 64-bit registers R0 to R15.

Some of the registers have special names and are used for special purposes:

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

There are also 16 128-bit XMM registers

XMM0
XMM1
XMM2
XMM3
XMM4
XMM5
XMM6
XMM7
XMM8
XMM9
XMM10
XMM11
XMM12
XMM13
XMM14
XMM15

## Addressing

```
[ number ]
[ reg ]
[ reg + reg*scale ]      scale is 1, 2, 4, or 8 only
[ reg + number ]
[ reg + reg*scale + number ]
```

# Reserving Memory and Initializing Memory

db - define byte and initialize
dw - define wird and initialize
dd - define double-word and initialize
dq - define quad-word and initialize

dt - ???

The resb, resw, resq assembler instructions are used to reserver space without initializiation.

buffer:         resb    64              ; reserve 64 bytes (without initializing)
wordvar:        resw    1               ; reserve a word
realarray:      resq    10              ; array of ten reals

# Examples Windows

## Hello World

```
;
; Simple NASM syntax assembly program for x86 (32 bit).
;
; Use commands below to assemble, link and run ($ is the prompt):
; $ "C:\Program Files\NASM\nasm.exe" -f elf32 helloworld.asm
; $ "C:\Program Files\NASM\nasm.exe" -f win32 -o helloworld.obj helloworld.asm
;
; Make sure the .o file is not opened in another application because windows locks files
; $ "C:\aaa_se\MyLinker\x64\Debug\MyLinker.exe" -entry "main" -out "helloworld.exe" helloworld.obj
;
; $ gcc -m32 -o hello hello.o
; $ ./hello
; Hello, world!
;
; set PATH=%PATH%;C:\Users\U5353\Downloads\nasm-3.02-win64\nasm-3.02
;
; del main.obj main.exe
;
; nasm -f win32 -o main.obj main.asm
; ld -mi386pe -o main.exe main.obj
; C:\masm32\bin\link /subsystem:windows /nodefaultlib /entry:main helloworld.obj C:\masm32\lib\kernel32.lib
;
; https://stackoverflow.com/questions/64413414/unresolved-external-symbol-printf-in-windows-x64-assembly-programming-with-nasm
; Install Visual Studio in some available version (Community Edition maybe)
; Download NASM in the portable version as a zip archive from https://www.nasm.us/pub/nasm/releasebuilds/3.02/win64/
;
; open "x64 Native Tools Command Prompt for VS...." from the start menu
;
; set PATH=%PATH%;C:\Users\U5353\Downloads\nasm-3.02-win64\nasm-3.02
; cd C:\aaa_se\asm\helloworld
; del main.obj main.exe
; nasm -f win64 -o main.obj main.asm
; link main.obj /subsystem:console /out:main.exe kernel32.lib legacy_stdio_definitions.lib msvcrt.lib

; extern printf

; section .text

; global main

; main:
;     push ebp
;     mov ebp, esp

;     push msg
;     call printf
;     add esp, 4

;     leave
;     ret

; section .data
;     msg     db  'Hello, world!', 13, 10, 0



bits 64
default rel          ; make [msg] default to RIP-relative, not 32-bit absolute

segment .data
   msg: db "Hello world!", 0xd, 0xa, 0    ; CR LF and terminating 0

segment .text
global main
extern ExitProcess
extern printf

main:
   push    rbp
   mov     rbp, rsp        ; frame pointer
   sub     rsp, 32         ; shadow space

   lea     rcx, [msg]
   call    printf

   xor     ecx, ecx
   call    ExitProcess    ; or better:   call exit
                          ; to flush stdout if redirected to a file

  ; leave              ; or just return from main
  ; xor    eax,eax     ; with main's return value becoming exit status
  ; ret
```


# Examples Linux

## Hello World

```
; ----------------------------------------------------------------------------------------
; Writes "Hello, World" to the console using only system calls. Runs on 64-bit Linux only.
; To assemble and run:
;
;     nasm -felf64 hello.asm && ld hello.o && ./a.out
; ----------------------------------------------------------------------------------------

          global    main

          section   .text
main:     mov       rax, 1                  ; system call for write
          mov       rdi, 1                  ; file handle 1 is stdout
          mov       rsi, message            ; address of string to output
          mov       rdx, 13                 ; number of bytes
          syscall                           ; invoke operating system to do the write
          mov       rax, 60                 ; system call for exit
          xor       rdi, rdi                ; exit code 0
          syscall                           ; invoke operating system to exit

          section   .data
message:  db        "Hello, World", 10      ; note the newline at the end
```


