   1                     ;****************** (C) COPYRIGHT 2017 STMicroelectronics ********************
   2                     ; File Name          : stm8_stl_cpustart_CSMC.s
   3                     ; Description        : This file contains STM8 CPU test function executed at 
   4                     ;                                           Start-up.
   5                     ; Author             : STMicroelectronics - MCD Application Team
   6                     ; Version            : V2.0.0
   7                     ; Date               : Dec-2017
   8                     ;*****************************************************************************
   9                     ; Redistribution and use in source and binary forms, with or without modification,
  10                     ; are permitted provided that the following conditions are met:
  11                     ;   1. Redistributions of source code must retain the above copyright notice,
  12                     ;      this list of conditions and the following disclaimer.
  13                     ;   2. Redistributions in binary form must reproduce the above copyright notice,
  14                     ;      this list of conditions and the following disclaimer in the documentation
  15                     ;      and/or other materials provided with the distribution.
  16                     ;   3. Neither the name of STMicroelectronics nor the names of its contributors
  17                     ;      may be used to endorse or promote products derived from this software
  18                     ;      without specific prior written permission.
  19                     ;
  20                     ; THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
  21                     ; AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
  22                     ; IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  23                     ; DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
  24                     ; FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
  25                     ; DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
  26                     ; SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
  27                     ; CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
  28                     ; OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
  29                     ; OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
  30                     ;*****************************************************************************
  31                     ;
  32                         switch .text
  33                     ;
  34                         xdef _STL_StartUpCPUTest
  35                         xref _FailSafe
  36                         xref _CtrlFlowCnt
  37                         xref _CtrlFlowCntInv
  38                     ;
  39                     ;*******************************************************************************
  40                     ; Function Name  : STL_StartUpCPUTest
  41                     ; Description    : Full STM8 CPU test at start-up
  42                     ; Input          : None.
  43                     ; Output         : Jump directly to a Fail Safe routine in case of failure
  44                     ; Return         : SUCCESS (=1)
  45                     ; WARNING        : all registers destroyed when exiting this function
  46                     ;                 (excluding stack pointer)
  47                     ;*******************************************************************************/
  48  0000               _STL_StartUpCPUTest:
  49  0000 ce0000            LDW   X,_CtrlFlowCnt
  50  0003 1c0003            ADDW  X,#3          ; CtrlFlowCnt += CPU_INIT_CALLEE
  51  0006 cf0000            LDW   _CtrlFlowCnt,x
  52                         ; If X not functional, corruption will be detected later
  53                     ;
  54                         ; Check flags of code condition register
  55  0009 4f                CLR   A             ; Set Z(ero) Flag
  56  000a 2704aca200a2      JRNE  ErrorCPU      ; Fails if Z=0
  57  0010 a601              LD    A,#1          ; Reset Z Flag
  58  0012 27f8              JREQ  ErrorCPU      ; Fails if Z=1
  59                     ;   
  60  0014 a002              SUB   A,#2          ; Set N(egative) Flag (A=0xFF)
  61  0016 2af4              JRPL  ErrorCPU      ; Fails if N=0
  62  0018 ab02              ADD   A,#2          ; Reset N and set C Flags (Res=0x101)
  63  001a 2bf0              JRMI  ErrorCPU      ; Fails if N=1
  64                     ;
  65  001c 98                RCF                 ; Reset C(arry) Flag
  66  001d 25ed              JRC   ErrorCPU      ; Fails if C=1
  67  001f 99                SCF                 ; Set C(arry) Flag
  68  0020 24ea              JRNC  ErrorCPU      ; Fails if C=0
  69                     ;
  70  0022 ab0f              ADD   A,#$0F        ; Set H(alf) carry Flag (A=0x10)
  71  0024 90287b            JRNH  ErrorCPU      ; Fails if H=0
  72  0027 ab0f              ADD   A,#$0F        ; Reset H(alf) carry Flag (A=0x1F)
  73  0029 902976            JRH   ErrorCPU      ; Fails if H=1
  74                     ;
  75  002c a680              LD    A,#$80
  76  002e ab80              ADD   A,#$80        ; Set V (oVerflow) Flag (Res=0x100)
  77  0030 2870              JRNV  ErrorCPU      ; Fails if V=0
  78  0032 9c                RVF                 ; Reset V Flag
  79  0033 296d              JRV   ErrorCPU      ; Fails if V=1
  80                     ;
  81  0035 9b                SIM                 ; Set interrupt mask
  82  0036 902c69            JRNM  ErrorCPU      ; Fails if I0=0
  83  0039 9a                RIM                 ; Reset I0 bit
  84  003a 902c02            JRNM  skip          ; If I0=O, skip next instruction
  85  003d 2063              JRA   ErrorCPU
  86  003f               skip:
  87  003f 9b                SIM                 ; Set I0 bit (mask interrupts)
  88                     ;   
  89                         ; Check CPU register: A, X, Y
  90  0040 a6aa              LD    A,#$AA
  91  0042 a1aa              CP    A,#$AA
  92  0044 265c              JRNE  ErrorCPU
  93  0046 a655              LD    A,#$55
  94  0048 a155              CP    A,#$55
  95  004a 2656              JRNE  ErrorCPU
  96  004c a611              LD    A,#$11
  97                     ;   
  98  004e aeaaaa            LDW   X,#$AAAA
  99  0051 a3aaaa            CPW   X,#$AAAA
 100  0054 264c              JRNE  ErrorCPU
 101  0056 ae5555            LDW   X,#$5555
 102  0059 a35555            CPW   X,#$5555
 103  005c 2644              JRNE  ErrorCPU
 104  005e ae1234            LDW   X,#$1234
 105                     ;
 106  0061 90aeaaaa          LDW   Y,#$AAAA
 107  0065 90a3aaaa          CPW   Y,#$AAAA
 108  0069 2637              JRNE  ErrorCPU
 109  006b 90ae5555          LDW   Y,#$5555
 110  006f 90a35555          CPW   Y,#$5555
 111  0073 262d              JRNE  ErrorCPU
 112  0075 90ae5678          LDW   Y,#$5678
 113                     ;   
 114                         ; Verify ramp pattern
 115  0079 a111              CP    A,#$11
 116  007b 2625              JRNE  ErrorCPU
 117  007d a31234            CPW   X,#$1234
 118  0080 2620              JRNE  ErrorCPU
 119  0082 90a35678          CPW   Y,#$5678
 120  0086 261a              JRNE  ErrorCPU
 121                     ;   
 122                         ; Check Stack pointer
 123  0088 9096              LDW   Y,SP            ; Save current stack pointer in Y
 124  008a ae5555            LDW   X,#$5555
 125  008d 94                LDW   SP,X
 126  008e 96                LDW   X,SP
 127  008f a35555            CPW   X,#$5555
 128  0092 260e              JRNE  ErrorCPU
 129  0094 aeaaaa            LDW   X,#$AAAA
 130  0097 94                LDW   SP,X
 131  0098 96                LDW   X,SP
 132  0099 a3aaaa            CPW   X,#$AAAA
 133  009c 2604              JRNE  ErrorCPU
 134  009e 9094              LDW   SP,Y            ; Restore stack pointer
 135  00a0 2006              JP    Exit
 136  00a2               ErrorCPU:
 137  00a2 ae0000            LDW   X,#0            ; Error Code if spurious return
 138  00a5 cc0000            JP    _FailSafe
 139                     ;
 140  00a8               Exit:
 141  00a8 ce0000            LDW   X,_CtrlFlowCntInv
 142  00ab 1d0003            SUBW  X,#3            ; CtrlFlowCntInv -= CPU_INIT_CALLEE
 143  00ae cf0000            LDW   _CtrlFlowCntInv,X
 144  00b1 a601              LD    A,#1            ; Returns success
 145  00b3 81                RET
 146                     ;
 147                         END
