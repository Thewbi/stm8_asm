int main() {

    int arr[] = { 64, 34, 25, 12, 22, 11, 90 };

    int n = 6;

    int i;

    int swapped;
    swapped = 1;

    int a;
    int b;

    int next_idx;
    next_idx = 0;

    while (swapped) {

         swapped = 0;

         for (i = 0; i < n; i++) {

             a = arr[i];
             next_idx = i + 1;
             b = arr[next_idx];

             if (a > b) {
                 arr[i] = b;
                 arr[next_idx] = a;

                 swapped = 1;
             }

         }
    }

    if (arr[0] != 11) {
        return 11;
    }
    if (arr[1] != 12) {
        return 12;
    }
    if (arr[2] != 22) {
        return 22;
    }
    if (arr[3] != 25) {
        return 25;
    }
    if (arr[4] != 34) {
        return 34;
    }
    if (arr[5] != 64) {
        return 64;
    }
    if (arr[6] != 90) {
        return 90;
    }

    return 0;
}