// digi_by_digit_sqrt.c

#include <stdint.h>

/**/
//int32_t digit_by_digit_sqrt(int32_t val) {
int32_t digit_by_digit_sqrt(int val) {
    int32_t res = 0;


    int32_t bit = 1 << 30; // The highest power of 4 fitting in a signed 32-bit int

    // Shift 'bit' down until it is less than or equal to the target value
    while (bit > val) {
        bit >>= 2;
    }

    /*
    while (bit != 0) {
        if (val >= res + bit) {
            val -= res + bit;
            res = (res >> 1) + bit;
        }
        else {
            res >>= 1;
        }
        bit >>= 2;
    }
    */

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
    //int target = 144;
    int32_t target = 144;
    //return target;
    //printf("Integer Sqrt of %d is %d\n", target, cordic_int_sqrt(target));
    //printf("Integer Sqrt of %d is %d\n", target, digit_by_digit_sqrt(target));

    int result = 0;
    result = digit_by_digit_sqrt(target);
    return result;
}
