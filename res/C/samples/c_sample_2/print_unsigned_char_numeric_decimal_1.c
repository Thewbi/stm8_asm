// res\C\samples\c_sample_2\print_unsigned_char_numeric_decimal_1.c

int ExitProcess(int c);
int putchar(int c);

void print_unsigned_char_numeric_decimal(unsigned char data) {
    unsigned char buffer[3];
    int text_len = 0;
    while (data > 0) {

        //putchar('a');

        //data = data - 1;

        buffer[text_len] = data % 10;

        //text_len++;
        text_len = text_len + 1;

        data = data / 10;

        if (data == 0) {
            break;
        }
    }

    //int vv = text_len + 48;
    //putchar(vv);

    //putchar('x');

    //int i = text_len-1;
    //for (; i >= 0; i--) {
    //    putchar(buffer[i] + '0');
    //}

    // FIX:
    // xor rax, rax
    // mov al, byte ptr [rbp+-1]
    // cdp
    //
    // mov byte ptr [rbp-16+rax*1], cl

    //int i = 0;
    //for (i = 0; i < text_len; i++) {

    //int i = text_len - 1;
    text_len = text_len - 1;
    int i = 123;
    //for (i = 1; i >= 0; i--) {
    for (i = text_len; i >= 0; i--) {
         //putchar('c');
         //putchar(buffer[i] + '0');

         int temp = buffer[i];
         int vv = temp + 48;
         putchar(vv);
    }
}

int main() {
    unsigned char a = 130;
    //unsigned char a = 23;
    //unsigned char a = 123;
    //unsigned char a = 0;
    //unsigned char a = 0xFF;

    print_unsigned_char_numeric_decimal(a);

    ExitProcess(0);

    return 0;
}