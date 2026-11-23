// res\C\samples\c_samples\switch_3.c

int main() {
    int selector = 1;
    int a = 0;

    switch (selector) {
        case 1:
            a = a + 1;
            a = a + 1;
            break;
        default:
            break;
    }

    // if (a == 1) {
    //     return 0;
    // }
    return a;
}
