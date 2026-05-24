// Source: https://github.com/Shikha-code36/assembly-ARM-tutorial

.global _start
_start:
	MOV R0, #30
	MOV R7, #1

	SWI 0       // halt the chip
