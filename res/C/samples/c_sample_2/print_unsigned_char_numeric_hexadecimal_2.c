// res\C\samples\c_sample_2\print_unsigned_char_numeric_hexadecimal_2.c

int putchar(int c);

int main() {

    unsigned char buffer[3];
    //buffer[0] = 10;
    //buffer[1] = 15;

    buffer[0] = 0;
    buffer[1] = 0;
    buffer[2] = 0;

    //unsigned char data = 1; // 0x01
    //unsigned char data = 11; // 0x0B
    //unsigned char data = 15; // 0x0F
    //unsigned char data = 16; // 0x10
    //unsigned char data = 17; // 0x11
    //unsigned char data = 64; // 0x40
    //unsigned char data = 127; // 0x7F
    //unsigned char data = 128; // 0x80
    //unsigned char data = 168; // 0xA8
    unsigned char data = 255; // 0xFF

    unsigned char remainder = 0;
    unsigned char quotient = 0;

    int text_len = 0;
    //while (data > 0) {
    while (data != 0) {

        remainder = data % 16;
        buffer[text_len] = remainder;
        data = data / 16;
        text_len = text_len + 1;

        if (data == 0) {
            break;
        }
    }

    // in reverse order, walk through the buffer
    int i = 0;
    for (i = text_len; i > 0; i--) {

        // retrieve the number to output
        //unsigned char index = buffer[i-1];
        int index = buffer[i-1];
        int vv = 0;

        // convert decimal number to hex digit
        if ((index >= 0) && (index <= 9)) {

            vv = index + 48;

        } else if ((index >= 10) && (index <= 15)) {

            // 65 is the ASCII code for 'A'
            // As we want to map the number 10 to 'A'
            // first subtract 10 to arrive at an offset from zero,
            // then add 65 to arrive at an offset from 'A'

            //vv = index - 10 + 65; // this does not work!

            vv = index + 55; // this works!

        }

        putchar(vv);
    }

    return 0;
}