// res\C\samples\c_samples\char_3.c

/* strlen: return length of s */
int strlen(char s[])
{
    int i;
    while (s[i] != '\0')
        ++i;
    return i;
}

int main() {
    return 0;
}