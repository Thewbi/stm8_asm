// res\C\samples\c_samples\switch_2.c

int main() {
    int selector = 3;
    int a = 0;

    switch (selector) {
        case 10:
            a = a + 1;
            break;
        case 20:
            a = a + 2;
            a = a + 3;
            break;
        case 30:
            a = a + 4;
            a = a + 5;
            a = a + 6;
            break;
        default:
            break;
    }

    // if (a == 1) {
    //     return 0;
    // }
    return a;
}
