// res\C\samples\c_sample_2\print_unsigned_char_numeric_hexadecimal_1.c

int putchar(int c);

void print_unsigned_char_numeric_hexadecimal(unsigned char data) {
    unsigned char buffer[3];
    int text_len = 0;
    while (data >= 0) {
        buffer[text_len] = data % 16;
        //text_len++;
        text_len = text_len + 1;
        data = data / 16;
        if (data == 0) {
            break;
        }
    }

/*
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
*/

    //unsigned char output_char = characters[text_len];
    //putchar(output_char);

    int i = text_len - 1;
    for (; i >= 0; i--) {

        unsigned char index = buffer[i];

        int vv = index;
        if ((index >= 0) && (index <= 9)) {
            vv = index + 48;
        } else if ((index >= 10) && (index <= 15)) {
            vv = index + 65 - 10;
        }
        // else {
        //    vv = 48; // 0
        //}

        putchar(vv);

        //int vv = index + 48;
        //int vv = index + 65 - 10;
        //putchar(vv);

        //int vv = characters[index];
        //putchar(vv);

        //putchar('a');
        //unsigned char output_char = characters[index];

        //unsigned char output_char = characters[0];
        //putchar(output_char);
    }
}

int main() {
    //unsigned char a = 10;
    //unsigned char a = 15;
    //unsigned char a = 23;
    //unsigned char a = 32;
    //unsigned char a = 123;
    unsigned char a = 255;
    //unsigned char a = 0;
    //unsigned char a = 0xFF;

    //print_unsigned_char_numeric_decimal(a);


    print_unsigned_char_numeric_hexadecimal(a);

    return 0;
}