// res\C\samples\c_sample_2\print_unsigned_char_numeric_hexadecimal_0.c

int putchar(int c);

void print_unsigned_char_numeric_hexadecimal(unsigned char data) {
    unsigned char buffer[3];
    int text_len = 0;
    while (data >= 0) {
        buffer[text_len] = data % 16;
        text_len++;
        data = data / 16;
        if (data == 0) {
            break;
        }
    }

    unsigned char characters[16];
    characters[0] = '0';
    characters[1] = '1';
    characters[2] = '2';
    characters[3] = '3';
    characters[4] = '4';
    characters[5] = '5';
    characters[6] = '6';
    characters[7] = '7';
    characters[8] = '8';
    characters[9] = '9';
    characters[10] = 'A';
    characters[11] = 'B';
    characters[12] = 'C';
    characters[13] = 'D';
    characters[14] = 'E';
    characters[15] = 'F';

    int i = text_len - 1;
    for (; i >= 0; i--) {
        unsigned char output_char = characters[buffer[i]];
        putchar(output_char);
    }
}

int main() {
    //unsigned char a = 23;
    //unsigned char a = 123;
    //unsigned char a = 255;
    //unsigned char a = 0;
    unsigned char a = 0xFF;

    //print_unsigned_char_numeric_decimal(a);


    print_unsigned_char_numeric_hexadecimal(a);

    return 0;
}