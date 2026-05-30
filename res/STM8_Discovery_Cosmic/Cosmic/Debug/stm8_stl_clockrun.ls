   1                     ; C Compiler for STM8 (COSMIC Software)
   2                     ; Parser V4.11.13 - 05 Feb 2019
   3                     ; Generator V4.4.9 - 06 Feb 2019
 101                     	switch	.ubsct
 102  0000               L14_expected_value:
 103  0000 0000          	ds.b	2
 104  0002               L73_lsi_period:
 105  0002 0000          	ds.b	2
 249                     ; 64 ClockStatus STL_ClockFreqTest(void)
 249                     ; 65 {
 251                     	switch	.text
 252  0000               _STL_ClockFreqTest:
 254  0000 88            	push	a
 255       00000001      OFST:	set	1
 258                     ; 66   ClockStatus result = TEST_ONGOING; /* In case of unexpected exit */
 260  0001 a603          	ld	a,#3
 261  0003 6b01          	ld	(OFST+0,sp),a
 263                     ; 70   CtrlFlowCnt += FREQ_TEST_CALLEE;
 265  0005 ce0000        	ldw	x,_CtrlFlowCnt
 266  0008 1c0017        	addw	x,#23
 267  000b cf0000        	ldw	_CtrlFlowCnt,x
 268                     ; 72   if ((lsi_period = STL_MeasureLSIPeriod()) != 0u)
 270  000e cd0000        	call	_STL_MeasureLSIPeriod
 272  0011 bf02          	ldw	L73_lsi_period,x
 273  0013 be02          	ldw	x,L73_lsi_period
 274  0015 272e          	jreq	L721
 275                     ; 74     expected_value = calc_captured_value();
 277  0017 cd0000        	call	_calc_captured_value
 279  001a bf00          	ldw	L14_expected_value,x
 280                     ; 76     if ((lsi_period < (expected_value * 3u / 4u))\
 280                     ; 77     || (lsi_period > (expected_value * 5u / 4u)))
 282  001c be00          	ldw	x,L14_expected_value
 283  001e a603          	ld	a,#3
 284  0020 cd0000        	call	c_bmulx
 286  0023 54            	srlw	x
 287  0024 54            	srlw	x
 288  0025 b302          	cpw	x,L73_lsi_period
 289  0027 220d          	jrugt	L331
 291  0029 be00          	ldw	x,L14_expected_value
 292  002b a605          	ld	a,#5
 293  002d cd0000        	call	c_bmulx
 295  0030 54            	srlw	x
 296  0031 54            	srlw	x
 297  0032 b302          	cpw	x,L73_lsi_period
 298  0034 240b          	jruge	L131
 299  0036               L331:
 300                     ; 79       switch_clock_system(to_HSI);      /* Switch back to internal clock */
 302  0036 a6e1          	ld	a,#225
 303  0038 cd0000        	call	_switch_clock_system
 305                     ; 80       result = EXT_SOURCE_FAIL;	        /* Sub-harmonics: HSE +/-25% out of expected */
 307  003b a605          	ld	a,#5
 308  003d 6b01          	ld	(OFST+0,sp),a
 311  003f 2004          	jra	L721
 312  0041               L131:
 313                     ; 91       result = FREQ_OK;         /* frequecy is within expected range */
 315  0041 a609          	ld	a,#9
 316  0043 6b01          	ld	(OFST+0,sp),a
 318  0045               L721:
 319                     ; 95   CtrlFlowCntInv -= FREQ_TEST_CALLEE;
 321  0045 ce0000        	ldw	x,_CtrlFlowCntInv
 322  0048 1d0017        	subw	x,#23
 323  004b cf0000        	ldw	_CtrlFlowCntInv,x
 324                     ; 96   return (result);
 326  004e 7b01          	ld	a,(OFST+0,sp)
 329  0050 5b01          	addw	sp,#1
 330  0052 81            	ret
 343                     	xref	_CtrlFlowCntInv
 344                     	xref	_CtrlFlowCnt
 345                     	xref	_FailSafe
 346                     	xdef	_STL_ClockFreqTest
 347                     	xref	_STL_MeasureLSIPeriod
 348                     	xref	_calc_captured_value
 349                     	xref	_switch_clock_system
 350                     	xref.b	c_x
 369                     	xref	c_bmulx
 370                     	end
