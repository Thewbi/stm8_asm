// digi_by_digit_sqrt.c

//#include <stdio.h>
#include <stdint.h>
/*
// 1.207569 in 16.16 Fixed Point (1.207569 * 65536)
#define CORDIC_GAIN_INV 79138

int32_t cordic_int_sqrt(int32_t input_val) {
    // Convert integer input into 16.16 fixed-point representation
    int64_t w = ((int64_t)input_val) << 16;

    // Initialize x and y (0.25 in 16.16 fixed point is 1 << 14)
    int64_t x = w + (1 << 14);
    int64_t y = w - (1 << 14);

    int iteration = 1;

    // Perform 16 iterations for a 16-bit precision outcome
    for (int i = 1; i <= 16; i++) {
        int64_t x_shift = x >> i;
        int64_t y_shift = y >> i;

        if (y >= 0) {
            x -= y_shift;
            y -= x_shift;
        }
        else {
            x += y_shift;
            y += x_shift;
        }

        // Hyperbolic rule: Repeat iterations 4, 13...
        if (i == 4 || i == 13) {
            x_shift = x >> i;
            y_shift = y >> i;
            if (y >= 0) {
                x -= y_shift;
                y -= x_shift;
            }
            else {
                x += y_shift;
                y += x_shift;
            }
        }
    }

    // Apply the inverse scale factor and shift back down to plain integer
    int64_t result = (x * CORDIC_GAIN_INV) >> 16; // Multiply by 1.2075
    return (int32_t)(result >> 16);             // Convert fixed-point back to int
}
*/

int32_t digit_by_digit_sqrt(int32_t val) {
    int32_t res = 0;
    int32_t bit = 1 << 30; // The highest power of 4 fitting in a signed 32-bit int

    // Shift 'bit' down until it is less than or equal to the target value
    while (bit > val) {
        bit >>= 2;
    }

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
    return res;
}

int main() {
    int32_t target = 144;
    //printf("Integer Sqrt of %d is %d\n", target, cordic_int_sqrt(target));
    //printf("Integer Sqrt of %d is %d\n", target, digit_by_digit_sqrt(target));

    int result = digit_by_digit_sqrt(target);
    return result;
}

/*
// helloworld.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>

int main()
{
    std::cout << "Hello World!\n";
}

// Run program: Ctrl + F5 or Debug > Start Without Debugging menu
// Debug program: F5 or Debug > Start Debugging menu

// Tips for Getting Started:
//   1. Use the Solution Explorer window to add/manage files
//   2. Use the Team Explorer window to connect to source control
//   3. Use the Output window to see build output and other messages
//   4. Use the Error List window to view errors
//   5. Go to Project > Add New Item to create new code files, or Project > Add Existing Item to add existing code files to the project
//   6. In the future, to open this project again, go to File > Open > Project and select the .sln file
*/
