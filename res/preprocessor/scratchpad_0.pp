#include <stdint.h>

struct listnode
{
    uint8_t version;
    uint32_t device_id;
    uint8_t status;
    uint16_t voltage_mv;
    uint8_t channel;
    uint32_t timestamp;
    uint16_t sample_rate;
    uint8_t mode;
    uint8_t flags;
    float temperature;
    uint8_t mac_addr[6];
    uint16_t crc;
};

typedef struct listnode listnode_t;
