   1                     ;****************** (C) COPYRIGHT 2017 STMicroelectronics ********************
   2                     ; File Name          : stm8_stl_fullRam_CSMC.s
   3                     ; Description        : This file contains the RAM functional to be done at
   4                     ;                      start-up. This test is destructive and will initialize
   5                     ;                      the whole RAM to zero.
   6                     ; Author             : STMicroelectronics - MCD Application Team
   7                     ; Version            : V2.0.0
   8                     ; Date               : Dec-2017
   9                     ;*****************************************************************************
  10                     ; Redistribution and use in source and binary forms, with or without modification,
  11                     ; are permitted provided that the following conditions are met:
  12                     ;   1. Redistributions of source code must retain the above copyright notice,
  13                     ;      this list of conditions and the following disclaimer.
  14                     ;   2. Redistributions in binary form must reproduce the above copyright notice,
  15                     ;      this list of conditions and the following disclaimer in the documentation
  16                     ;      and/or other materials provided with the distribution.
  17                     ;   3. Neither the name of STMicroelectronics nor the names of its contributors
  18                     ;      may be used to endorse or promote products derived from this software
  19                     ;      without specific prior written permission.
  20                     ;
  21                     ; THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
  22                     ; AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
  23                     ; IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  24                     ; DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
  25                     ; FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
  26                     ; DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
  27                     ; SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
  28                     ; CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
  29                     ; OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
  30                     ; OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
  31                     ;*****************************************************************************
  32                     ;   
  33                         switch .text
  34                     ;
  35                         xdef    _STL_FullRamMarchC
  36                     ;   
  37                         xref    _CtrlFlowCntInv
  38                         xref.b _FULL_RAM_OK_s
  39                         xref    _RAM_END_INC_s, _RAM_END_DEC_s
  40                     ;
  41                     ;******************************************************************************
  42                     ;* Function Name  : STL_FullRamMarchC
  43                     ;* Description    : This function verifies that RAM is functional,
  44                     ;*                  using the March C- algorithm.
  45                     ;* Input          : None
  46                     ;* Output         : The whole RAM is initialized with 0 when exiting this fct,
  47                     ;*                  at the exception of CtrlFlowCntInv, set to 0xFFFFFFFF.
  48                     ;* Return         : {ERROR=0; FULL_RAM_SUCCESS=1}
  49                     ;******************************************************************************
  50  0000               _STL_FullRamMarchC:
  51                     ;
  52  0000 9085              POPW  Y             ; Trick: save stacked return adress into Y
  53                     ;
  54                         ; Step 1: Write background with addresses increasing
  55  0002 5f                CLRW  X             ; p = RAM_START
  56  0003               step1:                  ; do {
  57  0003 7f                CLR  (X)            ;   *p = BCKGRND
  58  0004 5c                INCW  X             ; } while (p <= RAM_END)
  59  0005 a30000            CPW   X,#_RAM_END_INC_s 
  60  0008 25f9              JRC   step1 
  61                     ;
  62                         ; Step 2: Verify background and write inv background with addresses increasing
  63  000a 5f                CLRW  X             ; p = RAM_START
  64  000b               step2:                  ; do {
  65  000b f6                LD    A,(X)         ;   if (*p != BCKGRND)
  66  000c 263c              JRNE  Error         ;     Early test termination
  67  000e a6ff              LD    A,#255        ;   *p++ = INV_BCKGRND
  68  0010 f7                LD   (X),A
  69  0011 5c                INCW  X             ; } while (p <= RAM_END)
  70  0012 a30000            CPW   X,#_RAM_END_INC_s     
  71  0015 25f4              JRC   step2
  72                     ;
  73                         ; Step 3: Verify inv background and write background with addresses increasing
  74  0017 5f                CLRW  X             ; p = RAM_START
  75  0018               step3:                  ; do {}
  76  0018 f6                LD    A,(X)         ;   if (*p != INV_BCKGRND)
  77  0019 a1ff              CP    A,#255
  78  001b 262d              JRNE  Error         ;     Early test termination
  79  001d 7f                CLR  (X)            ;   *p++ = BCKGRND
  80  001e 5c                INCW  X             ; } while (p <= RAM_END)
  81  001f a30000            CPW   X,#_RAM_END_INC_s
  82  0022 25f4              JRC   step3
  83                     ;
  84                         ; Step 4: Verify background and write inv background with addresses decreasing
  85  0024 ae0000            LDW   X,#_RAM_END_DEC_s ; p = RAM_END
  86  0027               step4:                  ; do {
  87  0027 f6                LD    A,(X)         ;   if (*p != BCKGRND)
  88  0028 2620              JRNE  Error         ;     Early test termination
  89  002a a6ff              LD    A,#255
  90  002c f7                LD   (X),A          ;   *p-- = INV_BCKGRND
  91  002d 5a                DECW  X
  92  002e 2af7              JRPL  step4         ; } while (p > RAM_START)
  93                     ;
  94                         ; Step 5: Verify inv background and write background with addresses decreasing
  95  0030 ae0000            LDW   X,#_RAM_END_DEC_s ; p = RAM_END
  96  0033               step5:                  ; do {
  97  0033 f6                LD    A,(X)         ;   if (*p != INV_BCKGRND)
  98  0034 a1ff              CP    A,#255
  99  0036 2612              JRNE  Error         ;     Early test termination
 100  0038 7f                CLR  (X)            ;   *p++ = BCKGRND
 101  0039 5a                DECW  X
 102  003a 2af7              JRPL  step5         ; } while (p > RAM_START)
 103                     ;
 104                         ; Step 6: Verify background with addresses increasing
 105  003c 5f                CLRW  X             ; p = RAM_START
 106  003d               step6:                  ; do {
 107  003d f6                LD    A,(X)         ;   if (*p != BCKGRND)
 108  003e 260a              JRNE  Error         ;     Early test termination
 109  0040 5c                INCW  X             ; } while (p <= RAM_END)
 110  0041 a30000            CPW   X,#_RAM_END_INC_s     
 111  0044 25f7              JRC   step6
 112                     ;
 113  0046 a600              LD    A,#_FULL_RAM_OK_s
 114  0048 2001              JP    Exit
 115  004a               Error:
 116  004a 4f                CLR A
 117  004b               Exit:
 118  004b aeffff            LDW   X,#$ffff
 119  004e cf0000            LDW   _CtrlFlowCntInv,x
 120                     ;
 121  0051 9089              PUSHW Y ; Trick: restore stacked return adress from Y
 122  0053 81                RET
 123                     ;
 124                         END
