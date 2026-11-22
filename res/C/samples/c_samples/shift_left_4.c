// res\C\samples\c_samples\shift_left_4.c

int main() {

    char arr[20];

    arr[0] = 0x00;
    arr[1] = 0x01;
    arr[2] = 0x02;
    arr[3] = 0x03;
    arr[4] = 0x04;
    arr[5] = 0x05;
    arr[6] = 0x06;
    arr[7] = 0x07;
    arr[8] = 0x08;
    arr[9] = 0x09;
    arr[10] = 0x50;
    arr[11] = 0x01;

    int temp = 0;
    int offset = 10;

    temp = (arr[offset+1] << 8) + arr[offset+0]; // 0x0150 == 336dec

    return temp;
}


