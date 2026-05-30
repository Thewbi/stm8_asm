   1                     ;****************** (C) COPYRIGHT 2017 STMicroelectronics ********************
   2                     ; File Name          : stm8_stl_cpurun_CSMC.s
   3                     ; Description        : This file contains STM8 CPU test function executed at run 
   4                     ; Author             : STMicroelectronics - MCD Application Team
   5                     ; Version            : V2.0.0
   6                     ; Date               : Dec-2017
   7                     ;*****************************************************************************
   8                     ; Redistribution and use in source and binary forms, with or without modification,
   9                     ; are permitted provided that the following conditions are met:
  10                     ;   1. Redistributions of source code must retain the above copyright notice,
  11                     ;      this list of conditions and the following disclaimer.
  12                     ;   2. Redistributions in binary form must reproduce the above copyright notice,
  13                     ;      this list of conditions and the following disclaimer in the documentation
  14                     ;      and/or other materials provided with the distribution.
  15                     ;   3. Neither the name of STMicroelectronics nor the names of its contributors
  16                     ;      may be used to endorse or promote products derived from this software
  17                     ;      without specific prior written permission.
  18                     ;
  19                     ; THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
  20                     ; AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
  21                     ; IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  22                     ; DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
  23                     ; FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
  24                     ; DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
  25                     ; SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
  26                     ; CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
  27                     ; OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
  28                     ; OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
  29                     ;*****************************************************************************
  30                     ;
  31                         switch .text
  32                     ;
  33                         xdef _STL_RunTimeCPUTest  
  34                         xref _FailSafe
  35                         xref _CtrlFlowCnt
  36                         xref _CtrlFlowCntInv
  37                     ;
  38                     ;*******************************************************************************
  39                     ; Function Name  : STL_RunTimeCPUTest
  40                     ; Description    : Full STM8 CPU test to be executed during run-time
  41                     ; Input          : None.
  42                     ; Output         : jump directly to a Fail Safe routine in case of failure
  43                     ; Return         : SUCCESS (=1)
  44                     ; WARNING        : all registers destroyed when exiting this function
  45                     ;                 (excluding stack pointer)
  46                     ;*******************************************************************************/
  47  0000               _STL_RunTimeCPUTest:
  48  0000 ce0000            LDW   X,_CtrlFlowCnt
  49  0003 1c000b            ADDW  X,#11       ; CtrlFlowCnt += CPU_RUN_CALLEE
  50  0006 cf0000            LDW   _CtrlFlowCnt,x
  51                         ; If X not functional, corruption will be detected later
  52                     ;
  53                         ; Check CPU register: A, X, Y
  54  0009 a6aa              LD    A,#$AA
  55  000b a1aa              CP    A,#$AA
  56  000d 2644              JRNE  ErrorCPU
  57  000f a655              LD    A,#$55
  58  0011 a155              CP    A,#$55
  59  0013 263e              JRNE  ErrorCPU
  60  0015 a611              LD    A,#$11
  61                     ;   
  62  0017 aeaaaa            LDW   X,#$AAAA
  63  001a a3aaaa            CPW   X,#$AAAA
  64  001d 2634              JRNE  ErrorCPU
  65  001f ae5555            LDW   X,#$5555
  66  0022 a35555            CPW   X,#$5555
  67  0025 262c              JRNE  ErrorCPU
  68  0027 ae1234            LDW   X,#$1234
  69                     ;
  70  002a 90aeaaaa          LDW   Y,#$AAAA
  71  002e 90a3aaaa          CPW   Y,#$AAAA
  72  0032 261f              JRNE  ErrorCPU
  73  0034 90ae5555          LDW   Y,#$5555
  74  0038 90a35555          CPW   Y,#$5555
  75  003c 2615              JRNE  ErrorCPU
  76  003e 90ae5678          LDW   Y,#$5678
  77                     ;    
  78                         ; Verify ramp pattern
  79  0042 a111              CP    A,#$11
  80  0044 260d              JRNE  ErrorCPU
  81  0046 a31234            CPW   X,#$1234
  82  0049 2608              JRNE  ErrorCPU
  83  004b 90a35678          CPW   Y,#$5678
  84  004f 2602              JRNE  ErrorCPU
  85  0051 2006              JP Exit
  86  0053               ErrorCPU:
  87  0053 ae0012            LDW   X,#$12         ; Error Code if spurious return
  88  0056 cc0000            JP   _FailSafe
  89                     ;
  90  0059               Exit:
  91  0059 ce0000            LDW   X,_CtrlFlowCntInv
  92  005c 1d000b            SUBW  X,#11         ; CtrlFlowCntInv -= CPU_RUN_CALLEE
  93  005f cf0000            LDW   _CtrlFlowCntInv,X
  94                     ;    
  95  0062 a601              LD    A,#1          ; Returns success
  96  0064 81                RET
  97                     ;    
  98                         END
