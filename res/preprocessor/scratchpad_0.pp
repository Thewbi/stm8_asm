// res\C\samples\c_samples\switch_3.c

int main() {
    int selector = 1;
    int a = 0;

    switch (selector) {
        case 1:
            a = a + 1;
            a = a + 2;
        //    a = a + 3;
        //    a = a + 4;
            break;

        case 2:
            break;

        case 3:
            a = a + 10;
            a = a + 11;
            break;

        default:
            a = a + 5;
            a = a + 6;
            break;
    }

    // if (a == 1) {
    //     return 0;
    // }
    return a;
}
