// res\C\samples\c_sample_2\digit_by_digit_sqrt_1.c

#include <stdint.h>

int32_t digit_by_digit_sqrt(int32_t val) {
    int32_t res = 0;
    // The highest power of 4 fitting in a signed 32-bit int
    int32_t bit = 1 << 30;

    // Shift 'bit' down until it is less than or equal to the target value
    while (bit > val) {
        bit >>= 2;
    }

    while (bit != 0) {

        int intermediate = res + bit;
        if (val >= intermediate) {
            val -= intermediate;

            int temp = res >> 1;
            res = temp + bit;
        }
        else {
            res >>= 1;
        }
        bit >>= 2;
    }
    return res;
}

int main() {
    //int32_t target = 144; // int-sqrt(144) == 12
    int32_t target = 29; // int-sqrt(29) == 5
    //printf("Integer Sqrt of %d is %d\n", target, cordic_int_sqrt(target));
    //printf("Integer Sqrt of %d is %d\n", target, digit_by_digit_sqrt(target));

    int result = digit_by_digit_sqrt(target);

    if (result == 5) {
        return 0;
    }
    return result;
}
