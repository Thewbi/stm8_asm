.global _start
_start:
        MOV R0, #1          @ ARGUMENT 1: Dateideskriptor (1 = stdout) -> Gespeichert in R0
        LDR R1, =message    @ ARGUMENT 2: Zeiger auf die Nachricht
        LDR R2, =len        @ ARGUMENT 3: Länge der Nachricht
        MOV R7, #4          @ Systemaufruf-Nummer 4 (sys_write in Linux)
        SWI 0

        MOV R7, #1
        SWI 0

.data
message:
        .asciz "Hello World!\n"
len = .-message
