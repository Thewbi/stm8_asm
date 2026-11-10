int sqrt(int val) {
    int res = 0;
    int bit = 1 << 30; // The highest power of 4 fitting in a signed 32-bit int

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
