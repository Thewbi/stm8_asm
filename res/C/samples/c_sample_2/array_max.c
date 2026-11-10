// res\C\samples\c_sample_2\array_max.c

int main() {

    int nums[] = { 4, 2, 7, 1, 9 };
    int n = 5;
    int max = nums[0];

    int i = 0;
    for (i = 1; i < n; i++) {
        if (nums[i] > max) {
            max = nums[i];
        }
    }

    return max;
}
