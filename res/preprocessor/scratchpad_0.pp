// res\C\samples\c_samples\bool_1.c

#include <stdbool.h>

int main() {

    bool done = false;

    int i = 0;
    while (!done) {
        if (i == 10) {
            done = true;
        }
        i++;
    }

    return i;
}
