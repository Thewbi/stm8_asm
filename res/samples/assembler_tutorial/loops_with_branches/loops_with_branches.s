.global _start
//.equ endlist, 0xaaaaaaaa
.equ endlist, 0x0a

_start:

    // (https://developer.arm.com/documentation/dui0473/m/writing-arm-assembly-language/load-immediate-values)
    // (https://developer.arm.com/documentation/dui0473/m/writing-arm-assembly-language/load-immediate-values-using-ldr-rd---const)
    //
    // You can also use the LDR pseudo-instruction to load immediate values into a register.
	LDR R0,=list
	LDR R3,=endlist
	LDR R1,[R0]
	ADD R2,R2,R1

loop:

//	LDR R1,[R0,#4]!     // Error: Thumb does not support this addressing mode

    LDR R1, [R0]
    ADD R0, R0, #4

	CMP R1,R3           // why would R1 ever contain 0xaaaaaaaa? The endlist constant is not inserted at the end of the linst at all!
	BEQ exit
	ADD R2,R2,R1
	BAL loop
	
exit:
    SWI 0               // halt the chip

.data
list:
	.word 1,2,3,4,5,6,7,8,9,10
	