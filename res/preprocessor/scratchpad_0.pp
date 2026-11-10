// res\C\samples\c_sample_2\vector_len.c

#include <math.h>

// Funktion zur Berechnung der Vektorlaenge
double calculate_vector_length(int x, int y, int z) {

    int temp_x = x * x;
    int temp_y = y * y;
    int temp_z = z * z;

    //int temp = x * x + y * y + z * z;

    int temp = temp_x + temp_y; // expected: 13, is: 25 !!!!!
    //temp = temp + temp_z;

    //return sqrt(temp);

    return temp;
}

int main() {

    int x = 2;
    int y = 3;
    int z = 4;

    int result = calculate_vector_length(x, y, z);

    return result;
}
