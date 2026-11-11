// res\C\samples\c_samples\struct_13.c

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

enum PALETTE {
     OBJ_P0,
     OBJ_P1,
     NOT_OBJ
 };

struct PIXEL_DATA {
    uint8_t binary_data;
    enum FETCH_SOURCE source;
    bool palette;
    bool priority;
    uint16_t address;
    uint8_t x_pos;
};

//typedef struct PIXEL_DATA PIXEL_DATA_t;


int main() {
    return 0;
}
