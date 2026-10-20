// first line returns value, second line increments pointer
#define va_arg(arg_ptr, data_type) \
    (* (* data_type) arg_ptr); \
    arg_ptr += sizeof(data_type);

Macro:

sum += va_arg(ap, double);
