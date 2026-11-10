// res\C\samples\c_sample_2\vector_len.c

#include <math.h>

// typedef struct {
//     double x;
//     double y;
//     double z;
// } Vector3D;

// Funktion zur Berechnung der Vektorlaenge
double calculate_vector_length(int x, int y, int z) {
    int temp = x * x + y * y + z * z;
    return sqrt(temp);
}

int main() {

    int x = 2;
    int y = 3;
    int z = 4;

    int result = calculate_vector_length(x, y, z);

    return result;
}
