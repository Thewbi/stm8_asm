int sqrt(int val) {
    int res = 0;
    // The highest power of 4 fitting in a signed 32-bit int
    int bit = 1 << 30;

    // Shift 'bit' down until it is less than or equal to the target value
    while (bit > val) {
        bit >>= 2;
    }

    while (bit != 0) {

        int intermediate = res + bit;
        if (val >= intermediate) {
            val -= intermediate;

            int temp = res >> 1;
            res = temp + bit;
        }
        else {
            res >>= 1;
        }
        bit >>= 2;
    }
    return res;
}
