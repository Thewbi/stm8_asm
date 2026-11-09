// res\C\samples\c_samples\enum_1.c

//#include <stdio.h>

enum Level {
	LOW,
	MEDIUM,
	HIGH
};

int main() {

	// Create an enum variable and assign a value to it
	enum Level level_var = MEDIUM;

    // In C, an enum (enumeration) is fundamentally just an integer
    // under the hood, and the compiler will let you assign arbitrary
    // integer values to it. However, an enum is not meant for data security
    // or strict type enforcement; instead, it is a tool for readability,
    // maintainability, and code organization.

	level_var = (enum Level) 40;
    level_var = 40;

    int dog = HIGH;

	// Print the enum variable
	//printf("%d", level_var);

	return 0;
}
