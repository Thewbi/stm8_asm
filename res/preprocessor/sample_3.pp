//#define EXPR_A (2 + 3)
//#define EXPR_B (EXPR_A * 4)

#define EXPR_B 10
//#define EXPR_B 15
//#define EXPR_B 20

#define EXPR_C 10
//#define EXPR_C 15

//#if EXPR_B == 20
//    // This code WILL be included
//#endif

int main() {
#if EXPR_B == 10

    // This code WILL be included
    //printf("test\n");
    int a = 10;

    #if EXPR_C == 10
        int d = 10;
    #else
        int d = 50;
    #endif

#elif EXPR_B == 20
    int b = 20;
#else
    int c = 30;
#endif

    return 0;
}


