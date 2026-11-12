// res\C\samples\c_samples\function_call_1.c

int add_func(int x, int y);
//void add_func(int x, int y);
//float add_func(int x, int y);
//int add_func(float x, int y);
//int add_func(int x);

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

//int add_func(int x, int y) {
//    return x + y;
//}
