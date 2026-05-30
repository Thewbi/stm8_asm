   1                     ; C Compiler for STM8 (COSMIC Software)
   2                     ; Parser V4.11.13 - 05 Feb 2019
   3                     ; Generator V4.4.9 - 06 Feb 2019
 195                     ; 83 ClassBTestStatus STL_crc16Run(void)
 195                     ; 84 {
 197                     	switch	.text
 198  0000               _STL_crc16Run:
 200  0000 88            	push	a
 201       00000001      OFST:	set	1
 204                     ; 85   ClassBTestStatus result = CTRL_FLW_ERROR; /* In case of abnormal func exit*/
 206                     ; 87   CtrlFlowCnt += CRC16_RUN_TEST_CALLEE;
 208  0001 ce0000        	ldw	x,_CtrlFlowCnt
 209  0004 1c0029        	addw	x,#41
 210  0007 cf0000        	ldw	_CtrlFlowCnt,x
 211                     ; 90   if ((CurrentCrc16 ^ CurrentCrc16Inv) == 0xFFFFu)
 213  000a ce0000        	ldw	x,_CurrentCrc16
 214  000d 01            	rrwa	x,a
 215  000e c80001        	xor	a,_CurrentCrc16Inv+1
 216  0011 01            	rrwa	x,a
 217  0012 c80000        	xor	a,_CurrentCrc16Inv
 218  0015 01            	rrwa	x,a
 219  0016 a3ffff        	cpw	x,#65535
 220  0019 2622          	jrne	L701
 221                     ; 95         switch ( _block_checksum160() )
 223  001b cd0000        	call	__block_checksum160
 226                     ; 132         break;
 227  001e 4d            	tnz	a
 228  001f 270d          	jreq	L73
 229  0021 4a            	dec	a
 230  0022 2713          	jreq	L14
 231  0024 a0fe          	sub	a,#254
 232  0026 2706          	jreq	L73
 233  0028               L34:
 234                     ; 130       default:
 234                     ; 131         result = TEST_FAILURE;
 236  0028 a603          	ld	a,#3
 237  002a 6b01          	ld	(OFST+0,sp),a
 239                     ; 132         break;
 241  002c 2013          	jra	L511
 242  002e               L73:
 243                     ; 112       case CRC_ERROR:  /* flash test error is ignored at debug mode with Cosmic */
 243                     ; 113     #endif /* DEBUG && _COSMIC_ */
 243                     ; 114       case CRC_OK:
 243                     ; 115         result = TEST_OK;
 245  002e a604          	ld	a,#4
 246  0030 6b01          	ld	(OFST+0,sp),a
 248                     ; 117           STL_FlashCrc16Init(); /* Prepare next test */
 250  0032 cd0000        	call	_STL_FlashCrc16Init
 252                     ; 121         break;
 254  0035 200a          	jra	L511
 255  0037               L14:
 256                     ; 123       case CRC_ONGOING:
 256                     ; 124         result = TEST_RUNNING;
 258  0037 0f01          	clr	(OFST+0,sp)
 260                     ; 125         break;
 262  0039 2006          	jra	L511
 263  003b               L311:
 264                     ; 132         break;
 265  003b 2004          	jra	L511
 266  003d               L701:
 267                     ; 137     result = CLASS_B_DATA_FAIL;
 269  003d a601          	ld	a,#1
 270  003f 6b01          	ld	(OFST+0,sp),a
 272  0041               L511:
 273                     ; 140   CtrlFlowCntInv -= CRC16_RUN_TEST_CALLEE;
 275  0041 ce0000        	ldw	x,_CtrlFlowCntInv
 276  0044 1d0029        	subw	x,#41
 277  0047 cf0000        	ldw	_CtrlFlowCntInv,x
 278                     ; 142   return (result);
 280  004a 7b01          	ld	a,(OFST+0,sp)
 283  004c 5b01          	addw	sp,#1
 284  004e 81            	ret
 317                     	xref	__block_checksum160
 318                     	xref	_CurrentCrc16Inv
 319                     	xref	_CtrlFlowCntInv
 320                     	xref	_CurrentCrc16
 321                     	xref	_CtrlFlowCnt
 322                     	xref	_FailSafe
 323                     	xdef	_STL_crc16Run
 324                     	xref	_STL_FlashCrc16Init
 343                     	end
