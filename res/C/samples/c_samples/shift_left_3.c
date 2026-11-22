// res\C\samples\c_samples\shift_left_3.c

int main() {

    char arr[3];

    arr[0] = 0x50;
    arr[1] = 0x01;

    int temp = 0;

    temp = (arr[1] << 8) + arr[0]; // 0x0150 == 336dec

    return temp;
}


