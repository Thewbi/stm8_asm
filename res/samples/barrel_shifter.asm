; https://kannwischer.eu/croatia2023/20230608_croatia_m4.pdf

; example
mov r0 , #42
mov r1 , #37
ror r1, r1, #1
orr r2 , r0 , r1
lsl r2 , r2 , #1
eor r0 , r2

; more efficient
mov r0 , #42
mov r1 , #37
orr r2 , r0 , r1 , ror #1
eor r0 , r0 , r2 , lsl #1