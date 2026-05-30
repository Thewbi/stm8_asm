   1                     ; C Compiler for STM8 (COSMIC Software)
   2                     ; Parser V4.11.13 - 05 Feb 2019
   3                     ; Generator V4.4.9 - 06 Feb 2019
 247                     ; 73 ClockStatus STL_ClockStartUpTest(void)
 247                     ; 74 {
 249                     	switch	.text
 250  0000               _STL_ClockStartUpTest:
 252  0000 5205          	subw	sp,#5
 253       00000005      OFST:	set	5
 256                     ; 75   ClockStatus clck_sts = TEST_ONGOING; /* In case of unexpected exit */
 258  0002 a603          	ld	a,#3
 259  0004 6b05          	ld	(OFST+0,sp),a
 261                     ; 79   CtrlFlowCnt += CLOCK_POR_CALLEE;
 263  0006 ce0000        	ldw	x,_CtrlFlowCnt
 264  0009 1c000d        	addw	x,#13
 265  000c cf0000        	ldw	_CtrlFlowCnt,x
 266                     ; 82   if (STL_LSIinit() != SUCCESS)
 268  000f cd0100        	call	_STL_LSIinit
 270  0012 a101          	cp	a,#1
 271  0014 2702          	jreq	L321
 272                     ; 84     clck_sts = LSI_START_FAIL;
 274  0016 0f05          	clr	(OFST+0,sp)
 276  0018               L321:
 277                     ; 86   if (clck_sts == TEST_ONGOING)
 279  0018 7b05          	ld	a,(OFST+0,sp)
 280  001a a103          	cp	a,#3
 281  001c 2703          	jreq	L6
 282  001e cc00b3        	jp	L521
 283  0021               L6:
 284                     ; 89     if (STL_InitClock_Xcross_Measurement() == ERROR)
 286  0021 cd01c3        	call	_STL_InitClock_Xcross_Measurement
 288  0024 4d            	tnz	a
 289  0025 2606          	jrne	L721
 290                     ; 91       clck_sts = XCROSS_CONFIG_FAIL;
 292  0027 a608          	ld	a,#8
 293  0029 6b05          	ld	(OFST+0,sp),a
 296  002b 202d          	jra	L131
 297  002d               L721:
 298                     ; 96       lsi_period = STL_MeasureLSIPeriod();
 300  002d cd0239        	call	_STL_MeasureLSIPeriod
 302  0030 1f01          	ldw	(OFST-4,sp),x
 304                     ; 97       expected_value = calc_captured_value();
 306  0032 cd018b        	call	_calc_captured_value
 308  0035 1f03          	ldw	(OFST-2,sp),x
 310                     ; 100       if (lsi_period < (expected_value * 4u / 5u))
 312  0037 1e03          	ldw	x,(OFST-2,sp)
 313  0039 58            	sllw	x
 314  003a 58            	sllw	x
 315  003b a605          	ld	a,#5
 316  003d 62            	div	x,a
 317  003e 1301          	cpw	x,(OFST-4,sp)
 318  0040 2306          	jrule	L331
 319                     ; 103         clck_sts = HSI_SOURCE_FAIL;
 321  0042 a604          	ld	a,#4
 322  0044 6b05          	ld	(OFST+0,sp),a
 325  0046 2012          	jra	L131
 326  0048               L331:
 327                     ; 105       else if (lsi_period > (expected_value * 6u / 5u))
 329  0048 1e03          	ldw	x,(OFST-2,sp)
 330  004a a606          	ld	a,#6
 331  004c cd0000        	call	c_bmulx
 333  004f a605          	ld	a,#5
 334  0051 62            	div	x,a
 335  0052 1301          	cpw	x,(OFST-4,sp)
 336  0054 2404          	jruge	L131
 337                     ; 108         clck_sts = HSI_SOURCE_FAIL;
 339  0056 a604          	ld	a,#4
 340  0058 6b05          	ld	(OFST+0,sp),a
 342  005a               L131:
 343                     ; 120     if (clck_sts == TEST_ONGOING)
 345  005a 7b05          	ld	a,(OFST+0,sp)
 346  005c a103          	cp	a,#3
 347  005e 2619          	jrne	L141
 348                     ; 122       if (STL_HSE_CSSinit() != SUCCESS)
 350  0060 cd0140        	call	_STL_HSE_CSSinit
 352  0063 a101          	cp	a,#1
 353  0065 2706          	jreq	L341
 354                     ; 124         clck_sts = HSE_START_FAIL;
 356  0067 a601          	ld	a,#1
 357  0069 6b05          	ld	(OFST+0,sp),a
 360  006b 200c          	jra	L141
 361  006d               L341:
 362                     ; 129         if (switch_clock_system(to_HSE) != SUCCESS)
 364  006d a6b4          	ld	a,#180
 365  006f ad50          	call	_switch_clock_system
 367  0071 a101          	cp	a,#1
 368  0073 2704          	jreq	L141
 369                     ; 131           clck_sts = HSI_HSE_SWITCH_FAIL;
 371  0075 a602          	ld	a,#2
 372  0077 6b05          	ld	(OFST+0,sp),a
 374  0079               L141:
 375                     ; 140     if (clck_sts == TEST_ONGOING)
 377  0079 7b05          	ld	a,(OFST+0,sp)
 378  007b a103          	cp	a,#3
 379  007d 2634          	jrne	L521
 380                     ; 143       lsi_period = STL_MeasureLSIPeriod();
 382  007f cd0239        	call	_STL_MeasureLSIPeriod
 384  0082 1f01          	ldw	(OFST-4,sp),x
 386                     ; 144       expected_value = calc_captured_value();
 388  0084 cd018b        	call	_calc_captured_value
 390  0087 1f03          	ldw	(OFST-2,sp),x
 392                     ; 147       if (lsi_period < (expected_value * 3u / 4u))
 394  0089 1e03          	ldw	x,(OFST-2,sp)
 395  008b a603          	ld	a,#3
 396  008d cd0000        	call	c_bmulx
 398  0090 54            	srlw	x
 399  0091 54            	srlw	x
 400  0092 1301          	cpw	x,(OFST-4,sp)
 401  0094 2306          	jrule	L351
 402                     ; 150         clck_sts = EXT_SOURCE_FAIL;
 404  0096 a605          	ld	a,#5
 405  0098 6b05          	ld	(OFST+0,sp),a
 408  009a 2017          	jra	L521
 409  009c               L351:
 410                     ; 152       else if (lsi_period > (expected_value * 5u / 4u))
 412  009c 1e03          	ldw	x,(OFST-2,sp)
 413  009e a605          	ld	a,#5
 414  00a0 cd0000        	call	c_bmulx
 416  00a3 54            	srlw	x
 417  00a4 54            	srlw	x
 418  00a5 1301          	cpw	x,(OFST-4,sp)
 419  00a7 2406          	jruge	L751
 420                     ; 155         clck_sts = EXT_SOURCE_FAIL;
 422  00a9 a605          	ld	a,#5
 423  00ab 6b05          	ld	(OFST+0,sp),a
 426  00ad 2004          	jra	L521
 427  00af               L751:
 428                     ; 159         clck_sts = FREQ_OK;   /* Crystal or Resonator started correctly, with expected frequency */
 430  00af a609          	ld	a,#9
 431  00b1 6b05          	ld	(OFST+0,sp),a
 433  00b3               L521:
 434                     ; 165   CtrlFlowCntInv -= CLOCK_POR_CALLEE;
 436  00b3 ce0000        	ldw	x,_CtrlFlowCntInv
 437  00b6 1d000d        	subw	x,#13
 438  00b9 cf0000        	ldw	_CtrlFlowCntInv,x
 439                     ; 166   return(clck_sts);
 441  00bc 7b05          	ld	a,(OFST+0,sp)
 444  00be 5b05          	addw	sp,#5
 445  00c0 81            	ret
 520                     ; 182 ErrorStatus switch_clock_system(uint8_t clck)
 520                     ; 183 {
 521                     	switch	.text
 522  00c1               _switch_clock_system:
 524  00c1 88            	push	a
 525  00c2 5203          	subw	sp,#3
 526       00000003      OFST:	set	3
 529                     ; 184   uint16_t time_out = CLK_SWITCH_TIMEOUT;
 531  00c4 ae0491        	ldw	x,#1169
 532  00c7 1f02          	ldw	(OFST-1,sp),x
 534                     ; 185   ErrorStatus result = SUCCESS;
 536  00c9 a601          	ld	a,#1
 537  00cb 6b01          	ld	(OFST-2,sp),a
 539                     ; 187   if (CLK->SWR != clck)
 541  00cd c650c4        	ld	a,20676
 542  00d0 1104          	cp	a,(OFST+1,sp)
 543  00d2 2727          	jreq	L122
 544                     ; 189     CLK->SWCR &= (uint8_t)(~CLK_SWCR_SWIF);			    /* clear SWIF flag */
 546  00d4 721750c5      	bres	20677,#3
 547                     ; 190     CLK->SWCR |= CLK_SWCR_SWEN;	                /* enable clock switching control */
 549  00d8 721250c5      	bset	20677,#1
 550                     ; 191     CLK->SWR = clck;										        /* initiate automatic switch mode */
 552  00dc 7b04          	ld	a,(OFST+1,sp)
 553  00de c750c4        	ld	20676,a
 555  00e1 2007          	jra	L722
 556  00e3               L322:
 557                     ; 198         --time_out;
 559  00e3 1e02          	ldw	x,(OFST-1,sp)
 560  00e5 1d0001        	subw	x,#1
 561  00e8 1f02          	ldw	(OFST-1,sp),x
 563  00ea               L722:
 564                     ; 195       while (((CLK->SWCR & CLK_SWCR_SWIF) == 0u)  &&  (time_out != 0u))
 566  00ea c650c5        	ld	a,20677
 567  00ed a508          	bcp	a,#8
 568  00ef 2604          	jrne	L332
 570  00f1 1e02          	ldw	x,(OFST-1,sp)
 571  00f3 26ee          	jrne	L322
 572  00f5               L332:
 573                     ; 200       if (time_out == 0u)
 575  00f5 1e02          	ldw	x,(OFST-1,sp)
 576  00f7 2602          	jrne	L122
 577                     ; 202         result =  ERROR;
 579  00f9 0f01          	clr	(OFST-2,sp)
 581  00fb               L122:
 582                     ; 205   return result;
 584  00fb 7b01          	ld	a,(OFST-2,sp)
 587  00fd 5b04          	addw	sp,#4
 588  00ff 81            	ret
 635                     ; 220 ErrorStatus STL_LSIinit(void)
 635                     ; 221 {
 636                     	switch	.text
 637  0100               _STL_LSIinit:
 639  0100 5203          	subw	sp,#3
 640       00000003      OFST:	set	3
 643                     ; 222   ErrorStatus result = SUCCESS;
 645  0102 a601          	ld	a,#1
 646  0104 6b01          	ld	(OFST-2,sp),a
 648                     ; 224     uint16_t time_out = LSI_START_TIMEOUT;
 650  0106 ae06a4        	ldw	x,#1700
 651  0109 1f02          	ldw	(OFST-1,sp),x
 653                     ; 227   CtrlFlowCnt += LSI_INIT_CALLEE;
 655  010b ce0000        	ldw	x,_CtrlFlowCnt
 656  010e 1c0011        	addw	x,#17
 657  0111 cf0000        	ldw	_CtrlFlowCnt,x
 658                     ; 234       CLK->ICKR |= CLK_ICKR_LSIEN;
 660  0114 721650c0      	bset	20672,#3
 662  0118 2007          	jra	L362
 663  011a               L162:
 664                     ; 244         time_out--;
 666  011a 1e02          	ldw	x,(OFST-1,sp)
 667  011c 1d0001        	subw	x,#1
 668  011f 1f02          	ldw	(OFST-1,sp),x
 670  0121               L362:
 671                     ; 241       while (((CLK->ICKR & CLK_ICKR_LSIRDY) == 0u) && (time_out != 0u))
 673  0121 c650c0        	ld	a,20672
 674  0124 a510          	bcp	a,#16
 675  0126 2604          	jrne	L762
 677  0128 1e02          	ldw	x,(OFST-1,sp)
 678  012a 26ee          	jrne	L162
 679  012c               L762:
 680                     ; 246       if (time_out == 0u)
 682  012c 1e02          	ldw	x,(OFST-1,sp)
 683  012e 2602          	jrne	L172
 684                     ; 248         result = ERROR;     /* Internal low speed oscillator failure */
 686  0130 0f01          	clr	(OFST-2,sp)
 688  0132               L172:
 689                     ; 259   CtrlFlowCntInv -= LSI_INIT_CALLEE;
 691  0132 ce0000        	ldw	x,_CtrlFlowCntInv
 692  0135 1d0011        	subw	x,#17
 693  0138 cf0000        	ldw	_CtrlFlowCntInv,x
 694                     ; 261   return (result);
 696  013b 7b01          	ld	a,(OFST-2,sp)
 699  013d 5b03          	addw	sp,#3
 700  013f 81            	ret
 748                     ; 277 ErrorStatus STL_HSE_CSSinit(void)
 748                     ; 278 {
 749                     	switch	.text
 750  0140               _STL_HSE_CSSinit:
 752  0140 5203          	subw	sp,#3
 753       00000003      OFST:	set	3
 756                     ; 279   ErrorStatus result = SUCCESS;
 758  0142 a601          	ld	a,#1
 759  0144 6b01          	ld	(OFST-2,sp),a
 761                     ; 280   uint16_t time_out = HSE_START_TIMEOUT;
 763  0146 ae4268        	ldw	x,#17000
 764  0149 1f02          	ldw	(OFST-1,sp),x
 766                     ; 282   CtrlFlowCnt += HSE_INIT_CALLEE;
 768  014b ce0000        	ldw	x,_CtrlFlowCnt
 769  014e 1c0013        	addw	x,#19
 770  0151 cf0000        	ldw	_CtrlFlowCnt,x
 771                     ; 288     CLK->ECKR |= CLK_ECKR_HSEEN;
 773  0154 721050c1      	bset	20673,#0
 775  0158 2007          	jra	L713
 776  015a               L513:
 777                     ; 298       --time_out;
 779  015a 1e02          	ldw	x,(OFST-1,sp)
 780  015c 1d0001        	subw	x,#1
 781  015f 1f02          	ldw	(OFST-1,sp),x
 783  0161               L713:
 784                     ; 295     while (((CLK->ECKR & CLK_ECKR_HSERDY) == 0u) && (time_out != 0u))
 786  0161 c650c1        	ld	a,20673
 787  0164 a502          	bcp	a,#2
 788  0166 2604          	jrne	L323
 790  0168 1e02          	ldw	x,(OFST-1,sp)
 791  016a 26ee          	jrne	L513
 792  016c               L323:
 793                     ; 301   if (time_out == 0u)
 795  016c 1e02          	ldw	x,(OFST-1,sp)
 796  016e 2604          	jrne	L523
 797                     ; 303     result = ERROR;     /* Internal low speed oscillator failure */
 799  0170 0f01          	clr	(OFST-2,sp)
 802  0172 2009          	jra	L723
 803  0174               L523:
 804                     ; 310     enableInterrupts();
 807  0174 9a            rim
 809                     ; 311     CLK->CSSR |= CLK_CSSR_CSSDIE;     /* CSS detection interrupt enable */
 812  0175 721450c8      	bset	20680,#2
 813                     ; 312     CLK->CSSR |= CLK_CSSR_CSSEN;      /* CSS enable */
 815  0179 721050c8      	bset	20680,#0
 816  017d               L723:
 817                     ; 321   CtrlFlowCntInv -= HSE_INIT_CALLEE;
 819  017d ce0000        	ldw	x,_CtrlFlowCntInv
 820  0180 1d0013        	subw	x,#19
 821  0183 cf0000        	ldw	_CtrlFlowCntInv,x
 822                     ; 323   return (result);
 824  0186 7b01          	ld	a,(OFST-2,sp)
 827  0188 5b03          	addw	sp,#3
 828  018a 81            	ret
 863                     ; 338 uint16_t calc_captured_value(void)
 863                     ; 339 {
 864                     	switch	.text
 865  018b               _calc_captured_value:
 867  018b 89            	pushw	x
 868       00000002      OFST:	set	2
 871                     ; 360       switch ( CLK->CMSR )
 873  018c c650c3        	ld	a,20675
 875                     ; 370           break;
 876  018f a0b4          	sub	a,#180
 877  0191 2726          	jreq	L333
 878  0193 a02d          	sub	a,#45
 879  0195 2707          	jreq	L133
 880  0197               L533:
 881                     ; 368         default:
 881                     ; 369           capt_val = 4u;                 /* invalid value in CMSR will generate HW reset */
 883  0197 ae0004        	ldw	x,#4
 884  019a 1f01          	ldw	(OFST-1,sp),x
 886                     ; 370           break;
 888  019c 2020          	jra	L753
 889  019e               L133:
 890                     ; 362         case 0xE1:
 890                     ; 363           capt_val = (uint16_t)(((4u * HSI_VALUE) / LSI_VALUE) >> ((CLK->CKDIVR & CLK_CKDIVR_HSIDIV) >> 3u));
 892  019e ae01f4        	ldw	x,#500
 893  01a1 bf02          	ldw	c_lreg+2,x
 894  01a3 ae0000        	ldw	x,#0
 895  01a6 bf00          	ldw	c_lreg,x
 896  01a8 c650c6        	ld	a,20678
 897  01ab 44            	srl	a
 898  01ac 44            	srl	a
 899  01ad 44            	srl	a
 900  01ae a403          	and	a,#3
 901  01b0 cd0000        	call	c_lursh
 903  01b3 be02          	ldw	x,c_lreg+2
 904  01b5 1f01          	ldw	(OFST-1,sp),x
 906                     ; 364           break;
 908  01b7 2005          	jra	L753
 909  01b9               L333:
 910                     ; 365         case 0xB4:
 910                     ; 366           capt_val = (uint16_t)((4u * HSE_VALUE) / LSI_VALUE);
 912  01b9 ae01f4        	ldw	x,#500
 913  01bc 1f01          	ldw	(OFST-1,sp),x
 915                     ; 367           break;
 917  01be               L753:
 918                     ; 381   return (capt_val);
 920  01be 1e01          	ldw	x,(OFST-1,sp)
 923  01c0 5b02          	addw	sp,#2
 924  01c2 81            	ret
 972                     ; 389 ErrorStatus STL_InitClock_Xcross_Measurement(void)
 972                     ; 390 {
 973                     	switch	.text
 974  01c3               _STL_InitClock_Xcross_Measurement:
 976  01c3 5203          	subw	sp,#3
 977       00000003      OFST:	set	3
 980                     ; 391   uint16_t time_out = FREQ_MEAS_TIMEOUT;
 982  01c5 ae06a4        	ldw	x,#1700
 983  01c8 1f02          	ldw	(OFST-1,sp),x
 985                     ; 392   ErrorStatus sts = SUCCESS;
 987  01ca a601          	ld	a,#1
 988  01cc 6b01          	ld	(OFST-2,sp),a
 990                     ; 394   CtrlFlowCnt += XCLK_MEASURE_INIT_CALLEE;
 992  01ce ce0000        	ldw	x,_CtrlFlowCnt
 993  01d1 1c0017        	addw	x,#23
 994  01d4 cf0000        	ldw	_CtrlFlowCnt,x
 995                     ; 442     CLK->PCKENR1 |= (CLK_PCKENR1_TIM3);
 997  01d7 721c50c7      	bset	20679,#6
 998                     ; 445     AWU->CSR |= AWU_CSR_MSR;
1000  01db 721050f0      	bset	20720,#0
1001                     ; 448     TIM3->PSCR = 0u;                                    /* init divider register /1 */
1003  01df 725f532a      	clr	21290
1004                     ; 449     TIM3->ARRH = 0xffu;			                /* init ARR & OC1 compare registers */
1006  01e3 35ff532b      	mov	21291,#255
1007                     ; 450     TIM3->ARRL = 0xffu;
1009  01e7 35ff532c      	mov	21292,#255
1010                     ; 451     TIM3->CNTRH = 0xffu;
1012  01eb 35ff5328      	mov	21288,#255
1013                     ; 452     TIM3->CNTRL = 0xffu;
1015  01ef 35ff5329      	mov	21289,#255
1016                     ; 453     TIM3->CCMR1 &= (uint8_t)(~(TIM3_CCMR_ICxPSC | TIM3_CCMR_CCxS));
1018  01f3 c65325        	ld	a,21285
1019  01f6 a4f0          	and	a,#240
1020  01f8 c75325        	ld	21285,a
1021                     ; 454     TIM3->CCMR1 |= ((2u << 2) & TIM3_CCMR_ICxPSC) | (1u & TIM3_CCMR_CCxS);
1023  01fb c65325        	ld	a,21285
1024  01fe aa09          	or	a,#9
1025  0200 c75325        	ld	21285,a
1026                     ; 456     TIM3->CCER1 = TIM3_CCER1_CC1E; 		        /* CC1 IC enable, rising edge */
1028  0203 35015327      	mov	21287,#1
1029                     ; 457     TIM3->CR1 |= (uint8_t)(TIM3_CR1_URS | TIM3_CR1_CEN);      /* enable timer */
1031  0207 c65320        	ld	a,21280
1032  020a aa05          	or	a,#5
1033  020c c75320        	ld	21280,a
1035  020f 2007          	jra	L504
1036  0211               L304:
1037                     ; 461       --time_out;
1039  0211 1e02          	ldw	x,(OFST-1,sp)
1040  0213 1d0001        	subw	x,#1
1041  0216 1f02          	ldw	(OFST-1,sp),x
1043  0218               L504:
1044                     ; 459     while (((TIM3->SR1 & TIM3_SR1_CC1IF) != TIM3_SR1_CC1IF) && (time_out != 0u))
1046  0218 c65322        	ld	a,21282
1047  021b a402          	and	a,#2
1048  021d a102          	cp	a,#2
1049  021f 2704          	jreq	L114
1051  0221 1e02          	ldw	x,(OFST-1,sp)
1052  0223 26ec          	jrne	L304
1053  0225               L114:
1054                     ; 489   CtrlFlowCntInv -= XCLK_MEASURE_INIT_CALLEE;
1056  0225 ce0000        	ldw	x,_CtrlFlowCntInv
1057  0228 1d0017        	subw	x,#23
1058  022b cf0000        	ldw	_CtrlFlowCntInv,x
1059                     ; 491   if (time_out == 0u)
1061  022e 1e02          	ldw	x,(OFST-1,sp)
1062  0230 2602          	jrne	L314
1063                     ; 493     sts = ERROR;
1065  0232 0f01          	clr	(OFST-2,sp)
1067  0234               L314:
1068                     ; 495   return(sts);
1070  0234 7b01          	ld	a,(OFST-2,sp)
1073  0236 5b03          	addw	sp,#3
1074  0238 81            	ret
1077                     	switch	.ubsct
1078  0000               L124_period:
1079  0000 0000          	ds.b	2
1080  0002               L714_temp_cc1_last:
1081  0002 0000          	ds.b	2
1082  0004               L514_temp_cc1:
1083  0004 0000          	ds.b	2
1145                     ; 510 uint16_t STL_MeasureLSIPeriod(void)
1145                     ; 511 {
1146                     	switch	.text
1147  0239               _STL_MeasureLSIPeriod:
1149  0239 5204          	subw	sp,#4
1150       00000004      OFST:	set	4
1153                     ; 512   uint16_t time_out = LSI_MEASURE_TIMEOUT;
1155  023b ae4268        	ldw	x,#17000
1156  023e 1f03          	ldw	(OFST-1,sp),x
1158                     ; 517   CtrlFlowCnt += XLCK_LSI_PERIOD_CALLEE;
1160  0240 ce0000        	ldw	x,_CtrlFlowCnt
1161  0243 1c001b        	addw	x,#27
1162  0246 cf0000        	ldw	_CtrlFlowCnt,x
1163                     ; 559     TIM3->SR2 &= (uint8_t)(~TIM3_SR2_CC1OF);    /* clear CC1 overcapture flag */
1165  0249 72135323      	bres	21283,#1
1166                     ; 560     TIM3->SR1 &= (uint8_t)(~TIM3_SR1_CC1IF);    /* clear CC1 capture flag */
1168  024d 72135322      	bres	21282,#1
1170  0251 2007          	jra	L754
1171  0253               L554:
1172                     ; 565       --time_out;
1174  0253 1e03          	ldw	x,(OFST-1,sp)
1175  0255 1d0001        	subw	x,#1
1176  0258 1f03          	ldw	(OFST-1,sp),x
1178  025a               L754:
1179                     ; 563     while (((TIM3->SR1 & TIM3_SR1_CC1IF) != TIM3_SR1_CC1IF) && (time_out != 0u))
1181  025a c65322        	ld	a,21282
1182  025d a402          	and	a,#2
1183  025f a102          	cp	a,#2
1184  0261 2704          	jreq	L364
1186  0263 1e03          	ldw	x,(OFST-1,sp)
1187  0265 26ec          	jrne	L554
1188  0267               L364:
1189                     ; 568     temp_cc1_last = ((uint16_t)(TIM3->CCR1H) << 8);
1191  0267 c6532d        	ld	a,21293
1192  026a 5f            	clrw	x
1193  026b 97            	ld	xl,a
1194  026c 4f            	clr	a
1195  026d 02            	rlwa	x,a
1196  026e bf02          	ldw	L714_temp_cc1_last,x
1197                     ; 569     temp_cc1_last += TIM3->CCR1L;          /* preload register is frozen till CCR1L value is not read */
1199  0270 c6532e        	ld	a,21294
1200  0273 5f            	clrw	x
1201  0274 97            	ld	xl,a
1202  0275 1f01          	ldw	(OFST-3,sp),x
1204  0277 be02          	ldw	x,L714_temp_cc1_last
1205  0279 72fb01        	addw	x,(OFST-3,sp)
1206  027c bf02          	ldw	L714_temp_cc1_last,x
1208  027e 2007          	jra	L764
1209  0280               L564:
1210                     ; 575       --time_out;
1212  0280 1e03          	ldw	x,(OFST-1,sp)
1213  0282 1d0001        	subw	x,#1
1214  0285 1f03          	ldw	(OFST-1,sp),x
1216  0287               L764:
1217                     ; 573     while (((TIM3->SR1 & TIM3_SR1_CC1IF) != TIM3_SR1_CC1IF) && (time_out != 0u))
1219  0287 c65322        	ld	a,21282
1220  028a a402          	and	a,#2
1221  028c a102          	cp	a,#2
1222  028e 2704          	jreq	L374
1224  0290 1e03          	ldw	x,(OFST-1,sp)
1225  0292 26ec          	jrne	L564
1226  0294               L374:
1227                     ; 579     if (((TIM3->SR2 & TIM3_SR2_CC1OF) != 0u))
1229  0294 c65323        	ld	a,21283
1230  0297 a502          	bcp	a,#2
1231  0299 2705          	jreq	L574
1232                     ; 581       period = 0u;                         /* when overcaptured, ignore this measurement result */
1234  029b 5f            	clrw	x
1235  029c bf00          	ldw	L124_period,x
1237  029e 202a          	jra	L774
1238  02a0               L574:
1239                     ; 583     else if (time_out == 0u)
1241  02a0 1e03          	ldw	x,(OFST-1,sp)
1242  02a2 2607          	jrne	L105
1243                     ; 585       period = 1u;                         /* set wrong (too short) LSI period at case of no timer event */
1245  02a4 ae0001        	ldw	x,#1
1246  02a7 bf00          	ldw	L124_period,x
1248  02a9 201f          	jra	L774
1249  02ab               L105:
1250                     ; 590       temp_cc1 = ((uint16_t)(TIM3->CCR1H) << 8);
1252  02ab c6532d        	ld	a,21293
1253  02ae 5f            	clrw	x
1254  02af 97            	ld	xl,a
1255  02b0 4f            	clr	a
1256  02b1 02            	rlwa	x,a
1257  02b2 bf04          	ldw	L514_temp_cc1,x
1258                     ; 591       temp_cc1 += TIM3->CCR1L;               /* preload register is frozen till CCR1L value is not read */
1260  02b4 c6532e        	ld	a,21294
1261  02b7 5f            	clrw	x
1262  02b8 97            	ld	xl,a
1263  02b9 1f01          	ldw	(OFST-3,sp),x
1265  02bb be04          	ldw	x,L514_temp_cc1
1266  02bd 72fb01        	addw	x,(OFST-3,sp)
1267  02c0 bf04          	ldw	L514_temp_cc1,x
1268                     ; 593       period = temp_cc1 - temp_cc1_last;
1270  02c2 be04          	ldw	x,L514_temp_cc1
1271  02c4 72b00002      	subw	x,L714_temp_cc1_last
1272  02c8 bf00          	ldw	L124_period,x
1273  02ca               L774:
1274                     ; 634   CtrlFlowCntInv -= XLCK_LSI_PERIOD_CALLEE;
1276  02ca ce0000        	ldw	x,_CtrlFlowCntInv
1277  02cd 1d001b        	subw	x,#27
1278  02d0 cf0000        	ldw	_CtrlFlowCntInv,x
1279                     ; 635   return (period);
1281  02d3 be00          	ldw	x,L124_period
1284  02d5 5b04          	addw	sp,#4
1285  02d7 81            	ret
1298                     	xdef	_STL_HSE_CSSinit
1299                     	xdef	_STL_LSIinit
1300                     	xref	_CtrlFlowCntInv
1301                     	xref	_CtrlFlowCnt
1302                     	xref	_FailSafe
1303                     	xdef	_STL_MeasureLSIPeriod
1304                     	xdef	_STL_InitClock_Xcross_Measurement
1305                     	xdef	_calc_captured_value
1306                     	xdef	_switch_clock_system
1307                     	xdef	_STL_ClockStartUpTest
1308                     	xref.b	c_lreg
1309                     	xref.b	c_x
1328                     	xref	c_lursh
1329                     	xref	c_bmulx
1330                     	end
