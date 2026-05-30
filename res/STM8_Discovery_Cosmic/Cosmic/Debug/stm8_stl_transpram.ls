   1                     ; C Compiler for STM8 (COSMIC Software)
   2                     ; Parser V4.11.13 - 05 Feb 2019
   3                     ; Generator V4.4.9 - 06 Feb 2019
 136                     ; 72 void STL_TranspMarchInit(void)
 136                     ; 73 {
 138                     	switch	.text
 139  0000               _STL_TranspMarchInit:
 143                     ; 74   CtrlFlowCnt += RAM_MARCH_INIT_CALLEE;
 145  0000 ce0000        	ldw	x,_CtrlFlowCnt
 146  0003 1c0029        	addw	x,#41
 147  0006 cf0000        	ldw	_CtrlFlowCnt,x
 148                     ; 76   p_RunTimeRamChk = CLASS_B_START;
 150  0009 ae0000        	ldw	x,#__clb_start
 151  000c bf00          	ldw	_p_RunTimeRamChk,x
 152                     ; 79   p_RunTimeRamChk -= RT_RAM_BLOCK_OVERLAP;    
 154  000e be00          	ldw	x,_p_RunTimeRamChk
 155  0010 1d0001        	subw	x,#1
 156  0013 bf00          	ldw	_p_RunTimeRamChk,x
 157                     ; 80   p_RunTimeRamChkInv = (uint8_t *)(uint16_t)(~(uint16_t)(p_RunTimeRamChk));
 159  0015 be00          	ldw	x,_p_RunTimeRamChk
 160  0017 53            	cplw	x
 161  0018 bf00          	ldw	_p_RunTimeRamChkInv,x
 162                     ; 82   CtrlFlowCntInv -= RAM_MARCH_INIT_CALLEE;
 164  001a ce0000        	ldw	x,_CtrlFlowCntInv
 165  001d 1d0029        	subw	x,#41
 166  0020 cf0000        	ldw	_CtrlFlowCntInv,x
 167                     ; 84 }
 170  0023 81            	ret
 283                     ; 99 ClassBTestStatus STL_TranspMarch(void)
 283                     ; 100 {
 284                     	switch	.text
 285  0024               _STL_TranspMarch:
 287  0024 5206          	subw	sp,#6
 288       00000006      OFST:	set	6
 291                     ; 101   ClassBTestStatus result = TEST_RUNNING;
 293  0026 0f06          	clr	(OFST+0,sp)
 295                     ; 110   ISRCtrlFlowCnt += RAM_MARCH_ISR_CALLEE;
 297  0028 ce0000        	ldw	x,_ISRCtrlFlowCnt
 298  002b 1c000b        	addw	x,#11
 299  002e cf0000        	ldw	_ISRCtrlFlowCnt,x
 300                     ; 113   if ((((uint16_t)p_RunTimeRamChk) ^ ((uint16_t)p_RunTimeRamChkInv)) == 0xFFFFu)
 302  0031 be00          	ldw	x,_p_RunTimeRamChk
 303  0033 01            	rrwa	x,a
 304  0034 b801          	xor	a,_p_RunTimeRamChkInv+1
 305  0036 01            	rrwa	x,a
 306  0037 b800          	xor	a,_p_RunTimeRamChkInv
 307  0039 01            	rrwa	x,a
 308  003a a3ffff        	cpw	x,#65535
 309  003d 2703          	jreq	L01
 310  003f cc0198        	jp	L521
 311  0042               L01:
 312                     ; 116     if (p_RunTimeRamChk >= CLASS_B_END)
 314  0042 be00          	ldw	x,_p_RunTimeRamChk
 315  0044 a30000        	cpw	x,#__clb_end
 316  0047 2403          	jruge	L21
 317  0049 cc00e8        	jp	L721
 318  004c               L21:
 319                     ; 119       p_RunTimeRamChk = &aRunTimeRamBuf[0];
 321  004c ae0000        	ldw	x,#_aRunTimeRamBuf
 322  004f bf00          	ldw	_p_RunTimeRamChk,x
 323                     ; 120       p_RunTimeRamChkInv = (uint8_t *)(uint16_t)(~(uint16_t)(p_RunTimeRamChk));
 325  0051 ae0000        	ldw	x,#_aRunTimeRamBuf
 326  0054 53            	cplw	x
 327  0055 bf00          	ldw	_p_RunTimeRamChkInv,x
 328  0057               L131:
 329                     ; 126         *p_RunTimeRamChk++ = BCKGRND;
 331  0057 be00          	ldw	x,_p_RunTimeRamChk
 332  0059 1c0001        	addw	x,#1
 333  005c bf00          	ldw	_p_RunTimeRamChk,x
 334  005e 1d0001        	subw	x,#1
 335  0061 7f            	clr	(x)
 336                     ; 128       while (p_RunTimeRamChk < &aRunTimeRamBuf[RT_RAM_BUF_SIZE]);
 338  0062 be00          	ldw	x,_p_RunTimeRamChk
 339  0064 a30008        	cpw	x,#_aRunTimeRamBuf+8
 340  0067 25ee          	jrult	L131
 341                     ; 131       p_RunTimeRamChk = &aRunTimeRamBuf[0];
 343  0069 ae0000        	ldw	x,#_aRunTimeRamBuf
 344  006c bf00          	ldw	_p_RunTimeRamChk,x
 345  006e               L731:
 346                     ; 134         if (*p_RunTimeRamChk != BCKGRND)
 348  006e 923d00        	tnz	[_p_RunTimeRamChk.w]
 349  0071 2704          	jreq	L541
 350                     ; 136           result = TEST_FAILURE;
 352  0073 a603          	ld	a,#3
 353  0075 6b06          	ld	(OFST+0,sp),a
 355  0077               L541:
 356                     ; 138         *p_RunTimeRamChk++ = INV_BCKGRND;
 358  0077 be00          	ldw	x,_p_RunTimeRamChk
 359  0079 1c0001        	addw	x,#1
 360  007c bf00          	ldw	_p_RunTimeRamChk,x
 361  007e 1d0001        	subw	x,#1
 362  0081 a6ff          	ld	a,#255
 363  0083 f7            	ld	(x),a
 364                     ; 140       while (p_RunTimeRamChk < &aRunTimeRamBuf[RT_RAM_BUF_SIZE]);
 366  0084 be00          	ldw	x,_p_RunTimeRamChk
 367  0086 a30008        	cpw	x,#_aRunTimeRamBuf+8
 368  0089 25e3          	jrult	L731
 369                     ; 172       p_RunTimeRamChk = &aRunTimeRamBuf[RT_RAM_BUF_SIZE];
 371  008b ae0008        	ldw	x,#_aRunTimeRamBuf+8
 372  008e bf00          	ldw	_p_RunTimeRamChk,x
 373  0090               L741:
 374                     ; 175         --p_RunTimeRamChk;
 376  0090 be00          	ldw	x,_p_RunTimeRamChk
 377  0092 1d0001        	subw	x,#1
 378  0095 bf00          	ldw	_p_RunTimeRamChk,x
 379                     ; 176         if ( *p_RunTimeRamChk != INV_BCKGRND )
 381  0097 92c600        	ld	a,[_p_RunTimeRamChk.w]
 382  009a a1ff          	cp	a,#255
 383  009c 2704          	jreq	L551
 384                     ; 178           result = TEST_FAILURE;
 386  009e a603          	ld	a,#3
 387  00a0 6b06          	ld	(OFST+0,sp),a
 389  00a2               L551:
 390                     ; 180         *p_RunTimeRamChk = BCKGRND;
 392  00a2 923f00        	clr	[_p_RunTimeRamChk.w]
 393                     ; 182       while (p_RunTimeRamChk > &aRunTimeRamBuf[0]);
 395  00a5 be00          	ldw	x,_p_RunTimeRamChk
 396  00a7 a30000        	cpw	x,#_aRunTimeRamBuf
 397  00aa 22e4          	jrugt	L741
 398                     ; 186       p_RunTimeRamChk = &aRunTimeRamBuf[0];
 400  00ac ae0000        	ldw	x,#_aRunTimeRamBuf
 401  00af bf00          	ldw	_p_RunTimeRamChk,x
 402  00b1               L751:
 403                     ; 189         if ( *p_RunTimeRamChk++ != BCKGRND )
 405  00b1 be00          	ldw	x,_p_RunTimeRamChk
 406  00b3 1c0001        	addw	x,#1
 407  00b6 bf00          	ldw	_p_RunTimeRamChk,x
 408  00b8 1d0001        	subw	x,#1
 409  00bb 7d            	tnz	(x)
 410  00bc 2704          	jreq	L161
 411                     ; 191           result = TEST_FAILURE;
 413  00be a603          	ld	a,#3
 414  00c0 6b06          	ld	(OFST+0,sp),a
 416  00c2               L161:
 417                     ; 194       while (p_RunTimeRamChk < &aRunTimeRamBuf[RT_RAM_BUF_SIZE]);
 419  00c2 be00          	ldw	x,_p_RunTimeRamChk
 420  00c4 a30008        	cpw	x,#_aRunTimeRamBuf+8
 421  00c7 25e8          	jrult	L751
 422                     ; 197       p_RunTimeRamChk = CLASS_B_START - 1;
 424  00c9 aeffff        	ldw	x,#__clb_start-1
 425  00cc bf00          	ldw	_p_RunTimeRamChk,x
 426                     ; 198       p_RunTimeRamChkInv = ((uint8_t *)(uint16_t)(~(uint16_t)(p_RunTimeRamChk)));
 428  00ce aeffff        	ldw	x,#__clb_start-1
 429  00d1 53            	cplw	x
 430  00d2 bf00          	ldw	_p_RunTimeRamChkInv,x
 431                     ; 199       if (result == TEST_RUNNING)
 433  00d4 0d06          	tnz	(OFST+0,sp)
 434  00d6 2608          	jrne	L761
 435                     ; 201         result = TEST_OK; /* Means all selected variable memory was scanned */
 437  00d8 a604          	ld	a,#4
 438  00da 6b06          	ld	(OFST+0,sp),a
 441  00dc ac9c019c      	jpf	L532
 442  00e0               L761:
 443                     ; 205         result = TEST_FAILURE;
 445  00e0 a603          	ld	a,#3
 446  00e2 6b06          	ld	(OFST+0,sp),a
 448  00e4 ac9c019c      	jpf	L532
 449  00e8               L721:
 450                     ; 213       p_ram_block_start = p_RunTimeRamChk;
 452  00e8 be00          	ldw	x,_p_RunTimeRamChk
 453  00ea 1f01          	ldw	(OFST-5,sp),x
 455                     ; 214       p_ram_block_end = p_RunTimeRamChk + RT_RAM_BLOCKSIZE + 2u * RT_RAM_BLOCK_OVERLAP;
 457  00ec be00          	ldw	x,_p_RunTimeRamChk
 458  00ee 1c0006        	addw	x,#6
 459  00f1 1f04          	ldw	(OFST-2,sp),x
 461                     ; 215       index = 1u; /* first and last item of the buffer is not used and tested as overlay */
 463  00f3 a601          	ld	a,#1
 464  00f5 6b03          	ld	(OFST-3,sp),a
 466  00f7               L571:
 467                     ; 218         aRunTimeRamBuf[index++] = *p_RunTimeRamChk;
 469  00f7 7b03          	ld	a,(OFST-3,sp)
 470  00f9 97            	ld	xl,a
 471  00fa 0c03          	inc	(OFST-3,sp)
 473  00fc 9f            	ld	a,xl
 474  00fd 5f            	clrw	x
 475  00fe 97            	ld	xl,a
 476  00ff 92c600        	ld	a,[_p_RunTimeRamChk.w]
 477  0102 d70000        	ld	(_aRunTimeRamBuf,x),a
 478                     ; 219         *p_RunTimeRamChk++ = BCKGRND;
 480  0105 be00          	ldw	x,_p_RunTimeRamChk
 481  0107 1c0001        	addw	x,#1
 482  010a bf00          	ldw	_p_RunTimeRamChk,x
 483  010c 1d0001        	subw	x,#1
 484  010f 7f            	clr	(x)
 485                     ; 221       while (p_RunTimeRamChk < p_ram_block_end);
 487  0110 be00          	ldw	x,_p_RunTimeRamChk
 488  0112 1304          	cpw	x,(OFST-2,sp)
 489  0114 25e1          	jrult	L571
 490                     ; 225       p_RunTimeRamChk = p_ram_block_start;
 492  0116 1e01          	ldw	x,(OFST-5,sp)
 493  0118 bf00          	ldw	_p_RunTimeRamChk,x
 494  011a               L302:
 495                     ; 228         if (*p_RunTimeRamChk  != BCKGRND)
 497  011a 923d00        	tnz	[_p_RunTimeRamChk.w]
 498  011d 2704          	jreq	L112
 499                     ; 230           result = TEST_FAILURE;
 501  011f a603          	ld	a,#3
 502  0121 6b06          	ld	(OFST+0,sp),a
 504  0123               L112:
 505                     ; 232         *p_RunTimeRamChk++ = INV_BCKGRND;
 507  0123 be00          	ldw	x,_p_RunTimeRamChk
 508  0125 1c0001        	addw	x,#1
 509  0128 bf00          	ldw	_p_RunTimeRamChk,x
 510  012a 1d0001        	subw	x,#1
 511  012d a6ff          	ld	a,#255
 512  012f f7            	ld	(x),a
 513                     ; 234       while (p_RunTimeRamChk < p_ram_block_end);
 515  0130 be00          	ldw	x,_p_RunTimeRamChk
 516  0132 1304          	cpw	x,(OFST-2,sp)
 517  0134 25e4          	jrult	L302
 518                     ; 267       p_RunTimeRamChk = p_ram_block_end;
 520  0136 1e04          	ldw	x,(OFST-2,sp)
 521  0138 bf00          	ldw	_p_RunTimeRamChk,x
 522  013a               L312:
 523                     ; 270         --p_RunTimeRamChk;
 525  013a be00          	ldw	x,_p_RunTimeRamChk
 526  013c 1d0001        	subw	x,#1
 527  013f bf00          	ldw	_p_RunTimeRamChk,x
 528                     ; 271         if (*p_RunTimeRamChk != INV_BCKGRND)
 530  0141 92c600        	ld	a,[_p_RunTimeRamChk.w]
 531  0144 a1ff          	cp	a,#255
 532  0146 2704          	jreq	L122
 533                     ; 273           result = TEST_FAILURE;
 535  0148 a603          	ld	a,#3
 536  014a 6b06          	ld	(OFST+0,sp),a
 538  014c               L122:
 539                     ; 275         *p_RunTimeRamChk = BCKGRND;
 541  014c 923f00        	clr	[_p_RunTimeRamChk.w]
 542                     ; 277       while (p_RunTimeRamChk > p_ram_block_start);
 544  014f be00          	ldw	x,_p_RunTimeRamChk
 545  0151 1301          	cpw	x,(OFST-5,sp)
 546  0153 22e5          	jrugt	L312
 547                     ; 281       p_RunTimeRamChk = p_ram_block_start;
 549  0155 1e01          	ldw	x,(OFST-5,sp)
 550  0157 bf00          	ldw	_p_RunTimeRamChk,x
 551                     ; 282       index = 1u;
 553  0159 a601          	ld	a,#1
 554  015b 6b03          	ld	(OFST-3,sp),a
 556  015d               L322:
 557                     ; 285         if (*p_RunTimeRamChk != BCKGRND)
 559  015d 923d00        	tnz	[_p_RunTimeRamChk.w]
 560  0160 2704          	jreq	L132
 561                     ; 287           result = TEST_FAILURE;
 563  0162 a603          	ld	a,#3
 564  0164 6b06          	ld	(OFST+0,sp),a
 566  0166               L132:
 567                     ; 289         *p_RunTimeRamChk++ = aRunTimeRamBuf[index++];
 569  0166 7b03          	ld	a,(OFST-3,sp)
 570  0168 97            	ld	xl,a
 571  0169 0c03          	inc	(OFST-3,sp)
 573  016b 9f            	ld	a,xl
 574  016c 5f            	clrw	x
 575  016d 97            	ld	xl,a
 576  016e d60000        	ld	a,(_aRunTimeRamBuf,x)
 577  0171 be00          	ldw	x,_p_RunTimeRamChk
 578  0173 1c0001        	addw	x,#1
 579  0176 bf00          	ldw	_p_RunTimeRamChk,x
 580  0178 1d0001        	subw	x,#1
 581  017b f7            	ld	(x),a
 582                     ; 291       while (p_RunTimeRamChk < p_ram_block_end);
 584  017c be00          	ldw	x,_p_RunTimeRamChk
 585  017e 1304          	cpw	x,(OFST-2,sp)
 586  0180 25db          	jrult	L322
 587                     ; 294       p_RunTimeRamChk -=  (2u *  RT_RAM_BLOCK_OVERLAP);
 589  0182 be00          	ldw	x,_p_RunTimeRamChk
 590  0184 1d0002        	subw	x,#2
 591  0187 bf00          	ldw	_p_RunTimeRamChk,x
 592                     ; 295       p_RunTimeRamChkInv = ((uint8_t *)(uint16_t)(~(uint16_t)(p_RunTimeRamChk)));
 594  0189 be00          	ldw	x,_p_RunTimeRamChk
 595  018b 53            	cplw	x
 596  018c bf00          	ldw	_p_RunTimeRamChkInv,x
 597                     ; 296       if (result != TEST_RUNNING)
 599  018e 0d06          	tnz	(OFST+0,sp)
 600  0190 270a          	jreq	L532
 601                     ; 298         result = TEST_FAILURE;  /* byte block under test was not functional */
 603  0192 a603          	ld	a,#3
 604  0194 6b06          	ld	(OFST+0,sp),a
 606  0196 2004          	jra	L532
 607  0198               L521:
 608                     ; 305     result = CLASS_B_DATA_FAIL;
 610  0198 a601          	ld	a,#1
 611  019a 6b06          	ld	(OFST+0,sp),a
 613  019c               L532:
 614                     ; 308   ISRCtrlFlowCntInv -= RAM_MARCH_ISR_CALLEE;
 616  019c ce0000        	ldw	x,_ISRCtrlFlowCntInv
 617  019f 1d000b        	subw	x,#11
 618  01a2 cf0000        	ldw	_ISRCtrlFlowCntInv,x
 619                     ; 310   return (result);
 621  01a5 7b06          	ld	a,(OFST+0,sp)
 624  01a7 5b06          	addw	sp,#6
 625  01a9 81            	ret
 638                     	xref	_ISRCtrlFlowCntInv
 639                     	xref	_CtrlFlowCntInv
 640                     	xref	_ISRCtrlFlowCnt
 641                     	xref	_CtrlFlowCnt
 642                     	xref.b	_p_RunTimeRamChkInv
 643                     	xref.b	_p_RunTimeRamChk
 644                     	xref	_aRunTimeRamBuf
 645                     	xref	__clb_end
 646                     	xref	__clb_start
 647                     	xref	_FailSafe
 648                     	xdef	_STL_TranspMarch
 649                     	xdef	_STL_TranspMarchInit
 668                     	end
