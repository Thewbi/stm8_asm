// res\C\samples\c_sample_2\fibonacci_iterative.c

int main() {

    int n = 6;

    // n = 1  2  3  4  5  6  7   8   9  10  11  12
    //     0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
    int i = 0;
    int first = 0;
    int second = 1;
    int next = 0;
    for (i = 1; i <= n; i++) {
        if (n == 1) {
            return first;
        }
        if (n == 2) {
            return second;
        }

        if (i >= 3) {
            next = first + second;
            if (i == n) {
                //return next;
                break;
            }
            first = second;
            second = next;
        }
    }

    if (next == 5) {
        return 0;
    }
    return next;
}
