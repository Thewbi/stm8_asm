// res\C\samples\c_samples\break_0.c

#include <stdbool.h>

int main() {

    bool done = false;

    int i = 0;
    while (!done) {
        if (i == 10) {
            break;
            done = true;
        }
        i++;
    }

    return 0;
}
