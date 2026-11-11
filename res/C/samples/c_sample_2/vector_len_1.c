// res\C\samples\c_sample_2\vector_len.c

#include <math.h>

// Funktion zur Berechnung der Vektorlaenge
double calculate_vector_length(int x, int y, int z) {

    int temp_x = x * x; // 2*2 = 4
    int temp_y = y * y; // 3*3 = 9
    int temp_z = z * z; // 4*4 = 16

    //int temp = x * x + y * y + z * z;

    int temp = temp_x + temp_y; // expected: 13
    temp = temp + temp_z; // expected: 29

    int result = sqrt(temp); // sqrt(29) == 5, as 4*4==16, 5*5==25, 6*6==36

    return result;

    //return temp;

    //return x; // expected: 2
    //return y; // expected: 3
    //return z; // expected: 4

    //return temp_x; // 4
    //return temp_y; // 9
    //return temp_z; // 16
}

int main() {

    int x = 2;
    int y = 3;
    int z = 4;

    int result = calculate_vector_length(x, y, z);

    if (result == 5) {
        return 0;
    }
    return result;
}
