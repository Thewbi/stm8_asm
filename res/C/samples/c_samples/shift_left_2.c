// res\C\samples\c_samples\shift_left_2.c

int main() {

    int arr[3];

    arr[0] = 1;
    arr[1] = 2;
    arr[2] = 3;

    int temp = 0;
    int shift_val = 8;

    temp = arr[0] << shift_val;

    return temp;
}


