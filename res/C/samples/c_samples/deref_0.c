// res\C\samples\c_samples\deref_0.c

int deref(char *s)
{
    int val = *s;

    return val;
}

int main() {
    int result = deref("Hello World!");

    return result;
}

