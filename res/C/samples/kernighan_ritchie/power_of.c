int power(int base, int n) {
    int i;
    int p;

    p = 1;
    for (i = 1; i <= n; ++i) {
        p = p * base;
    }
    return p;
}

int main() {
    int result = power(2, 4);

    return result;
}