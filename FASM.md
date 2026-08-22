# x86-64 Registers in Windows 10

From https://gpfault.net/posts/asm-tut-0.txt.html

| Register | Lower byte | Lower word | Lower dword | Comments |
| - | - | - | - | - |
| rax | al | ax | eax | |
| rbx | bl | bx	ebx | |
| rcx | cl | cx	ecx | |
| rdx | dl | dx	edx | |
| rsp | spl | sp | esp | rsp holds the stack pointer |
| rsi | sil | si | esi | rsi and rdi serve as source and destination index for "string manipulation" instructions. |
| rdi | dil | di | edi | rsi and rdi serve as source and destination index for "string manipulation" instructions. |
| rbp | bpl | bp | ebp | |
| r8 | r8b | r8w | r8d | |
| r9 | r9b | r9w | r9d | |
| r10 | r10b | r10w | r10d | |
| r11 | r11b | r11w | r11d | |
| r12 | r12b | r12w | r12d | |
| r13 | r13b | r13w | r13d | |
| r14 | r14b | r14w | r14d | |
| r15 | r15b | r15w | r15d | |

rip - instruction pointer
rflags - contains the flags

# Basic FASM template

```
format PE64 NX GUI 6.0
entry start

section '.text' code readable executable
start:
        int3
        ret
```