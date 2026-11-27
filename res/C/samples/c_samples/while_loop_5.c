// res\C\samples\c_samples\while_loop_5.c

int putchar(int c);

int main() {

    unsigned char data = 128;

    putchar('a');

    //while (data >= 0) {
    while (data != 0) {
        putchar('d');

        data = data - 1;
    }

    putchar('f');

    return 0;
}
