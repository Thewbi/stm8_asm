#pragma once

#define TEST_1

#ifndef TEST_1
#define _SAL_VERSION 20
#endif

int main() {

#if _SAL_VERSION == 20
    int a = 0;
#endif

    return 0;
}
