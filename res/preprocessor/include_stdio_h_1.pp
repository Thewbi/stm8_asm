// #ifndef _PREFAST_
// #define _PREFAST_
// #endif
//
// #ifndef _SAL_VERSION
// #define _SAL_VERSION 20
// #endif
//
// // Override SAL macros to expand to nothing
// #ifndef _In_
// #define _In_
// #endif
//
// #ifndef _Out_
// #define _Out_
// #endif
//
// // Force standard header guards to think SAL is already processed
// #ifndef __SAL_H_VERSION
// #define __SAL_H_VERSION 1
// #endif



//#define _M_X64

#define _USE_DECLSPECS_FOR_SAL 0
#define _USE_ATTRIBUTES_FOR_SAL 0

#include <stdio.h>

int main() {
    printf("test\n");
    return 0;
}
