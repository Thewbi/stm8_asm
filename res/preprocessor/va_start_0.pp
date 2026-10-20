#define va_start(arg_ptr, last_fixed_param) \
    arg_ptr = &last_fixed_param; \
    arg_ptr += sizeof(last_fixed_param);

Macro:

va_start(ap, format);