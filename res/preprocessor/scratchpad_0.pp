// res\C\samples\c_samples\switch_5.c

int main() {
    int selector = 1;
    int a = 0;

    switch (selector) {
        case 1:
            a = a + 11;
            a = a + 22;
            break;

        default:
            break;
    }
    return a;
}
