// res\C\samples\c_samples\sqrt_0.c

#include <stdint.h>

int32_t digit_by_digit_sqrt(int val) {
    int32_t res = 0;

    // The highest power of 4 fitting in a signed 32-bit int
    int32_t bit = 1 << 30;

    // Shift 'bit' down until it is less than or equal to the target value
    while (bit > val) {
        bit >>= 2;
    }

    while (bit != 0) {
        int temp = res + bit;
        if (val >= temp) {
            val -= temp;
            res = (res >> 1);
            res = res + bit;
        } else {
            res >>= 1;
        }
        bit >>= 2;
    }

    return res;
}


int main() {
    int32_t target = 144;
    int result = digit_by_digit_sqrt(target);
    if (result == 12) {
        return 0;
    }
    return result;
}
