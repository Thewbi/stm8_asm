// res\C\samples\c_sample_2\factorial_iterative.c

int main() {

    int result = 1;
    int i;
    int n = 10;

    for (i = 2; i <= n; i++) {
        //result *= i;
        result = result * i;
    }

    return result;
}
