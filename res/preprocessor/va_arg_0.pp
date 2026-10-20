// first line returns value, second line increments pointer
#define va_arg(arg_ptr, data_type) \
    (* (* data_type) arg_ptr); \
    ((* data_type) arg_ptr)++;

Macro:

sum += va_arg(ap, double);
