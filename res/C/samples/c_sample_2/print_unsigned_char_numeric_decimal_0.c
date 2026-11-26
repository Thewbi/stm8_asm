// res\C\samples\c_sample_2\print_unsigned_char_numeric_decimal_0.c

int putchar(int c);

void print_unsigned_char_numeric_decimal(unsigned char data) {
    unsigned char buffer[3];
    int text_len = 0;
    while (data >= 0) {
        buffer[text_len] = data % 10;
        //text_len++;
        text_len = text_len + 1;
        data = data / 10;
        if (data == 0) {
            break;
        }
    }
    int i = text_len-1;
    for (; i >= 0; i--) {
        putchar(buffer[i] + '0');
    }
}

int main() {
    unsigned char a = 23;
    //unsigned char a = 123;
    //unsigned char a = 0;
    //unsigned char a = 0xFF;

    print_unsigned_char_numeric_decimal(a);

    return 0;
}