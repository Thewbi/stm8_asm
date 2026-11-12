// res\C\samples\c_samples\struct_13.c

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

enum FETCH_SOURCE {
    BACKGROUND,
    WINDOW,
    OBJECT
};

enum PALETTE {
     OBJ_P0,
     OBJ_P1,
     NOT_OBJ
 };

typedef struct PIXEL_DATA {
    uint8_t binary_data;
    enum FETCH_SOURCE source;
    bool palette;
    bool priority;
    uint16_t address;
    uint8_t x_pos;
//} PIXEL_DATA_t;
} PIXEL_DATA;


int main() {
    PIXEL_DATA pd_1;
    //PIXEL_DATA_t pd_2;

    pd_1.x_pos = 17;

    if (pd_1.x_pos == 17) {
        return 0;
    }
    return pd_1.x_pos;
}
