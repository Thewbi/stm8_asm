   1                     ; C Compiler for STM8 (COSMIC Software)
   2                     ; Parser V4.11.13 - 05 Feb 2019
   3                     ; Generator V4.4.9 - 06 Feb 2019
 147                     ; 80 void STL_InitRunTimeChecks(void)
 147                     ; 81 {
 149                     	switch	.text
 150  0000               _STL_InitRunTimeChecks:
 154                     ; 92     LEDS_PORT->ODR ^= TEST_PIN;
 157  0000 901a5005      	bcpl	20485,#5
 158                     ; 88   TickCounter = 0u;
 160  0004 5f            	clrw	x
 161  0005 cf0000        	ldw	_TickCounter,x
 162                     ; 89   TickCounterInv = 0xFFFFu;
 164  0008 aeffff        	ldw	x,#65535
 165  000b cf0000        	ldw	_TickCounterInv,x
 166                     ; 91   TimeBaseFlag = 0u;
 168  000e 725f0000      	clr	_TimeBaseFlag
 169                     ; 92   TimeBaseFlagInv = 0xFFu;
 171  0012 35ff0000      	mov	_TimeBaseFlagInv,#255
 172                     ; 94   LastCtrlFlowCnt = 0u;
 174  0016 5f            	clrw	x
 175  0017 cf0000        	ldw	_LastCtrlFlowCnt,x
 176                     ; 95   LastCtrlFlowCntInv = 0xFFFFu;
 178  001a aeffff        	ldw	x,#65535
 179  001d cf0000        	ldw	_LastCtrlFlowCntInv,x
 180                     ; 98   ISRCtrlFlowCnt = 0u;
 182  0020 5f            	clrw	x
 183  0021 cf0000        	ldw	_ISRCtrlFlowCnt,x
 184                     ; 99   ISRCtrlFlowCntInv = 0xFFFFu;
 186  0024 aeffff        	ldw	x,#65535
 187  0027 cf0000        	ldw	_ISRCtrlFlowCntInv,x
 188                     ; 102   CtrlFlowCnt = 0u;
 190  002a 5f            	clrw	x
 191  002b cf0000        	ldw	_CtrlFlowCnt,x
 192                     ; 103   CtrlFlowCntInv = 0xFFFFu;
 194  002e aeffff        	ldw	x,#65535
 195  0031 cf0000        	ldw	_CtrlFlowCntInv,x
 196                     ; 111     CtrlFlowCnt = RAM_MARCH_INIT_CALLER;
 198  0034 ae0025        	ldw	x,#37
 199  0037 cf0000        	ldw	_CtrlFlowCnt,x
 200                     ; 112     STL_TranspMarchInit();
 202  003a cd0000        	call	_STL_TranspMarchInit
 204                     ; 113     CtrlFlowCntInv -= RAM_MARCH_INIT_CALLER;
 206  003d ce0000        	ldw	x,_CtrlFlowCntInv
 207  0040 1d0025        	subw	x,#37
 208  0043 cf0000        	ldw	_CtrlFlowCntInv,x
 209                     ; 119       STL_FlashCrc16Init();
 211  0046 cd0000        	call	_STL_FlashCrc16Init
 213                     ; 127     CtrlFlowCnt += LSI_CHECK_INIT_CALLER;
 215  0049 ce0000        	ldw	x,_CtrlFlowCnt
 216  004c 1c0039        	addw	x,#57
 217  004f cf0000        	ldw	_CtrlFlowCnt,x
 218                     ; 129     if (STL_InitClock_Xcross_Measurement() == ERROR)
 220  0052 cd0000        	call	_STL_InitClock_Xcross_Measurement
 222  0055 4d            	tnz	a
 223  0056 2606          	jrne	L56
 224                     ; 144   FailSafe(err_code);
 227  0058 ae0010        	ldw	x,#16
 228  005b cd0000        	call	_FailSafe
 230  005e               L56:
 231                     ; 134     CtrlFlowCntInv -= LSI_CHECK_INIT_CALLER;
 233  005e ce0000        	ldw	x,_CtrlFlowCntInv
 234  0061 1d0039        	subw	x,#57
 235  0064 cf0000        	ldw	_CtrlFlowCntInv,x
 236                     ; 139     aStackOverFlowPtrn[0] = 0xEEu;
 238  0067 35ee0000      	mov	_aStackOverFlowPtrn,#238
 239                     ; 140     aStackOverFlowPtrn[1] = 0xBBu;
 241  006b 35bb0001      	mov	_aStackOverFlowPtrn+1,#187
 242                     ; 141     aStackOverFlowPtrn[2] = 0xDDu;
 244  006f 35dd0002      	mov	_aStackOverFlowPtrn+2,#221
 245                     ; 142     aStackOverFlowPtrn[3] = 0xCCu;
 247  0073 35cc0003      	mov	_aStackOverFlowPtrn+3,#204
 248                     ; 147     CtrlFlowCnt += TIM_BASE_INIT_CALLER;
 250  0077 ce0000        	ldw	x,_CtrlFlowCnt
 251  007a 1c001d        	addw	x,#29
 252  007d cf0000        	ldw	_CtrlFlowCnt,x
 253                     ; 148     STL_TimeBaseInit();
 255  0080 cd0288        	call	_STL_TimeBaseInit
 257                     ; 149     CtrlFlowCntInv -= TIM_BASE_INIT_CALLER;
 259  0083 ce0000        	ldw	x,_CtrlFlowCntInv
 260  0086 1d001d        	subw	x,#29
 261  0089 cf0000        	ldw	_CtrlFlowCntInv,x
 262                     ; 153   if (((CtrlFlowCnt ^ CtrlFlowCntInv) != 0xFFFFu)
 262                     ; 154      || (CtrlFlowCnt != CHECKPOINT_INIT ))
 264  008c ce0000        	ldw	x,_CtrlFlowCnt
 265  008f 01            	rrwa	x,a
 266  0090 c80001        	xor	a,_CtrlFlowCntInv+1
 267  0093 01            	rrwa	x,a
 268  0094 c80000        	xor	a,_CtrlFlowCntInv
 269  0097 01            	rrwa	x,a
 270  0098 a3ffff        	cpw	x,#65535
 271  009b 2608          	jrne	L17
 273  009d ce0000        	ldw	x,_CtrlFlowCnt
 274  00a0 a300da        	cpw	x,#218
 275  00a3 270b          	jreq	L76
 276  00a5               L17:
 277                     ; 144   FailSafe(err_code);
 280  00a5 ae0011        	ldw	x,#17
 281  00a8 cd0000        	call	_FailSafe
 283  00ab               L37:
 284                     ; 92     LEDS_PORT->ODR ^= TEST_PIN;
 287  00ab 901a5005      	bcpl	20485,#5
 288                     ; 171 }
 291  00af 81            	ret
 292  00b0               L76:
 293                     ; 164     CtrlFlowCnt = 0u;
 295  00b0 5f            	clrw	x
 296  00b1 cf0000        	ldw	_CtrlFlowCnt,x
 297                     ; 165     CtrlFlowCntInv = 0xFFFFu;
 299  00b4 aeffff        	ldw	x,#65535
 300  00b7 cf0000        	ldw	_CtrlFlowCntInv,x
 301  00ba 20ef          	jra	L37
 394                     ; 191 void STL_DoRunTimeChecks(void)
 394                     ; 192 {
 395                     	switch	.text
 396  00bc               _STL_DoRunTimeChecks:
 398  00bc 88            	push	a
 399       00000001      OFST:	set	1
 402                     ; 194   if (TimeBaseFlag == 0xAAu)
 404  00bd c60000        	ld	a,_TimeBaseFlag
 405  00c0 a1aa          	cp	a,#170
 406  00c2 2703          	jreq	L01
 407  00c4 cc023e        	jp	L512
 408  00c7               L01:
 409                     ; 107     LEDS_PORT->ODR ^= (led);
 412  00c7 90125005      	bcpl	20485,#1
 413                     ; 201     if ((TimeBaseFlag ^ TimeBaseFlagInv) == 0xFFu)
 415  00cb c60000        	ld	a,_TimeBaseFlag
 416  00ce c80000        	xor	a,_TimeBaseFlagInv
 417  00d1 a1ff          	cp	a,#255
 418  00d3 2703          	jreq	L21
 419  00d5 cc0220        	jp	L712
 420  00d8               L21:
 421                     ; 206       TimeBaseFlag = 0u;
 423  00d8 725f0000      	clr	_TimeBaseFlag
 424                     ; 212         CtrlFlowCnt += CPU_RUN_CALLER;
 426  00dc ce0000        	ldw	x,_CtrlFlowCnt
 427  00df 1c0009        	addw	x,#9
 428  00e2 cf0000        	ldw	_CtrlFlowCnt,x
 429                     ; 213         if (STL_RunTimeCPUTest() != CPUTEST_SUCCESS)
 431  00e5 cd0000        	call	_STL_RunTimeCPUTest
 433  00e8 a101          	cp	a,#1
 434  00ea 2708          	jreq	L122
 435                     ; 144   FailSafe(err_code);
 438  00ec ae0012        	ldw	x,#18
 439  00ef cd0000        	call	_FailSafe
 441  00f2 2009          	jra	L322
 442  00f4               L122:
 443                     ; 219           CtrlFlowCntInv -= CPU_RUN_CALLER;
 445  00f4 ce0000        	ldw	x,_CtrlFlowCntInv
 446  00f7 1d0009        	subw	x,#9
 447  00fa cf0000        	ldw	_CtrlFlowCntInv,x
 448  00fd               L322:
 449                     ; 227         CtrlFlowCnt += STACK_OVERFLOW_CALLER;
 451  00fd ce0000        	ldw	x,_CtrlFlowCnt
 452  0100 1c001d        	addw	x,#29
 453  0103 cf0000        	ldw	_CtrlFlowCnt,x
 454                     ; 228         if (STL_CheckStack() != SUCCESS)
 456  0106 cd0240        	call	_STL_CheckStack
 458  0109 a101          	cp	a,#1
 459  010b 2708          	jreq	L522
 460                     ; 144   FailSafe(err_code);
 463  010d ae0013        	ldw	x,#19
 464  0110 cd0000        	call	_FailSafe
 466  0113 2009          	jra	L722
 467  0115               L522:
 468                     ; 234           CtrlFlowCntInv -= STACK_OVERFLOW_CALLER;
 470  0115 ce0000        	ldw	x,_CtrlFlowCntInv
 471  0118 1d001d        	subw	x,#29
 472  011b cf0000        	ldw	_CtrlFlowCntInv,x
 473  011e               L722:
 474                     ; 242         CtrlFlowCnt += FREQ_TEST_CALLER;
 476  011e ce0000        	ldw	x,_CtrlFlowCnt
 477  0121 1c0015        	addw	x,#21
 478  0124 cf0000        	ldw	_CtrlFlowCnt,x
 479                     ; 244         switch (STL_ClockFreqTest())
 481  0127 cd0000        	call	_STL_ClockFreqTest
 484                     ; 269             break;
 485  012a 4d            	tnz	a
 486  012b 273c          	jreq	L121
 487  012d 4a            	dec	a
 488  012e 2739          	jreq	L121
 489  0130 4a            	dec	a
 490  0131 2736          	jreq	L121
 491  0133 4a            	dec	a
 492  0134 2710          	jreq	L301
 493  0136 4a            	dec	a
 494  0137 2720          	jreq	L111
 495  0139 4a            	dec	a
 496  013a 2715          	jreq	L501
 497  013c 4a            	dec	a
 498  013d 2722          	jreq	L511
 499  013f 4a            	dec	a
 500  0140 2727          	jreq	L121
 501  0142 a002          	sub	a,#2
 502  0144 2623          	jrne	L121
 503  0146               L301:
 504                     ; 246           case FREQ_OK:
 504                     ; 247           case TEST_ONGOING:    
 504                     ; 248             CtrlFlowCntInv -= FREQ_TEST_CALLER;
 506  0146 ce0000        	ldw	x,_CtrlFlowCntInv
 507  0149 1d0015        	subw	x,#21
 508  014c cf0000        	ldw	_CtrlFlowCntInv,x
 509                     ; 249             break;
 511  014f 201e          	jra	L332
 512  0151               L501:
 513                     ; 144   FailSafe(err_code);
 516  0151 ae0014        	ldw	x,#20
 517  0154 cd0000        	call	_FailSafe
 519  0157 2016          	jra	L332
 520  0159               L111:
 524  0159 ae0015        	ldw	x,#21
 525  015c cd0000        	call	_FailSafe
 527  015f 200e          	jra	L332
 528  0161               L511:
 532  0161 ae0016        	ldw	x,#22
 533  0164 cd0000        	call	_FailSafe
 535  0167 2006          	jra	L332
 536  0169               L121:
 540  0169 ae0017        	ldw	x,#23
 541  016c cd0000        	call	_FailSafe
 543  016f               L332:
 544                     ; 277         CtrlFlowCnt += FLASH_RUN_TEST_CALLER;
 546  016f ce0000        	ldw	x,_CtrlFlowCnt
 547  0172 1c002f        	addw	x,#47
 548  0175 cf0000        	ldw	_CtrlFlowCnt,x
 549                     ; 278         RomTest = STL_crc16Run();
 551  0178 cd0000        	call	_STL_crc16Run
 553  017b 6b01          	ld	(OFST+0,sp),a
 555                     ; 279         switch ( RomTest )
 557  017d 7b01          	ld	a,(OFST+0,sp)
 559                     ; 300             break;
 560  017f 4d            	tnz	a
 561  0180 270e          	jreq	L521
 562  0182 4a            	dec	a
 563  0183 2725          	jreq	L331
 564  0185 4a            	dec	a
 565  0186 2722          	jreq	L331
 566  0188 4a            	dec	a
 567  0189 271f          	jreq	L331
 568  018b 4a            	dec	a
 569  018c 270d          	jreq	L721
 570  018e 201a          	jra	L331
 571  0190               L521:
 572                     ; 281           case TEST_RUNNING:
 572                     ; 282             CtrlFlowCntInv -= FLASH_RUN_TEST_CALLER;
 574  0190 ce0000        	ldw	x,_CtrlFlowCntInv
 575  0193 1d002f        	subw	x,#47
 576  0196 cf0000        	ldw	_CtrlFlowCntInv,x
 577                     ; 283             break;
 579  0199 2015          	jra	L732
 580  019b               L721:
 581                     ; 107     LEDS_PORT->ODR ^= (led);
 584  019b 90125005      	bcpl	20485,#1
 585                     ; 292             CtrlFlowCntInv -= FLASH_RUN_TEST_CALLER;           
 587  019f ce0000        	ldw	x,_CtrlFlowCntInv
 588  01a2 1d002f        	subw	x,#47
 589  01a5 cf0000        	ldw	_CtrlFlowCntInv,x
 590                     ; 293             break;
 592  01a8 2006          	jra	L732
 593  01aa               L331:
 594                     ; 144   FailSafe(err_code);
 597  01aa ae0018        	ldw	x,#24
 598  01ad cd0000        	call	_FailSafe
 600  01b0               L732:
 601                     ; 310       if (((CtrlFlowCnt ^ CtrlFlowCntInv) == 0xFFFFu)
 601                     ; 311       && ((LastCtrlFlowCnt ^ LastCtrlFlowCntInv) == 0xFFFFu))
 603  01b0 ce0000        	ldw	x,_CtrlFlowCnt
 604  01b3 01            	rrwa	x,a
 605  01b4 c80001        	xor	a,_CtrlFlowCntInv+1
 606  01b7 01            	rrwa	x,a
 607  01b8 c80000        	xor	a,_CtrlFlowCntInv
 608  01bb 01            	rrwa	x,a
 609  01bc a3ffff        	cpw	x,#65535
 610  01bf 2657          	jrne	L142
 612  01c1 ce0000        	ldw	x,_LastCtrlFlowCnt
 613  01c4 01            	rrwa	x,a
 614  01c5 c80001        	xor	a,_LastCtrlFlowCntInv+1
 615  01c8 01            	rrwa	x,a
 616  01c9 c80000        	xor	a,_LastCtrlFlowCntInv
 617  01cc 01            	rrwa	x,a
 618  01cd a3ffff        	cpw	x,#65535
 619  01d0 2646          	jrne	L142
 620                     ; 313         if (RomTest == TEST_OK)
 622  01d2 7b01          	ld	a,(OFST+0,sp)
 623  01d4 a104          	cp	a,#4
 624  01d6 2620          	jrne	L342
 625                     ; 320           if ((CtrlFlowCnt - LastCtrlFlowCnt) == (LAST_DELTA_MAIN))
 627  01d8 ce0000        	ldw	x,_CtrlFlowCnt
 628  01db 72b00000      	subw	x,_LastCtrlFlowCnt
 629  01df a300ef        	cpw	x,#239
 630  01e2 260c          	jrne	L542
 631                     ; 323             CtrlFlowCnt = 0u;
 633  01e4 5f            	clrw	x
 634  01e5 cf0000        	ldw	_CtrlFlowCnt,x
 635                     ; 324             CtrlFlowCntInv = 0xFFFFu;
 637  01e8 aeffff        	ldw	x,#65535
 638  01eb cf0000        	ldw	_CtrlFlowCntInv,x
 640  01ee 201a          	jra	L152
 641  01f0               L542:
 642                     ; 144   FailSafe(err_code);
 645  01f0 ae0019        	ldw	x,#25
 646  01f3 cd0000        	call	_FailSafe
 648  01f6 2012          	jra	L152
 649  01f8               L342:
 650                     ; 333           if ((CtrlFlowCnt - LastCtrlFlowCnt) != DELTA_MAIN)
 652  01f8 ce0000        	ldw	x,_CtrlFlowCnt
 653  01fb 72b00000      	subw	x,_LastCtrlFlowCnt
 654  01ff a300ef        	cpw	x,#239
 655  0202 2706          	jreq	L152
 656                     ; 144   FailSafe(err_code);
 659  0204 ae001a        	ldw	x,#26
 660  0207 cd0000        	call	_FailSafe
 662  020a               L152:
 663                     ; 339         LastCtrlFlowCnt = CtrlFlowCnt;
 665  020a ce0000        	ldw	x,_CtrlFlowCnt
 666  020d cf0000        	ldw	_LastCtrlFlowCnt,x
 667                     ; 340         LastCtrlFlowCntInv = CtrlFlowCntInv;
 669  0210 ce0000        	ldw	x,_CtrlFlowCntInv
 670  0213 cf0000        	ldw	_LastCtrlFlowCntInv,x
 672  0216 200e          	jra	L752
 673  0218               L142:
 674                     ; 144   FailSafe(err_code);
 677  0218 ae001b        	ldw	x,#27
 678  021b cd0000        	call	_FailSafe
 680  021e 2006          	jra	L752
 681  0220               L712:
 685  0220 ae001c        	ldw	x,#28
 686  0223 cd0000        	call	_FailSafe
 688  0226               L752:
 689                     ; 92     LEDS_PORT->ODR ^= TEST_PIN;
 692  0226 901a5005      	bcpl	20485,#5
 693                     ; 112   WWDG->CR = cnt | WWDG_CR_WDGA;
 696  022a 35ff50d1      	mov	20689,#255
 697                     ; 113   WWDG->WR = win;
 699  022e 357f50d2      	mov	20690,#127
 700                     ; 106   IWDG->KR = 0xAAu;
 703  0232 35aa50e0      	mov	20704,#170
 704                     ; 92     LEDS_PORT->ODR ^= TEST_PIN;
 707  0236 901a5005      	bcpl	20485,#5
 708                     ; 107     LEDS_PORT->ODR ^= (led);
 711  023a 90125005      	bcpl	20485,#1
 712  023e               L512:
 713                     ; 367 }
 716  023e 84            	pop	a
 717  023f 81            	ret
 776                     ; 381 ErrorStatus STL_CheckStack(void)
 776                     ; 382 {
 777                     	switch	.text
 778  0240               _STL_CheckStack:
 780  0240 88            	push	a
 781       00000001      OFST:	set	1
 784                     ; 383     ErrorStatus result = ERROR;
 786                     ; 385   CtrlFlowCnt += STACK_OVERFLOW_CALLEE;
 788  0241 ce0000        	ldw	x,_CtrlFlowCnt
 789  0244 1c001f        	addw	x,#31
 790  0247 cf0000        	ldw	_CtrlFlowCnt,x
 791                     ; 387   if (aStackOverFlowPtrn[0] != 0xEEu)
 793  024a c60000        	ld	a,_aStackOverFlowPtrn
 794  024d a1ee          	cp	a,#238
 795  024f 2704          	jreq	L703
 796                     ; 389     result = ERROR;
 798  0251 0f01          	clr	(OFST+0,sp)
 801  0253 2025          	jra	L113
 802  0255               L703:
 803                     ; 393     if (aStackOverFlowPtrn[1] != 0xBBu)
 805  0255 c60001        	ld	a,_aStackOverFlowPtrn+1
 806  0258 a1bb          	cp	a,#187
 807  025a 2704          	jreq	L313
 808                     ; 395       result = ERROR;
 810  025c 0f01          	clr	(OFST+0,sp)
 813  025e 201a          	jra	L113
 814  0260               L313:
 815                     ; 399       if (aStackOverFlowPtrn[2] != 0xDDu)
 817  0260 c60002        	ld	a,_aStackOverFlowPtrn+2
 818  0263 a1dd          	cp	a,#221
 819  0265 2704          	jreq	L713
 820                     ; 401         result = ERROR;
 822  0267 0f01          	clr	(OFST+0,sp)
 825  0269 200f          	jra	L113
 826  026b               L713:
 827                     ; 405         if (aStackOverFlowPtrn[3] != 0xCCu)
 829  026b c60003        	ld	a,_aStackOverFlowPtrn+3
 830  026e a1cc          	cp	a,#204
 831  0270 2704          	jreq	L323
 832                     ; 407           result = ERROR;
 834  0272 0f01          	clr	(OFST+0,sp)
 837  0274 2004          	jra	L113
 838  0276               L323:
 839                     ; 411           result = SUCCESS;
 841  0276 a601          	ld	a,#1
 842  0278 6b01          	ld	(OFST+0,sp),a
 844  027a               L113:
 845                     ; 417   CtrlFlowCntInv -= STACK_OVERFLOW_CALLEE;
 847  027a ce0000        	ldw	x,_CtrlFlowCntInv
 848  027d 1d001f        	subw	x,#31
 849  0280 cf0000        	ldw	_CtrlFlowCntInv,x
 850                     ; 419   return (result);
 852  0283 7b01          	ld	a,(OFST+0,sp)
 855  0285 5b01          	addw	sp,#1
 856  0287 81            	ret
 881                     ; 435 void STL_TimeBaseInit(void)
 881                     ; 436 {
 882                     	switch	.text
 883  0288               _STL_TimeBaseInit:
 887                     ; 437   CtrlFlowCnt += TIM_BASE_INIT_CALLEE;
 889  0288 ce0000        	ldw	x,_CtrlFlowCnt
 890  028b 1c001f        	addw	x,#31
 891  028e cf0000        	ldw	_CtrlFlowCnt,x
 892                     ; 453     CLK->PCKENR1 |= CLK_PCKENR1_TIM4;      /* enable clock to TIM4 */
 894  0291 721850c7      	bset	20679,#4
 895                     ; 457   TIM4->PSCR = (uint8_t)(7u & TIM4_PSCR_PSC); /* prescaler syst clock/128 */
 897  0295 35075345      	mov	21317,#7
 898                     ; 459     TIM4->ARR = (uint8_t)187;	         /* auto reload gives 1ms period @ 187,5kHz (24MHz/128) */
 900  0299 35bb5346      	mov	21318,#187
 901                     ; 463   TIM4->IER |= TIM4_IER_UIE;             /* enable TIM4 update interrupt */
 903  029d 72105341      	bset	21313,#0
 904                     ; 464   ITC->ISPR6 |= (uint8_t)(1u << 6);           /* set the highest priority level for TIM4 due to RAM transparent test */
 906  02a1 721c7f75      	bset	32629,#6
 907                     ; 465   TIM4->CR1 |= TIM4_CR1_CEN;             /* counter enable */
 909  02a5 72105340      	bset	21312,#0
 910                     ; 468   CtrlFlowCntInv -= TIM_BASE_INIT_CALLEE;
 912  02a9 ce0000        	ldw	x,_CtrlFlowCntInv
 913  02ac 1d001f        	subw	x,#31
 914  02af cf0000        	ldw	_CtrlFlowCntInv,x
 915                     ; 469 }
 918  02b2 81            	ret
 942                     	switch	.ubsct
 943  0000               _debug_count:
 944  0000 0000          	ds.b	2
 945                     	xdef	_debug_count
 946                     	xdef	_STL_TimeBaseInit
 947                     	xdef	_STL_CheckStack
 948                     	xref	_LastCtrlFlowCntInv
 949                     	xref	_TimeBaseFlagInv
 950                     	xref	_TickCounterInv
 951                     	xref	_ISRCtrlFlowCntInv
 952                     	xref	_CtrlFlowCntInv
 953                     	xref	_aStackOverFlowPtrn
 954                     	xref	_LastCtrlFlowCnt
 955                     	xref	_TimeBaseFlag
 956                     	xref	_TickCounter
 957                     	xref	_ISRCtrlFlowCnt
 958                     	xref	_CtrlFlowCnt
 959                     	xref	_FailSafe
 960                     	xref	_STL_TranspMarchInit
 961                     	xref	_STL_crc16Run
 962                     	xref	_STL_FlashCrc16Init
 963                     	xref	_STL_ClockFreqTest
 964                     	xref	_STL_InitClock_Xcross_Measurement
 965                     	xref	_STL_RunTimeCPUTest
 966                     	xdef	_STL_DoRunTimeChecks
 967                     	xdef	_STL_InitRunTimeChecks
 987                     	end
