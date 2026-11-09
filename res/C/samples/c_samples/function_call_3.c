// res\C\samples\c_samples\function_call_0.c

int add_func(int x, int y) {
    return x + y;
}

int main() {
    int parameter_a = 1;
    int parameter_b = 2;
    int i = add_func(parameter_a, parameter_b);

    if (i == 3) {
        return 0;
    }
    return i;
}
