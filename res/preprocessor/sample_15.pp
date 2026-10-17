#define EXPR_A (2 + 3)
#define EXPR_B (EXPR_A * 4)

#if EXPR_B == 20
    // This code WILL be included
#endif

int main() {
#if EXPR_B == 20
    // This code WILL be included
    printf("test\n");
#endif

    return 0;
}