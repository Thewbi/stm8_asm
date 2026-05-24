;--------------------------------------------------------
; File Created by SDCC : free open source ISO C Compiler
; Version 4.5.0 #15242 (MINGW64)
;--------------------------------------------------------
	.module sum
	
;--------------------------------------------------------
; Public variables in this module
;--------------------------------------------------------
	.globl _main
;--------------------------------------------------------
; ram data
;--------------------------------------------------------
	.area DATA
;--------------------------------------------------------
; ram data
;--------------------------------------------------------
	.area INITIALIZED
;--------------------------------------------------------
; Stack segment in internal ram
;--------------------------------------------------------
	.area SSEG
__start__stack:
	.ds	1

;--------------------------------------------------------
; absolute external ram data
;--------------------------------------------------------
	.area DABS (ABS)

; default segment ordering for linker
	.area HOME
	.area GSINIT
	.area GSFINAL
	.area CONST
	.area INITIALIZER
	.area CODE

;--------------------------------------------------------
; interrupt vector
;--------------------------------------------------------
	.area HOME
__interrupt_vect:
	int s_GSINIT ; reset
;--------------------------------------------------------
; global & static initialisations
;--------------------------------------------------------
	.area HOME
	.area GSINIT
	.area GSFINAL
	.area GSINIT
	call	___sdcc_external_startup
	tnz	a
	jreq	__sdcc_init_data
	jp	__sdcc_program_startup
__sdcc_init_data:
; stm8_genXINIT() start
	ldw x, #l_DATA
	jreq	00002$
00001$:
	clr (s_DATA - 1, x)
	decw x
	jrne	00001$
00002$:
	ldw	x, #l_INITIALIZER
	jreq	00004$
00003$:
	ld	a, (s_INITIALIZER - 1, x)
	ld	(s_INITIALIZED - 1, x), a
	decw	x
	jrne	00003$
00004$:
; stm8_genXINIT() end
	.area GSFINAL
	jp	__sdcc_program_startup
;--------------------------------------------------------
; Home
;--------------------------------------------------------
	.area HOME
	.area HOME
__sdcc_program_startup:
	jp	_main
;	return from main will return to caller
;--------------------------------------------------------
; code
;--------------------------------------------------------
	.area CODE
;	sum.c: 1: int main(void) {
;	-----------------------------------------
;	 function main
;	-----------------------------------------
_main:
	sub	sp, #26
;	sum.c: 3: int data[11] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0xaa};
	clrw	x
	incw	x
	ldw	(0x01, sp), x
	ldw	x, #0x0002
	ldw	(0x03, sp), x
	ldw	x, #0x0003
	ldw	(0x05, sp), x
	ldw	x, #0x0004
	ldw	(0x07, sp), x
	ldw	x, #0x0005
	ldw	(0x09, sp), x
	ldw	x, #0x0006
	ldw	(0x0b, sp), x
	ldw	x, #0x0007
	ldw	(0x0d, sp), x
	ldw	x, #0x0008
	ldw	(0x0f, sp), x
	ldw	x, #0x0009
	ldw	(0x11, sp), x
	ldw	x, #0x000a
	ldw	(0x13, sp), x
	ldw	x, #0x00aa
	ldw	(0x15, sp), x
;	sum.c: 7: while (data[j] != 0xaa) {
	clrw	x
	ldw	(0x19, sp), x
00101$:
	ldw	x, (0x19, sp)
	sllw	x
	ldw	(0x17, sp), x
	ldw	x, sp
	incw	x
	addw	x, (0x17, sp)
	ldw	x, (x)
	cpw	x, #0x00aa
	jreq	00103$
;	sum.c: 9: j = j + 1;
	ldw	x, (0x19, sp)
	incw	x
	ldw	(0x19, sp), x
	jra	00101$
00103$:
;	sum.c: 12: __asm__("halt\n");
	halt
;	sum.c: 13: __asm__("wfi\n");
	wfi
;	sum.c: 15: return 0;
	clrw	x
;	sum.c: 16: }
	addw	sp, #26
	ret
	.area CODE
	.area CONST
	.area INITIALIZER
	.area CABS (ABS)
