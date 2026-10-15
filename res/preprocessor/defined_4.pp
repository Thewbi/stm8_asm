#define TEST
//#undef TEST

int main() {

#if defined TEST
    int a = 0;
#endif

    return 0;
}