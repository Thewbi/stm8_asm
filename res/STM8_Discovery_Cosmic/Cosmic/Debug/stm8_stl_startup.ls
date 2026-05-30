   1                     ; C Compiler for STM8 (COSMIC Software)
   2                     ; Parser V4.11.13 - 05 Feb 2019
   3                     ; Generator V4.4.9 - 06 Feb 2019
 101                     	switch	.ubsct
 102  0000               _p_RunTimeRamChkInv:
 103  0000 0000          	ds.b	2
 104  0002               _p_RunTimeRamChk:
 105  0002 0000          	ds.b	2
 106                     .RUN_TIME_BUF:	section	.bss
 107  0000               _aRunTimeRamBuf:
 108  0000 000000000000  	ds.b	8
 109                     	switch	.ubsct
 110  0004               _CRCBlockIndex:
 111  0004 00            	ds.b	1
 112                     .CLASS_B:	section	.bss
 113  0000               _CurrentDesc:
 114  0000 0000          	ds.b	2
 115  0002               _CurrentCrc16:
 116  0002 0000          	ds.b	2
 117                     	switch	.ubsct
 118  0005               _p_RunCrc16Chk:
 119  0005 0000          	ds.b	2
 120                     	switch	.CLASS_B
 121  0004               _LastCtrlFlowCnt:
 122  0004 0000          	ds.b	2
 123  0006               _TimeBaseFlag:
 124  0006 00            	ds.b	1
 125  0007               _TickCounter:
 126  0007 0000          	ds.b	2
 127  0009               _ISRCtrlFlowCnt:
 128  0009 0000          	ds.b	2
 129  000b               _CtrlFlowCnt:
 130  000b 0000          	ds.b	2
 131                     .STACK_BOTTOM:	section	.bss
 132  0000               _aStackOverFlowPtrn:
 133  0000 00000000      	ds.b	4
 134                     .CLASS_B_REV:	section	.bss
 135  0000               _CurrentDescInv:
 136  0000 0000          	ds.b	2
 137  0002               _CurrentCrc16Inv:
 138  0002 0000          	ds.b	2
 139                     	switch	.ubsct
 140  0007               _p_RunCrc16ChkInv:
 141  0007 0000          	ds.b	2
 142                     	switch	.CLASS_B_REV
 143  0004               _LastCtrlFlowCntInv:
 144  0004 0000          	ds.b	2
 145  0006               _TimeBaseFlagInv:
 146  0006 00            	ds.b	1
 147  0007               _TickCounterInv:
 148  0007 0000          	ds.b	2
 149  0009               _ISRCtrlFlowCntInv:
 150  0009 0000          	ds.b	2
 151  000b               _CtrlFlowCntInv:
 152  000b 0000          	ds.b	2
 412                     ; 90 void FailSafe(uint16_t err_no)
 412                     ; 91 {
 414                     	switch	.text
 415  0000               _FailSafe:
 419                     ; 92   disableInterrupts();
 422  0000 9b            sim
 424                     ; 97     LEDS_PORT->ODR |= (led);
 428  0001 72105005      	bset	20485,#0
 429  0005               L522:
 430                     ; 106   IWDG->KR = 0xAAu;
 433  0005 35aa50e0      	mov	20704,#170
 434                     ; 112   WWDG->CR = cnt | WWDG_CR_WDGA;
 437  0009 35ff50d1      	mov	20689,#255
 438                     ; 113   WWDG->WR = win;
 440  000d 357f50d2      	mov	20690,#127
 441  0011 20f2          	jra	L522
 481                     ; 137 void STL_StartUp (void)
 481                     ; 138 {
 482                     	switch	.text
 483  0013               _STL_StartUp:
 487                     ; 141   #endif /* STL_VERBOSE_FAILSAFE */
 490                     xref __stack
 492                     ; 142 
 495  0013 ae0000        ldw x,#__stack
 497                     ; 143 /* error code is passed as parameter to FailSafe routine */
 500  0016 94            ldw sp,x
 502                     ; 150   */
 504  0017 c650c6        	ld	a,20678
 505  001a a4e7          	and	a,#231
 506  001c c750c6        	ld	20678,a
 507                     ; 78     LEDS_PORT->DDR |= (ALL_LEDs);
 510  001f c65007        	ld	a,20487
 511  0022 aa0f          	or	a,#15
 512  0024 c75007        	ld	20487,a
 513                     ; 79     LEDS_PORT->CR1 |= (ALL_LEDs);
 515  0027 c65008        	ld	a,20488
 516  002a aa0f          	or	a,#15
 517  002c c75008        	ld	20488,a
 518                     ; 80     LEDS_PORT->ODR &= ~(ALL_LEDs);
 520  002f c65005        	ld	a,20485
 521  0032 a4f0          	and	a,#240
 522  0034 c75005        	ld	20485,a
 523                     ; 85     LEDS_PORT->DDR |= TEST_PIN;
 526  0037 721a5007      	bset	20487,#5
 527                     ; 86     LEDS_PORT->CR1 |= TEST_PIN;
 529  003b 721a5008      	bset	20488,#5
 530                     ; 87     LEDS_PORT->ODR &= ~TEST_PIN;
 532  003f 721b5005      	bres	20485,#5
 533                     ; 203   CtrlFlowCnt = CPU_POR_CALLER;
 535  0043 ae0002        	ldw	x,#2
 536  0046 cf000b        	ldw	_CtrlFlowCnt,x
 537                     ; 204   CtrlFlowCntInv = 0xFFFFu;
 539  0049 aeffff        	ldw	x,#65535
 540  004c cf000b        	ldw	_CtrlFlowCntInv,x
 541                     ; 207     if (STL_StartUpCPUTest() != CPUTEST_SUCCESS)
 543  004f cd0000        	call	_STL_StartUpCPUTest
 545  0052 a101          	cp	a,#1
 546  0054 2705          	jreq	L133
 547                     ; 144   FailSafe(err_code);
 550  0056 5f            	clrw	x
 551  0057 ada7          	call	_FailSafe
 553  0059 2009          	jra	L333
 554  005b               L133:
 555                     ; 214       CtrlFlowCntInv -= CPU_POR_CALLER;
 557  005b ce000b        	ldw	x,_CtrlFlowCntInv
 558  005e 1d0002        	subw	x,#2
 559  0061 cf000b        	ldw	_CtrlFlowCntInv,x
 560  0064               L333:
 561                     ; 224   IWDG->KR = 0xCCu;     /* IWDG Enable */
 563  0064 35cc50e0      	mov	20704,#204
 564                     ; 225   IWDG->KR = 0x55u;     /* IWDG WriteAccess Enable */
 566  0068 355550e0      	mov	20704,#85
 567                     ; 226   IWDG->PR = 0x04u;     /* IWDG Prescaler to 64 */
 569  006c 350450e1      	mov	20705,#4
 570                     ; 227   IWDG->RLR = 0xFFu;    /* set a 255ms timeout period */
 572  0070 35ff50e2      	mov	20706,#255
 573                     ; 106   IWDG->KR = 0xAAu;
 576  0074 35aa50e0      	mov	20704,#170
 577                     ; 231     CtrlFlowCnt += WDG_TEST_CALLER;
 579  0078 ce000b        	ldw	x,_CtrlFlowCnt
 580  007b 1c0005        	addw	x,#5
 581  007e cf000b        	ldw	_CtrlFlowCnt,x
 582                     ; 232     STL_WDGSelfTest();
 584  0081 cd0164        	call	_STL_WDGSelfTest
 586                     ; 233     CtrlFlowCntInv -= WDG_TEST_CALLER;
 588  0084 ce000b        	ldw	x,_CtrlFlowCntInv
 589  0087 1d0005        	subw	x,#5
 590  008a cf000b        	ldw	_CtrlFlowCntInv,x
 591                     ; 97     LEDS_PORT->ODR |= (led);
 594  008d 72125005      	bset	20485,#1
 595                     ; 245     CtrlFlowCnt += CRC16_TEST_CALLER;
 597  0091 ce000b        	ldw	x,_CtrlFlowCnt
 598  0094 1c0007        	addw	x,#7
 599  0097 cf000b        	ldw	_CtrlFlowCnt,x
 600                     ; 251             _classb_checksum160(); /* flash check result is executed */
 602  009a cd0000        	call	__classb_checksum160
 604                     ; 300         CtrlFlowCntInv -= CRC16_TEST_CALLER;
 606  009d ce000b        	ldw	x,_CtrlFlowCntInv
 607  00a0 1d0007        	subw	x,#7
 608  00a3 cf000b        	ldw	_CtrlFlowCntInv,x
 609                     ; 102     LEDS_PORT->ODR &= ~(led);
 612  00a6 72135005      	bres	20485,#1
 613                     ; 318   if (((CtrlFlowCnt ^ CtrlFlowCntInv) != 0xFFFFu)
 613                     ; 319   || (CtrlFlowCnt != CHECKPOINT1 ))
 615  00aa ce000b        	ldw	x,_CtrlFlowCnt
 616  00ad 01            	rrwa	x,a
 617  00ae c8000c        	xor	a,_CtrlFlowCntInv+1
 618  00b1 01            	rrwa	x,a
 619  00b2 c8000b        	xor	a,_CtrlFlowCntInv
 620  00b5 01            	rrwa	x,a
 621  00b6 a3ffff        	cpw	x,#65535
 622  00b9 2608          	jrne	L733
 624  00bb ce000b        	ldw	x,_CtrlFlowCnt
 625  00be a30011        	cpw	x,#17
 626  00c1 2706          	jreq	L143
 627  00c3               L733:
 628                     ; 144   FailSafe(err_code);
 631  00c3 ae0002        	ldw	x,#2
 632  00c6 cd0000        	call	_FailSafe
 634  00c9               L143:
 635                     ; 97     LEDS_PORT->ODR |= (led);
 638  00c9 72145005      	bset	20485,#2
 639                     ; 339     if (STL_FullRamMarchC() != FULL_RAM_OK)
 641  00cd cd0000        	call	_STL_FullRamMarchC
 643  00d0 a101          	cp	a,#1
 644  00d2 2706          	jreq	L343
 645                     ; 144   FailSafe(err_code);
 648  00d4 ae0003        	ldw	x,#3
 649  00d7 cd0000        	call	_FailSafe
 651  00da               L343:
 652                     ; 102     LEDS_PORT->ODR &= ~(led);
 655  00da 72155005      	bres	20485,#2
 656                     ; 97     LEDS_PORT->ODR |= (led);
 659  00de 72125005      	bset	20485,#1
 660                     ; 367     CtrlFlowCnt += CLOCK_POR_CALLER;
 662  00e2 ce000b        	ldw	x,_CtrlFlowCnt
 663  00e5 1c000b        	addw	x,#11
 664  00e8 cf000b        	ldw	_CtrlFlowCnt,x
 665                     ; 369     switch ( STL_ClockStartUpTest() )
 667  00eb cd0000        	call	_STL_ClockStartUpTest
 670                     ; 401         break;
 671  00ee 4d            	tnz	a
 672  00ef 271d          	jreq	L762
 673  00f1 4a            	dec	a
 674  00f2 2722          	jreq	L372
 675  00f4 4a            	dec	a
 676  00f5 2727          	jreq	L772
 677  00f7 4a            	dec	a
 678  00f8 2734          	jreq	L703
 679  00fa 4a            	dec	a
 680  00fb 2709          	jreq	L362
 681  00fd 4a            	dec	a
 682  00fe 2726          	jreq	L303
 683  0100 a004          	sub	a,#4
 684  0102 2730          	jreq	L743
 685  0104 2028          	jra	L703
 686  0106               L362:
 687                     ; 144   FailSafe(err_code);
 690  0106 ae0004        	ldw	x,#4
 691  0109 cd0000        	call	_FailSafe
 693  010c 2026          	jra	L743
 694  010e               L762:
 698  010e ae0005        	ldw	x,#5
 699  0111 cd0000        	call	_FailSafe
 701  0114 201e          	jra	L743
 702  0116               L372:
 706  0116 ae0006        	ldw	x,#6
 707  0119 cd0000        	call	_FailSafe
 709  011c 2016          	jra	L743
 710  011e               L772:
 714  011e ae0007        	ldw	x,#7
 715  0121 cd0000        	call	_FailSafe
 717  0124 200e          	jra	L743
 718  0126               L303:
 722  0126 ae0008        	ldw	x,#8
 723  0129 cd0000        	call	_FailSafe
 725  012c 2006          	jra	L743
 726  012e               L703:
 730  012e ae0009        	ldw	x,#9
 731  0131 cd0000        	call	_FailSafe
 733  0134               L743:
 734                     ; 403     CtrlFlowCntInv -= CLOCK_POR_CALLER;
 736  0134 ce000b        	ldw	x,_CtrlFlowCntInv
 737  0137 1d000b        	subw	x,#11
 738  013a cf000b        	ldw	_CtrlFlowCntInv,x
 739                     ; 102     LEDS_PORT->ODR &= ~(led);
 742  013d 72135005      	bres	20485,#1
 743                     ; 415   if (((CtrlFlowCnt ^ CtrlFlowCntInv) != 0xFFFFu) || (CtrlFlowCnt != CHECKPOINT2))
 745  0141 ce000b        	ldw	x,_CtrlFlowCnt
 746  0144 01            	rrwa	x,a
 747  0145 c8000c        	xor	a,_CtrlFlowCntInv+1
 748  0148 01            	rrwa	x,a
 749  0149 c8000b        	xor	a,_CtrlFlowCntInv
 750  014c 01            	rrwa	x,a
 751  014d a3ffff        	cpw	x,#65535
 752  0150 2608          	jrne	L353
 754  0152 ce000b        	ldw	x,_CtrlFlowCnt
 755  0155 a30089        	cpw	x,#137
 756  0158 2706          	jreq	L153
 757  015a               L353:
 758                     ; 144   FailSafe(err_code);
 761  015a ae000a        	ldw	x,#10
 762  015d cd0000        	call	_FailSafe
 764  0160               L153:
 765                     ; 423   GotoCompilerStartUp()
 768                     xdef __stext
 769  0160 cc0000        jp __stext
 771                     ; 424 }
 774  0163 81            	ret
 799                     ; 581 void STL_WDGSelfTest(void)
 799                     ; 582 {
 800                     	switch	.text
 801  0164               _STL_WDGSelfTest:
 805                     ; 608   if ((RST->SR & RST_SR_IWDGF) == 0u)
 807  0164 c650b3        	ld	a,20659
 808  0167 a502          	bcp	a,#2
 809  0169 261a          	jrne	L173
 810                     ; 615       RST->SR |= RST_SR_WWDGF;   /* Re-test always WWDG if applied */
 812  016b 721050b3      	bset	20659,#0
 813                     ; 618     IWDG->KR = (uint8_t)0xCCu;    /* IWDG Enable */
 815  016f 35cc50e0      	mov	20704,#204
 816                     ; 619     IWDG->KR = (uint8_t)0x55u;    /* IWDG WriteAccess Enable */
 818  0173 355550e0      	mov	20704,#85
 819                     ; 620     IWDG->PR = (uint8_t)0;        /* IWDG Prescaler to 4 */
 821  0177 725f50e1      	clr	20705
 822                     ; 621     IWDG->RLR = (uint8_t)0;       /* set the shortest timeout period */
 824  017b 725f50e2      	clr	20706
 825                     ; 106   IWDG->KR = 0xAAu;
 828  017f 35aa50e0      	mov	20704,#170
 829  0183               L373:
 831  0183 20fe          	jra	L373
 832  0185               L173:
 833                     ; 631       if ((RST->SR & RST_SR_WWDGF) == 0u)
 835  0185 c650b3        	ld	a,20659
 836  0188 a501          	bcp	a,#1
 837  018a 260a          	jrne	L104
 838                     ; 112   WWDG->CR = cnt | WWDG_CR_WDGA;
 841  018c 35c050d1      	mov	20689,#192
 842                     ; 113   WWDG->WR = win;
 844  0190 357f50d2      	mov	20690,#127
 845  0194               L304:
 847  0194 20fe          	jra	L304
 848  0196               L104:
 849                     ; 644         RST->SR |= (uint8_t)(RST_SR_WWDGF | RST_SR_IWDGF);   /* clear both flags */
 851  0196 c650b3        	ld	a,20659
 852  0199 aa03          	or	a,#3
 853  019b c750b3        	ld	20659,a
 854  019e               L773:
 855                     ; 660 }
 858  019e 81            	ret
 871                     	xref	__classb_checksum160
 872                     	xdef	_CurrentDescInv
 873                     	xdef	_CurrentCrc16Inv
 874                     	xdef	_p_RunCrc16ChkInv
 875                     	xdef	_LastCtrlFlowCntInv
 876                     	xdef	_TimeBaseFlagInv
 877                     	xdef	_TickCounterInv
 878                     	xdef	_ISRCtrlFlowCntInv
 879                     	xdef	_CtrlFlowCntInv
 880                     	xdef	_aStackOverFlowPtrn
 881                     	xdef	_CRCBlockIndex
 882                     	xdef	_CurrentDesc
 883                     	xdef	_CurrentCrc16
 884                     	xdef	_p_RunCrc16Chk
 885                     	xdef	_LastCtrlFlowCnt
 886                     	xdef	_TimeBaseFlag
 887                     	xdef	_TickCounter
 888                     	xdef	_ISRCtrlFlowCnt
 889                     	xdef	_CtrlFlowCnt
 890                     	xdef	_p_RunTimeRamChkInv
 891                     	xdef	_p_RunTimeRamChk
 892                     	xdef	_aRunTimeRamBuf
 893                     	xdef	_FailSafe
 894                     	xref	_STL_FullRamMarchC
 895                     	xref	_STL_ClockStartUpTest
 896                     	xref	_STL_StartUpCPUTest
 897                     	xdef	_STL_WDGSelfTest
 898                     	xdef	_STL_StartUp
 917                     	end
