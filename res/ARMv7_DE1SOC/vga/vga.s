@ https://ecse324.ece.mcgill.ca/simulator/?sys=arm-de1soc

.global _start
_start:
        bl      draw_test_screen
end:
        b       end

@ TODO: Insert VGA driver functions here.

@                            r0     r1        r2
@ void VGA_draw_point_ASM(int x, int y, short c);
VGA_draw_point_ASM:
		push    {r4, r5, r6, r7, r8, r9, r10, lr}
		
		@ 0xc8000000 | (y << 10) | (x << 1)
		
		mov		r9, #0xc8000000
		
		mov 	r10, #10
		lsls 	r1, r10 		@ y
		orr 	r9, r1
		
		mov 	r10, #1
		lsls 	r0, r10			@ x
		orr 	r9, r0

        @ Use suffixes B and H with the assembly memory access instructions in order to read/modify the bytes/half-words of the memory contents.
		strh     r2, [r9]
		
		pop     {r4, r5, r6, r7, r8, r9, r10, pc}
		BX      LR
	
@ void VGA_clear_pixelbuff_ASM();
VGA_clear_pixelbuff_ASM:
		push    {r4, r5, r6, r7, r8, r9, r10, lr}
		
		mov		r9, #0xc8000000
		
		@mov		r8, #0 				@ black
		mov			r8, #0xFFFFFFFF		@ white
		
		mov     r4, #0
		
		@ 320x240 =  76800 pixel.  76800 * 2 Byte = 153600 byte
		@ 320x320 = 102400 pixel. 102400 * 2 Byte = 204800 byte
		@ 320x400 = 128000 pixel. 153600 * 2 Byte = 256000 byte
		@ 320x480 = 153600 pixel. 153600 * 2 Byte = 307200 byte

@l_1_c:	cmp 	r4, #153600
l_1_c:	cmp 	r4, #256000		

		beq		end_1_c
		
		add 	r10, r9, r4		
		
		str		r8, [r10]
		
		add		r4, #4
		
		b 		l_1_c	
		
end_1_c:	pop     {r4, r5, r6, r7, r8, r9, r10, pc}
		BX      LR

@ void VGA_write_char_ASM(int x, int y, char c);
VGA_write_char_ASM:
		push    {r4, r5, r6, r7, r8, r9, r10, lr}
		
		@ 0xc9000000 | (y << 7) | (x << 0)
		
		mov		r9, #0xc9000000
		
		mov 	r10, #7
		lsls 	r1, r10 		@ y
		orr 	r9, r1
		
		mov 	r10, #0
		lsls 	r0, r10			@ x
		orr 	r9, r0

        @ Use suffixes B and H with the assembly memory access instructions in order to read/modify the bytes/half-words of the memory contents.
		strb     r2, [r9]
		
		pop     {r4, r5, r6, r7, r8, r9, r10, pc}
		BX      LR

@ void VGA_clear_charbuff_ASM();
VGA_clear_charbuff_ASM:
		push    {r4, r5, r6, r7, r8, r9, r10, lr}
		
		mov		r9, #0xc9000000
		
		mov		r8, #0 				@ black
		@mov			r8, #0xFFFFFFFF		@ white
		
		mov     r4, #0
		
		@ 80x60 = 4800 character. 4800 Byte		

@l_2_c:	cmp 	r4, #4800		
l_2_c:	cmp 	r4, #8000		
		beq		end_2_c
		
		add 	r10, r9, r4		
		
		str		r8, [r10]
		
		add		r4, #4
		
		b 		l_2_c	
		
end_2_c:	pop     {r4, r5, r6, r7, r8, r9, r10, pc}
		BX      LR



draw_test_screen:
        push    {r4, r5, r6, r7, r8, r9, r10, lr}
        bl      VGA_clear_pixelbuff_ASM
        bl      VGA_clear_charbuff_ASM
        mov     r6, #0
        ldr     r10, .draw_test_screen_L8
        ldr     r9, .draw_test_screen_L8+4
        ldr     r8, .draw_test_screen_L8+8
        b       .draw_test_screen_L2
.draw_test_screen_L7:
        add     r6, r6, #1
        cmp     r6, #320
        beq     .draw_test_screen_L4
.draw_test_screen_L2:
        smull   r3, r7, r10, r6
        asr     r3, r6, #31
        rsb     r7, r3, r7, asr #2
        lsl     r7, r7, #5
        lsl     r5, r6, #5
        mov     r4, #0
.draw_test_screen_L3:
        smull   r3, r2, r9, r5
        add     r3, r2, r5
        asr     r2, r5, #31
        rsb     r2, r2, r3, asr #9
        orr     r2, r7, r2, lsl #11
        lsl     r3, r4, #5
        smull   r0, r1, r8, r3
        add     r1, r1, r3
        asr     r3, r3, #31
        rsb     r3, r3, r1, asr #7
        orr     r2, r2, r3
        mov     r1, r4
        mov     r0, r6
        bl      VGA_draw_point_ASM
        add     r4, r4, #1
        add     r5, r5, #32
        cmp     r4, #240
        bne     .draw_test_screen_L3
        b       .draw_test_screen_L7
.draw_test_screen_L4:
        mov     r2, #72 	@ (ASCII H)
        mov     r1, #5  	@ (y)
        mov     r0, #20     @ (x)
        bl      VGA_write_char_ASM
        mov     r2, #101
        mov     r1, #5
        mov     r0, #21
        bl      VGA_write_char_ASM
        mov     r2, #108
        mov     r1, #5
        mov     r0, #22
        bl      VGA_write_char_ASM
        mov     r2, #108
        mov     r1, #5
        mov     r0, #23
        bl      VGA_write_char_ASM
        mov     r2, #111
        mov     r1, #5
        mov     r0, #24
        bl      VGA_write_char_ASM
        mov     r2, #32
        mov     r1, #5
        mov     r0, #25
        bl      VGA_write_char_ASM
        mov     r2, #87
        mov     r1, #5
        mov     r0, #26
        bl      VGA_write_char_ASM
        mov     r2, #111
        mov     r1, #5
        mov     r0, #27
        bl      VGA_write_char_ASM
        mov     r2, #114
        mov     r1, #5
        mov     r0, #28
        bl      VGA_write_char_ASM
        mov     r2, #108
        mov     r1, #5
        mov     r0, #29
        bl      VGA_write_char_ASM
        mov     r2, #100
        mov     r1, #5
        mov     r0, #30
        bl      VGA_write_char_ASM
        mov     r2, #33
        mov     r1, #5
        mov     r0, #31
        bl      VGA_write_char_ASM
		
		mov     r2, #0x46 	@ (ASCII F)
        mov     r1, #7  	@ (y)
        mov     r0, #20 	@ (x)
		bl      VGA_write_char_ASM
		mov     r2, #0x55 	@ (ASCII U)
        mov     r1, #7  	@ (y)
        mov     r0, #21 	@ (x)
		bl      VGA_write_char_ASM
		mov     r2, #0x43 	@ (ASCII C)
        mov     r1, #7		@ (y)
        mov     r0, #22 	@ (x)
		bl      VGA_write_char_ASM
		mov     r2, #0x4B 	@ (ASCII K)
        mov     r1, #7  	@ (y)
        mov     r0, #23 	@ (x)
		bl      VGA_write_char_ASM
		
        pop     {r4, r5, r6, r7, r8, r9, r10, pc}
.draw_test_screen_L8:
        .word   1717986919
        .word   -368140053
        .word   -2004318071
