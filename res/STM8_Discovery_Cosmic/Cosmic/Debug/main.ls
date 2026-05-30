   1                     ; C Compiler for STM8 (COSMIC Software)
   2                     ; Parser V4.11.13 - 05 Feb 2019
   3                     ; Generator V4.4.9 - 06 Feb 2019
 138                     ; 94 void main(void)
 138                     ; 95 {
 140                     	switch	.text
 141  0000               _main:
 145                     ; 106   IWDG->KR = 0xAAu;
 148  0000 35aa50e0      	mov	20704,#170
 149                     ; 112   WWDG->CR = cnt | WWDG_CR_WDGA;
 152  0004 35ff50d1      	mov	20689,#255
 153                     ; 113   WWDG->WR = win;
 155  0008 357f50d2      	mov	20690,#127
 156                     ; 78     LEDS_PORT->DDR |= (ALL_LEDs);
 159  000c c65007        	ld	a,20487
 160  000f aa0f          	or	a,#15
 161  0011 c75007        	ld	20487,a
 162                     ; 79     LEDS_PORT->CR1 |= (ALL_LEDs);
 164  0014 c65008        	ld	a,20488
 165  0017 aa0f          	or	a,#15
 166  0019 c75008        	ld	20488,a
 167                     ; 80     LEDS_PORT->ODR &= ~(ALL_LEDs);
 169  001c c65005        	ld	a,20485
 170  001f a4f0          	and	a,#240
 171  0021 c75005        	ld	20485,a
 172                     ; 85     LEDS_PORT->DDR |= TEST_PIN;
 175  0024 721a5007      	bset	20487,#5
 176                     ; 86     LEDS_PORT->CR1 |= TEST_PIN;
 178  0028 721a5008      	bset	20488,#5
 179                     ; 87     LEDS_PORT->ODR &= ~TEST_PIN;
 181  002c 721b5005      	bres	20485,#5
 182                     ; 118   enableInterrupts();
 185  0030 9a            rim
 187                     ; 122     STL_InitRunTimeChecks();
 190  0031 cd0000        	call	_STL_InitRunTimeChecks
 192                     ; 126     switch_clock_system(to_HSE);
 194  0034 a6b4          	ld	a,#180
 195  0036 cd0000        	call	_switch_clock_system
 197                     ; 133   enableInterrupts();
 200  0039 9a            rim
 202  003a               L56:
 203                     ; 155       STL_DoRunTimeChecks();
 205  003a cd0000        	call	_STL_DoRunTimeChecks
 208  003d 20fb          	jra	L56
 231                     ; 256   void assert_failed(void)
 231                     ; 257 #endif /* FULL_ASSERT */
 231                     ; 258 {
 232                     	switch	.text
 233  003f               _assert_failed:
 237                     ; 260 }
 240  003f 81            	ret
 253                     	xdef	_main
 254                     	xdef	_assert_failed
 255                     	xref	_FailSafe
 256                     	xref	_switch_clock_system
 257                     	xref	_STL_DoRunTimeChecks
 258                     	xref	_STL_InitRunTimeChecks
 277                     	end
