// res\C\samples\c_samples\function_call_0.c

int add_func(int x, int y) {
    return x + y;
}

int main() {
    int i = 0;
    i = add_func(2, 2);

    if (i == 3) {
        return 0;
    }
    return i;
}
