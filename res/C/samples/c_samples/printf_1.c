typedef void* va_list;

#define va_start(arg_ptr, last_fixed_param) \
    arg_ptr = &last_fixed_param; \
    arg_ptr += sizeof(last_fixed_param);

// // first line returns value, second line increments pointer
// #define va_arg(arg_ptr, data_type) \
//     (* (* data_type) arg_ptr); \
//     ((* data_type) arg_ptr)++;

// first line returns value, second line increments pointer
#define va_arg(arg_ptr, data_type) \
    (* (* data_type) arg_ptr); \
    arg_ptr += sizeof(data_type);

char *convert(unsigned int num, int base)
{
    static char representation[] = "0123456789ABCDEF";
    static char buffer[50];
    char *ptr;

    // start at the end of the buffer and zero terminate it
    ptr = &buffer[49];
    *ptr = '\0';

    // loop from right to left, inserting the digits of
    // the number in the requested number base
    do {
        *--ptr = representation[num % base];
        num /= base;
    } while (num != 0);

    return(ptr);
}

void putchar(char data) {

}

void puts(char * data_ptr) {

}

void printf(char* format, ...)
{
    va_list arg;
    va_start(arg, format);

    // This loop iterates over the format string, character by character.
    // This code has nothing to do with variable argument functionality.
    // This loop is pure C code.
    // It does not iterate over the variable arguments!
    char *traverse;
    for (traverse = format; *traverse != '\0'; traverse++)
    {
        // output characters unless a dollar-sign is encountered.
        // The % sign is always followed by a control-character
        // The % helps the application to determine when to call va_arg()
        // because the compiler will not provide the information as to
        // how many variable arguments are available! The source code / app
        // has to keep managing this information itself. The % does exactly that!
        while (*traverse != '%')
        {
            if (*traverse == '\0') {
                return;
            }

            putchar(*traverse);
            traverse++;
        }

        // inspect the current control-character
        // The task is to pull in the next variable argument and print it
        // according to the format that the control character dictates.
        unsigned int i;
	    char *s;
        switch (*traverse)
        {
            case 'c':   i = va_arg(arg, int);   // The format string dictates: Fetch char argument.
                                                // va_arg() is used to fetch the next variable argument
                        putchar(i);
                        break;

            case 'd':   i = va_arg(arg, int); // Fetch Decimal/Integer argument
                        if (i < 0) {
                            i = -i;
                            putchar('-');
                        }
                        puts(convert(i, 10));
                        break;

            case 'o':   i = va_arg(arg, unsigned int); // Fetch Octal representation
                        puts(convert(i, 8));
                        break;

            case 's':   s = va_arg(arg, char *); // Fetch string
                        puts(s);
                        break;

            case 'x':   i = va_arg(arg, unsigned int); // Fetch Hexadecimal representation
                        puts(convert(i, 16));
                        break;
        }

        va_end(arg);    // free resources
    }
}

int main() {

    int x = 100;
    printf("test: %d\n", x);

    return 0;
}
