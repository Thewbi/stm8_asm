// res\C\samples\c_samples\mod_0.c

int putchar(int c);

int main() {

    //unsigned char data = 1; // 0x01
    //unsigned char data = 11; // 0x0B
    //unsigned char data = 15; // 0x0F
    //unsigned char data = 16; // 0x10
    //unsigned char data = 17; // 0x11
    //unsigned char data = 64; // 0x40
    //unsigned char data = 127; // 0x7F
    unsigned char data = 128; // 0x80

    unsigned char remainder = 0;
    unsigned char quotient = 0;

    // outputs 0
    remainder = data % 16;
    remainder = remainder + 48;
    putchar(remainder);
    putchar(13);
    putchar(10);

    // outputs 8
    quotient = data / 16;
    quotient = quotient + 48;
    putchar(quotient);
    putchar(13);
    putchar(10);

    // shift one hex digit to the right
    data = data / 16;

    // outputs 8
    remainder = data % 16;
    remainder = remainder + 48;
    putchar(remainder);
    putchar(13);
    putchar(10);

    // outputs 0
    quotient = data / 16;
    quotient = quotient + 48;
    putchar(quotient);
    putchar(13);
    putchar(10);

    // shift one hex digit to the right
    data = data / 16;

    // outputs 0
    remainder = data % 16;
    remainder = remainder + 48;
    putchar(remainder);
    putchar(13);
    putchar(10);

    // outputs 0
    quotient = data / 16;
    quotient = quotient + 48;
    putchar(quotient);
    putchar(13);
    putchar(10);

    return 0;
}