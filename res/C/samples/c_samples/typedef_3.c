// res\C\samples\c_samples\typedef_3.c

#include <stdint.h>
#include <stdbool.h>

enum FETCH_SOURCE {
    BACKGROUND,
    WINDOW,
    OBJECT
};

typedef struct PIXEL_DATA {
    uint8_t binary_data;
    enum FETCH_SOURCE source;
    bool palette;
    bool priority;
    uint16_t address;
    uint8_t x_pos;
} PIXEL_DATA;

int main() {
    return 0;
}