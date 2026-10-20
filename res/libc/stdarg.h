//
// ap (argument pointer) implemented as a pointer
//

// #define va_start(ap, last_fixed_param) \
//     ap = &last_fixed_param; \
//     ap += sizeof(last_fixed_param);

// // // first line returns value, second line increments pointer
// // #define va_arg(ap, data_type) \
// //     (* (* data_type) ap); \
// //     ((* data_type) ap)++;

// // first line returns value, second line increments pointer
// #define va_arg(ap, data_type) \
//     (* (* data_type) ap); \
//     ap += sizeof(data_type);




//
// ap (argument pointer) implemented as a struct
//

typedef struct {

    // remember how many parameters have already been retrieve
    // from the first four fast-call parameters
    int register_count;

    // pointer which initially points to the last fixed parameter
    // and is then advanced over all fixed parameters
    void* stack_pointer;

    // size of the last fixed parameter
    int last_fixed_parameter_size;

} va_list;

#define va_start(ap, last_fixed_param) \
    ap.register_count = 0; \
    ap.stack_pointer = &last_fixed_param; \
    ap.last_fixed_parameter_size = sizeof(last_fixed_param);

// #define va_arg(ap, data_type) { \
//     switch (ap.register_count) { \
//         case 0: ap.register_count++; return (data_type) RCX; \
//         case 1: ap.register_count++; return (data_type) RDX; \
//         case 2: ap.register_count++; return (data_type) R8; \
//         case 3: ap.register_count++; return (data_type) R9; \
//         case 4: { \
//             ap.register_count++; \
//             // first advance the pointer over the last fixed parameter
//             ap.stack_pointer += ap.last_fixed_parameter_size; \
//             // return first variable parameter
//             return (* (* data_type) ap.stack_pointer); \
//         } \
//         default: { \
//             // remember address of current variable parameter
//             void* temp = ap.stack_pointer; \
//             // move past the current variable parameter in preparation of the next call
//             ap.stack_pointer += sizeof(data_type); \
//             // return current variable parameter
//             return (* (* data_type) temp); \
//         } \
//     } \
// } \

#define va_arg(ap, data_type) { \
    ap.register_count == 0 ? { ap.register_count++; (data_type) RCX; } \
    ap.register_count == 1 ? { ap.register_count++; (data_type) RDX; } \
    ap.register_count == 2 ? { ap.register_count++; (data_type) R8; } \
        case 2: ap.register_count++; return (data_type) R8; \
        case 3: ap.register_count++; return (data_type) R9; \
        case 4: { \
            ap.register_count++; \
            // first advance the pointer over the last fixed parameter
            ap.stack_pointer += ap.last_fixed_parameter_size; \
            // return first variable parameter
            return (* (* data_type) ap.stack_pointer); \
        } \
        default: { \
            // remember address of current variable parameter
            void* temp = ap.stack_pointer; \
            // move past the current variable parameter in preparation of the next call
            ap.stack_pointer += sizeof(data_type); \
            // return current variable parameter
            return (* (* data_type) temp); \
        } \
    } \
} \

#define va_end(ap) ;





