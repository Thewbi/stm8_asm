   1                     ;	VERIFY 16 BIT SINGLE CHECKSUM
   2                     ;	Copyright (c) 2008 by COSMIC Software
   3                     ;
   4                     ; Modified by STMicroelectronics for partial CRC calculation
   5                     ; Version            : V1.1.0
   6                     ; Date               : MAY-2012
   7                     ;******************************************************************************
   8                     ;
   9                     ; 				COPYRIGHT 2012 STMicroelectronics
  10                     ;
  11                     ; Licensed under MCD-ST Liberty SW License Agreement V2, (the "License");
  12                     ; You may not use this file except in compliance with the License.
  13                     ; You may obtain a copy of the License at:
  14                     ;
  15                     ;        http://www.st.com/software_license_agreement_liberty_v2
  16                     ;
  17                     ; Unless required by applicable law or agreed to in writing, software 
  18                     ; distributed under the License is distributed on an "AS IS" BASIS, 
  19                     ; WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  20                     ; See the License for the specific language governing permissions and
  21                     ; limitations under the License.
  22                     ;******************************************************************************
  23                     
  24                     	xdef	__block_checksum160, __cktype__
  25                     	xref	__ckdesc__
  26                     	xref.b	c_x
  27                     ;
  28                     	xref.b _CRC_ERROR_s, _CRC_ONGOING_s, _BLOCKSIZE_s
  29                     	xref _STL_FlashCrc16Init
  30                     	xref _CurrentDesc, _CurrentCrc16, _p_RunCrc16Chk
  31                     	xref _CurrentDescInv, _CurrentCrc16Inv, _p_RunCrc16ChkInv
  32                     	xref.b _CRCBlockIndex
  33                     ;
  35                     ;
  36       00000004      __cktype__: equ	4		; 16 bit single CRC
  37                     ;
  38                     	switch	.text
  39                     ;
  40                     ; Private functions ***********************************************************
  41                     ; Initialization
  42  0000               _STL_FlashCrc16Init:
  43  0000 ae0000        	ldw	x,#__ckdesc__	; descriptor address
  44  0003 cf0000        	ldw	_CurrentDesc,x
  45  0006 9093          	ldw y,x
  46  0008 9053          	cplw y
  47  000a 90cf0000      	ldw	_CurrentDescInv,y
  48  000e ee01          	ldw	x,(1,x)		; code address
  49  0010 cf0000        	ldw	_p_RunCrc16Chk,x
  50  0013 53            	cplw x
  51  0014 cf0000        	ldw	_p_RunCrc16ChkInv,x
  52  0017 5f            	clrw x
  53  0018 cf0000        	ldw _CurrentCrc16,x	 ; clear CRC and inverse redundant CRC
  54  001b 53            	cplw x
  55  001c cf0000        	ldw _CurrentCrc16Inv,x
  56  001f 81            	ret
  57                     ;
  58                     ; *****************************************************************************
  59                     ; Run time function
  60  0020               __block_checksum160:
  61  0020 90ce0000        ldw y,_CurrentDescInv ; Check & restore last descriptor address
  62  0024 9053            cplw y
  63  0026 90c30000        cpw y,_CurrentDesc
  64  002a 2665            jrne  crc_err
  65  002c 35000000      	mov _CRCBlockIndex,#_BLOCKSIZE_s	; Counter defining size of block to be tested
  66                     ;
  67  0030               bcld:
  68  0030 907d          	tnz	(y)		; test descriptor flag
  69  0032 274e          	jreq	fini		      ; end of the list, exit
  70                     ;  
  71  0034 ce0000        	ldw x,_CurrentCrc16Inv  ; Check & restore Last CRC
  72  0037 53              cplw x
  73  0038 c30000          cpw x,_CurrentCrc16
  74  003b 2654            jrne  crc_err
  75  003d 9e            	ld a,xh
  76  003e b700          	ld c_x,a
  77  0040 9f            	ld a,xl
  78  0041 ce0000        	ldw x,_p_RunCrc16ChkInv	; Check & restore Address pointer
  79  0044 53              cplw x
  80  0045 c30000          cpw x,_p_RunCrc16Chk
  81  0048 2647            jrne  crc_err
  82                       
  83  004a               bclc:										; Here starts CRC calculation
  84  004a 720e000000    	btjt	c_x,#7,here	    ; get bit 15
  85  004f               here:
  86  004f 49            	rlc	a		              ; and rotate
  87  0050 3900          	rlc	c_x		            ; crc
  88  0052 f8            	xor	a,(x)		          ; accumulate
  89  0053 5c            	incw	x		            ; next byte
  90  0054 90e303        	cpw	x,(3,y)		        ; check end of block
  91  0057 2723          	jreq	nxt_blck	      ; if end of segment compare CRC else continue
  92                     ;  
  93  0059 3a00          	dec _CRCBlockIndex    ; check end of block
  94  005b 26ed            jrne bclc             ; if in block continue else return on_going
  95                     ;  
  96  005d               on_going:
  97  005d 90cf0000      	ldw	_CurrentDesc,y		; Save descriptor address
  98  0061 9053          	cplw y
  99  0063 90cf0000      	ldw	_CurrentDescInv,y
 100  0067 cf0000        	ldw	_p_RunCrc16Chk,x	; Save current address pointer
 101  006a 53            	cplw x
 102  006b cf0000        	ldw	_p_RunCrc16ChkInv,x
 103  006e 97            	ld xl,a								; Save current CRC
 104  006f b600          	ld a,c_x
 105  0071 95            	ld xh,a
 106  0072 cf0000        	ldw _CurrentCrc16,x
 107  0075 53            	cplw x
 108  0076 cf0000        	ldw _CurrentCrc16Inv,x
 109  0079 a600          	ld a,#_CRC_ONGOING_s		; if test not completed
 110  007b 81            	ret
 111                     ;
 112  007c               nxt_blck:  
 113  007c 72a90005      	addw	y,#5		        ; skip to next descriptor
 114  0080 20ae          	jra	bcld		          ; and continue
 115                     ;
 116  0082               fini:
 117  0082 43            	cpl	a		              ; invert low byte
 118  0083 90e802        	xor	a,(2,y)		        ; result should be zero
 119  0086 2609          	jrne	crc_err	        ; if no, error exit
 120  0088 b600          	ld	a,c_x		          ; access high byte
 121  008a 43            	cpl	a		              ; invert it
 122  008b 90e801        	xor	a,(1,y)		        ; result should be zero
 123  008e 2601          	jrne	crc_err	        ; if no, error exit
 124  0090 81            	ret                   ; returns 0 if success
 125                     ;
 126  0091               crc_err:
 127  0091 a600          	ld a,#_CRC_ERROR_s	  ; Do not save context if error
 128  0093 81            	ret
 129                     	end
