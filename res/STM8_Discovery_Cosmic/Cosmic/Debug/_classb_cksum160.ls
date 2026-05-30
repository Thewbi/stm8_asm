   1                     ;	VERIFY 16 BIT SINGLE CHECKSUM
   2                     ;	Copyright (c) 2008 by COSMIC Software
   3                     ;
   4                     ; Modified by STMicroelectronics for class B conformance
   5                     ; Verifies descriptor table integrity
   6                     ; Version            : V1.1.0
   7                     ; Date               : JAN-2011
   8                     ;******************************************************************************
   9                     ;
  10                     ; 				COPYRIGHT 2012 STMicroelectronics
  11                     ;
  12                     ; Licensed under MCD-ST Liberty SW License Agreement V2, (the "License");
  13                     ; You may not use this file except in compliance with the License.
  14                     ; You may obtain a copy of the License at:
  15                     ;
  16                     ;        http://www.st.com/software_license_agreement_liberty_v2
  17                     ;
  18                     ; Unless required by applicable law or agreed to in writing, software 
  19                     ; distributed under the License is distributed on an "AS IS" BASIS, 
  20                     ; WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  21                     ; See the License for the specific language governing permissions and
  22                     ; limitations under the License.
  23                     ;******************************************************************************
  24                     
  25                     	xdef	__classb_checksum160, __cktype__
  26                     	xref	__ckdesc__
  27                     	xref.b	c_x
  29                     ;
  30       00000004      __cktype__: equ	4		; 16 bit single CRC
  31                     ;
  32                     	switch	.text
  33                     ;
  34  0000               __classb_checksum160:
  35  0000 90ae0000      	ldw	y,#__ckdesc__	; descriptor address
  36  0004 907d          	tnz	(y)		; test flag
  37  0006 272c          	jreq	Error				; if zero, indicates descriptor corruption
  38                     ;                     (at least one block should be present!)
  39  0008 4f            	clr	a		; clear crc accumulator
  40  0009 b700          	ld	c_x,a
  41                     ;
  42  000b               bcld:
  43  000b 907d          	tnz	(y)		; test flag
  44  000d 2718          	jreq	fini		; end of list, exit
  45  000f 93            	ldw	x,y
  46  0010 ee01          	ldw	x,(1,x)		; code address
  47  0012               bclc:
  48  0012 720e000000    	btjt	c_x,#7,here	; get bit 15
  49  0017               here:
  50  0017 49            	rlc	a		; and rotate
  51  0018 3900          	rlc	c_x		; crc
  52  001a f8            	xor	a,(x)		; accumulate
  53  001b 5c            	incw	x		; next byte
  54  001c 90e303        	cpw	x,(3,y)		; check end of block
  55  001f 26f1          	jrne	bclc		; no, continue
  56  0021 72a90005      	addw	y,#5		; skip to next descriptor
  57  0025 20e4          	jra	bcld		; and continue
  58                     ;
  59  0027               fini:
  60  0027 43            	cpl	a		; invert low byte
  61  0028 90e802        	xor	a,(2,y)		; result should be zero
  62  002b 97            	ld	xl,a
  63  002c b600          	ld	a,c_x		; access high byte
  64  002e 43            	cpl	a		; invert it
  65  002f 90e801        	xor	a,(1,y)		; result should be zero
  66  0032 95            	ld	xh,a
  67  0033 81            	ret			; and return
  68                     ;
  69  0034               Error:
  70  0034 aeffff        	ldw x,#0xffff	    ; return error code
  71  0037 81            	ret
  72                     	end
