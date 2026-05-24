                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ISO C Compiler
                                      3 ; Version 4.5.0 #15242 (MINGW64)
                                      4 ;--------------------------------------------------------
                                      5 	.module sum
                                      6 	
                                      7 ;--------------------------------------------------------
                                      8 ; Public variables in this module
                                      9 ;--------------------------------------------------------
                                     10 	.globl _main
                                     11 ;--------------------------------------------------------
                                     12 ; ram data
                                     13 ;--------------------------------------------------------
                                     14 	.area DATA
                                     15 ;--------------------------------------------------------
                                     16 ; ram data
                                     17 ;--------------------------------------------------------
                                     18 	.area INITIALIZED
                                     19 ;--------------------------------------------------------
                                     20 ; Stack segment in internal ram
                                     21 ;--------------------------------------------------------
                                     22 	.area SSEG
      000001                         23 __start__stack:
      000001                         24 	.ds	1
                                     25 
                                     26 ;--------------------------------------------------------
                                     27 ; absolute external ram data
                                     28 ;--------------------------------------------------------
                                     29 	.area DABS (ABS)
                                     30 
                                     31 ; default segment ordering for linker
                                     32 	.area HOME
                                     33 	.area GSINIT
                                     34 	.area GSFINAL
                                     35 	.area CONST
                                     36 	.area INITIALIZER
                                     37 	.area CODE
                                     38 
                                     39 ;--------------------------------------------------------
                                     40 ; interrupt vector
                                     41 ;--------------------------------------------------------
                                     42 	.area HOME
      008000                         43 __interrupt_vect:
      008000 82 00 80 07             44 	int s_GSINIT ; reset
                                     45 ;--------------------------------------------------------
                                     46 ; global & static initialisations
                                     47 ;--------------------------------------------------------
                                     48 	.area HOME
                                     49 	.area GSINIT
                                     50 	.area GSFINAL
                                     51 	.area GSINIT
      008007 CD 80 85         [ 4]   52 	call	___sdcc_external_startup
      00800A 4D               [ 1]   53 	tnz	a
      00800B 27 03            [ 1]   54 	jreq	__sdcc_init_data
      00800D CC 80 04         [ 2]   55 	jp	__sdcc_program_startup
      008010                         56 __sdcc_init_data:
                                     57 ; stm8_genXINIT() start
      008010 AE 00 00         [ 2]   58 	ldw x, #l_DATA
      008013 27 07            [ 1]   59 	jreq	00002$
      008015                         60 00001$:
      008015 72 4F 00 00      [ 1]   61 	clr (s_DATA - 1, x)
      008019 5A               [ 2]   62 	decw x
      00801A 26 F9            [ 1]   63 	jrne	00001$
      00801C                         64 00002$:
      00801C AE 00 00         [ 2]   65 	ldw	x, #l_INITIALIZER
      00801F 27 09            [ 1]   66 	jreq	00004$
      008021                         67 00003$:
      008021 D6 80 2C         [ 1]   68 	ld	a, (s_INITIALIZER - 1, x)
      008024 D7 00 00         [ 1]   69 	ld	(s_INITIALIZED - 1, x), a
      008027 5A               [ 2]   70 	decw	x
      008028 26 F7            [ 1]   71 	jrne	00003$
      00802A                         72 00004$:
                                     73 ; stm8_genXINIT() end
                                     74 	.area GSFINAL
      00802A CC 80 04         [ 2]   75 	jp	__sdcc_program_startup
                                     76 ;--------------------------------------------------------
                                     77 ; Home
                                     78 ;--------------------------------------------------------
                                     79 	.area HOME
                                     80 	.area HOME
      008004                         81 __sdcc_program_startup:
      008004 CC 80 2D         [ 2]   82 	jp	_main
                                     83 ;	return from main will return to caller
                                     84 ;--------------------------------------------------------
                                     85 ; code
                                     86 ;--------------------------------------------------------
                                     87 	.area CODE
                                     88 ;	sum.c: 1: int main(void) {
                                     89 ;	-----------------------------------------
                                     90 ;	 function main
                                     91 ;	-----------------------------------------
      00802D                         92 _main:
      00802D 52 1A            [ 2]   93 	sub	sp, #26
                                     94 ;	sum.c: 3: int data[11] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0xaa};
      00802F 5F               [ 1]   95 	clrw	x
      008030 5C               [ 1]   96 	incw	x
      008031 1F 01            [ 2]   97 	ldw	(0x01, sp), x
      008033 AE 00 02         [ 2]   98 	ldw	x, #0x0002
      008036 1F 03            [ 2]   99 	ldw	(0x03, sp), x
      008038 AE 00 03         [ 2]  100 	ldw	x, #0x0003
      00803B 1F 05            [ 2]  101 	ldw	(0x05, sp), x
      00803D AE 00 04         [ 2]  102 	ldw	x, #0x0004
      008040 1F 07            [ 2]  103 	ldw	(0x07, sp), x
      008042 AE 00 05         [ 2]  104 	ldw	x, #0x0005
      008045 1F 09            [ 2]  105 	ldw	(0x09, sp), x
      008047 AE 00 06         [ 2]  106 	ldw	x, #0x0006
      00804A 1F 0B            [ 2]  107 	ldw	(0x0b, sp), x
      00804C AE 00 07         [ 2]  108 	ldw	x, #0x0007
      00804F 1F 0D            [ 2]  109 	ldw	(0x0d, sp), x
      008051 AE 00 08         [ 2]  110 	ldw	x, #0x0008
      008054 1F 0F            [ 2]  111 	ldw	(0x0f, sp), x
      008056 AE 00 09         [ 2]  112 	ldw	x, #0x0009
      008059 1F 11            [ 2]  113 	ldw	(0x11, sp), x
      00805B AE 00 0A         [ 2]  114 	ldw	x, #0x000a
      00805E 1F 13            [ 2]  115 	ldw	(0x13, sp), x
      008060 AE 00 AA         [ 2]  116 	ldw	x, #0x00aa
      008063 1F 15            [ 2]  117 	ldw	(0x15, sp), x
                                    118 ;	sum.c: 7: while (data[j] != 0xaa) {
      008065 5F               [ 1]  119 	clrw	x
      008066 1F 19            [ 2]  120 	ldw	(0x19, sp), x
      008068                        121 00101$:
      008068 1E 19            [ 2]  122 	ldw	x, (0x19, sp)
      00806A 58               [ 2]  123 	sllw	x
      00806B 1F 17            [ 2]  124 	ldw	(0x17, sp), x
      00806D 96               [ 1]  125 	ldw	x, sp
      00806E 5C               [ 1]  126 	incw	x
      00806F 72 FB 17         [ 2]  127 	addw	x, (0x17, sp)
      008072 FE               [ 2]  128 	ldw	x, (x)
      008073 A3 00 AA         [ 2]  129 	cpw	x, #0x00aa
      008076 27 07            [ 1]  130 	jreq	00103$
                                    131 ;	sum.c: 9: j = j + 1;
      008078 1E 19            [ 2]  132 	ldw	x, (0x19, sp)
      00807A 5C               [ 1]  133 	incw	x
      00807B 1F 19            [ 2]  134 	ldw	(0x19, sp), x
      00807D 20 E9            [ 2]  135 	jra	00101$
      00807F                        136 00103$:
                                    137 ;	sum.c: 12: __asm__("halt\n");
      00807F 8E               [10]  138 	halt
                                    139 ;	sum.c: 13: __asm__("wfi\n");
      008080 8F               [10]  140 	wfi
                                    141 ;	sum.c: 15: return 0;
      008081 5F               [ 1]  142 	clrw	x
                                    143 ;	sum.c: 16: }
      008082 5B 1A            [ 2]  144 	addw	sp, #26
      008084 81               [ 4]  145 	ret
                                    146 	.area CODE
                                    147 	.area CONST
                                    148 	.area INITIALIZER
                                    149 	.area CABS (ABS)
