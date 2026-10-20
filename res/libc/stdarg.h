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
