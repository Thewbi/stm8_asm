// res\C\samples\c_samples\strlen_1.c

int strlen(char *s)
{
    int len = 0;
    char val = 0;
    do {
        val = *s;           // retrieve current char
        s = s + 1;          // increment pointer for next loop
        if (val != 0) {     // increment string length
            len++;
        }
    } while (val != 0);

    return len;
}

int main() {

    int result = strlen("Hello World!");

    return result;
}
