// https://developer.arm.com/documentation/dui0473/m/writing-arm-assembly-language/load-immediate-values-using-ldr-rd---const
//
// Load immediate values using LDR Rd, =const
// The LDR Rd,=const pseudo-instruction generates the most efficient single instruction to load any 32-bit number.
//
// You can use this pseudo-instruction to generate constants that are out of range of the MOV and MVN instructions.
//
// The LDR pseudo-instruction generates the most efficient single instruction for the specified immediate value:
//
// If the immediate value can be constructed with a single MOV or MVN instruction, the assembler generates the appropriate instruction.
// If the immediate value cannot be constructed with a single MOV or MVN instruction, the assembler:
// Places the value in a literal pool (a portion of memory embedded in the code to hold constant values).
// Generates an LDR instruction with a PC-relative address that reads the constant from the literal pool.

.global _start

#.equ endlist, 0x04
#.equ endlist, 0xFFFF
.equ endlist, 0xFFFFFFFF

_start:
    LDR R0, =endlist

// assembler: https://ret.futo.org/arm32/?thumb
// df f8 01 00

// assembler: https://armconverter.com/?code=LDR+R0,+%231
// DFF80100


    SWI 0       // halt the chip
