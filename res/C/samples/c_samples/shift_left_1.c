// res\C\samples\c_samples\shift_left_1.c

int main() {

    int arr[3];

    arr[0] = 1;
    arr[1] = 2;
    arr[2] = 3;

    int temp = 0;

    temp = (arr[0] << 8) + arr[1]; // 0x0102 == 258dec

    return temp;
}


