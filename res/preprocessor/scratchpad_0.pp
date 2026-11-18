// res\C\samples\c_samples\print_string_0.c

int putchar(int c);

int print_string(char *s)
{
    int len = 0;
    int val = 0;
    do {
        val = *s;           // retrieve current char
        s = s + 1;          // increment pointer for next loop
        if (val != 0) {     // increment string length
            int result = putchar(val);
            len++;
        }
    } while (val != 0);

    return len;
}


int main() {
    int result = print_string("Am achten Tage erschuf Gott den Compiler! Und er sah, dass er scheisse war!");

    return 0;
}

