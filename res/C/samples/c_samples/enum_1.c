// res\C\samples\c_samples\enum_1.c

//#include <stdio.h>

// an anonymous enum basically defines constants.
// The enum fields can now be used as indices into arrays
// or for any other purposes where constants might be
// usefull
enum {
    OPTION_A = 0,
    OPTION_B = 1
};

enum Level {
	LOW,
	MEDIUM,
	HIGH
};

int main() {

	// create an enum variable and assign a value to it
	enum Level level_var = MEDIUM;

    // in C, an enum (enumeration) is fundamentally just an integer
    // under the hood, and the compiler will let you assign arbitrary
    // integer values to it. However, an enum is not meant for data security
    // or strict type enforcement; instead, it is a tool for readability,
    // maintainability, and code organization.

	level_var = (enum Level) 40;
    level_var = 40;

    int dog = HIGH;

	// print the enum variable
	//printf("%d", level_var);

	return 0;
}
