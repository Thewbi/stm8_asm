// res\C\samples\c_samples\function_call_1.c

int add_func(int x, int y);

int main() {
    int i = 0;
    i = add_func(1, 2);

    if (i == 3) {
        return 0;
    }
    return i;
}

int add_func(int x, int y) {
    return x + y;
}
