// res\C\samples\c_samples\enum_6.c

enum speed {
    LOW_SPEED,
    MID_SPEED,
    MAX_SPEED
};

enum second_enum {
    FIRST,
    SECOND,
    THIRD,
    FOURTH
};

int main() {

    enum speed var_1 = MID_SPEED;

    switch (var_1) {
        case LOW_SPEED:
            return 1;

        case MID_SPEED:
            return 2;

        case MAX_SPEED:
            return 3;
    }

    return 0;
}
