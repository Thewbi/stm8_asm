// res\C\samples\c_samples\char_3.c

/* strlen: return length of s */
int strlen(char s[])
{
    int i = 0;
    //while (s[i] != '\0') {
    while (s[i] != 0) {
        ++i;
    }
    return i;
}

int main() {

    char arr[100];
    arr[0] = 'a';
    arr[1] = 'b';
    arr[2] = 'c';
    //arr[3] = '\0';
    arr[3] = 0;

    int len = strlen(arr);
    if (len == 3) {
        return 0;
    }
    return len;
}