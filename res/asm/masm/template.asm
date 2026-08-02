; x86 Assembly Example
; INCLUDES - Libraries required for the functionality of the program.
INCLUDELIB kernel32.lib                     ; Used for ExitProcess


; PROGRAM CONFIGURATION - Defines the processor, memory model, and stack size.
.386                                        ; Using x86_32 architecture
.model flat, stdcall
.stack 4096


; FUNCTION PROTOTYPES - Declaration of external functions used in this program.
ExitProcess PROTO dwExitCode:DWORD


; DATA SEGMENT - Reserved space for data used in the program.
.DATA


; CODE SEGMENT - Contains the actual code (instructions) of the program.
.CODE                   
    MainEntryPoint PROC                     ; Start of main procedure - Entry point of the program

        ; Your code here

        INVOKE ExitProcess, 0
    MainEntryPoint ENDP                     ; End of main procedure


; END OF FILE - Specifies the entry point and marks the end of this source file.
END MainEntryPoint                          ; End of program, specify the entry point
