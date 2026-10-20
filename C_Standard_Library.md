# Specification

The specification is not freely available.
The latest working draft is freely available and virtually identical to the latest released version.

It is available at https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf

The C standard library is referred to in the spec simply as the "Library".
Chapter 7 specifies the "Library".

The set of header files are listed:

```
<assert.h>
<complex.h>
<ctype.h>
<errno.h>
<fenv.h>
<float.h>
<inttypes.h>
<iso646.h>
<limits.h>
<locale.h>
<math.h>
<setjmp.h>
<signal.h>
<stdalign.h>
<stdarg.h>
<stdatomic.h>
<stdbool.h>
<stddef.h>
<stdint.h>
<stdio.h> - page 296
<stdlib.h>
<stdnoreturn.h>
<string.h>
<tgmath.h>
<threads.h>
<time.h>
<uchar.h>
<wchar.h>
<wctype.h>
```






# printf with var args

Source: https://www.firmcodes.com/write-printf-function-c/

In this section, we initialize the arguments of Myprintf( ) function by using standard argument library.

```
#include <stdarg.h>
```
The file <stdarg.h> from the C (Standard) Library contains datatypes, macros and functions for dealing with variable argument lists.
It needs to be included to get access to the data type va_list, the functions va_start(), va_arg(), va_end().

```
va_list arg;
```
This line declares a variable, arg, which we use to iterate over the variable arguments.
The data type of the variable is va_list, a special type defined by <stdarg.h>.

```
va_start(arg, <last_fixed_parameter_name>);
```
The first thing you need to know is that variable arguments have to be written using the ... token and
this token hast to be the last parameter in the list of parameteres of a function.
The next step is used to make arg point to the beginning of the list of variable arguments.
In order to point arg to the beginning, the name of the last fixed argument (the last argument that comes before the variable argument list token ...)
is used in a call to va_start().
This line initializes the arg variable with function’s last fixed argument i.e. the 'format' parameter in the example.
va_start() uses this to figure out where the variable arguments begin.
Implementation:
Remember that arguments are passed on the stack. The va_start function contains the "magic" code to initialize the va_list with the correct stack pointer.
It must be passed the last named argument in the function declaration or it will not work.

```
i = va_arg(arg, <data_type>);
```
va_arg() fetches the next argument from the argument list.
The second parameter to va_arg() is the data type of the argument we expect.
Note: va_arg( ) function will never receive arguments of type char, short int, or float.
va_arg( ) function only accept arguments of type char *, unsigned int, int or double.
Implementation:
What va_arg does is use this saved stack pointer, and extract the correct amount of bytes for the type provided,
and then modify ap so it points to the next argument on the stack.

```
va_end(arg);
```
Finally, when we’re finished processing the all arguments, we call va_end(), which performs any necessary cleanup.


In reality these functions (va_start, va_arg and va_end) are not actually functions,
but implemented as preprocessor macros. The actual implementation also depends on the compiler,
as different compilers can have different layout of the stack and how it pushes arguments on the stack.




But why it is not set to the beginning by default?

>> Maybe because of historical reasons from when compilers weren't smart enough.
Maybe because you might have a varargs function prototype which doesn't actually
care about the varargs and setting up varargs happens to be expensive on that particular system.
Maybe because of more complex operations where you do va_copy or maybe you want to restart working with the arguments multiple times and call va_start multiple times.
The short version is: because the language standard says so.

<< This has absolutely nothing to do with "compilers not being smart enough".
The nature of a variadic function is such that the number of arguments passed
is determined at runtime but the compiler operates at compile time.
It is thus impossible for any compiler, no matter how smart to know the number
of arguments that will be passed to a variadic function. The only way to do this
at compile time would be to reserve an automatic "invisible" parameter for the argument count,
and let the compiler always initialise it, but the designers of C chose not to implement variadic functions this way.

>> Eh? What does the number of arguments have to do with which vararg argument will be the first?
I see no reason for the compiler to not be able to figure out that in foo(int a, ...) the varargs start after a.
Which is what the first question was about and is perfectly well known at compile time.
The count of arguments not being known at compile time I actually mention in the answer,
so I don't understand why you felt the need to repeat it.

<< The compiler could count the args at compile time, but the function itself executes at runtime and the only way
to pass that count from compile time to runtime would be to pass it as an extra argument, possibly hidden,
but nevertheless as an extra argument. It has thus nothing to do with the "smartness" of the compiler but
it has all to do with the parameter passing convention. C simply does not have any such convention by which
the argument count is automatically passed as an extra parameter, instead, C leaves this up to the programmer to do manually.

>> You're talking about the argument count again which is completely irrelevant. As to your second comment,
where the variadic list starts it is perfectly clear at compile time and there's no way to start it at any other point.
With foo(int a, ...) it is perfectly clear that the variadic list starts after a always.
The C standard says "The parameter parmN is the identifier of the rightmost parameter in the variable parameter
list in the function definition (the one just before the , ...).".
Every compiler knows which parameter is the rightmost, "just before the ...".

The second users confuses location of first variable parameter with the amount of variable parameters.

Both users are right to some extend.
The takeaway is, the amount of variable arguments is not transferred from compile-time to runtime.
This means that when calling va_arg(arg, <data_type>), you need to determine yourself how often it
is valid to call va_arg(arg, <data_type>) or in other terms, how many variable arguments really
exist! The compiler will not save you! Calling va_arg(arg, <data_type>); more often than there
are actual variable arguments does not lead to an explicit error! The function will return but
it will return a garbage value after the last variable argument was consumed!

printf() knows the amount of variable arguments by counting the control characters.
Other functions must use a similar mechanism. It is possible to force the user to supply
an extra integer as a parameter which is the amount of variable arguments that the user has entered.

It is not possible to have a function that takes only variable arguments without knowing how many
variable arguments are provided anywhere in the function!





```
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
        switch (*traverse)
        {
            case 'c':   i = va_arg(arg, int);   // The format string dictates: Fetch char argument.
                                                // va_arg() is used to fetch the next variable argument
                        putchar(i);
                        break;

            case 'd':   i = va_arg(arg, int); // Fetch Decimal/Integer argument
                        if (i < 0)
                        {
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
```


```
ERROR: This function returns a pointer to a char array which is a local variable!!!!!!!!!
No, it does not! the buffer is declared static!
This means it loves outside the function!

TODO implement static in the compiler

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
```






https://stackoverflow.com/questions/15784729/an-example-of-use-of-varargs-in-c


```
#include <stdarg.h>

double average(int count, ...)
{
    va_list ap; // argument_pointer
    va_start(ap, count); // Requires the last fixed parameter (to get the address)

    // the developer needs to manage the amount of calls to va_arg() themselves.
    // In this case, the variable 'count' limits the amount of calls to va_arg().
    // The compiler does not provide the information as to how many variable arguments
    // have been specified when average() was called!
    double sum = 0;
    int j;
    for (j = 0; j < count; j++) {
        sum += va_arg(ap, double); // Requires the type to cast to. Increments ap to the next argument.
    }

    va_end(ap); // clean up

    return sum / count;
}
```


# <stdarg.h>

## Implementation where va_list is a pointer

#define va_start(arg_ptr, last_fixed_param) \
arg_ptr = &last_fixed_param; \
arg_ptr += sizeof(last_fixed_param);

// first line returns value, second line increments pointer
#define va_arg(arg_ptr, data_type) \
(* (* data_type) arg_ptr); \
((* data_type) arg_ptr)++;

// first line returns value, second line increments pointer
#define va_arg(arg_ptr, data_type) \
(* (* data_type) arg_ptr); \
arg_ptr += sizeof(data_type);

## Implementation where va_list is a struct

The fastcall calling convention inserts the first four parameters
into the registers RCX, RDX, R8 and R9 in that order. All additional
parameters are placed on the stack.

With fastcall, va_list needs to keep track of two information items
1. The amount of registers read
2. The address of the current variable parameter

These two items are combined into a struct.

```
typedef struct va_list {

    // remember how many parameters have already been retrieve
    // from the first four fast-call parameters
    int register_count;

    // pointer which initially points to the last fixed parameter
    // and is then advanced over all fixed parameters
    void* stack_pointer;

    // size of the last fixed parameter
    int last_fixed_parameter_size;

} va_list_t;
```

```
void va_start(arg_ptr &va_list_t, last_fixed_param &int) {

    // no registers consumed initially
    arg_ptr->register_count = 0;

    // point to last fixed parameter on the stack
    arg_ptr->stack_pointer = last_fixed_param;

    // store size of last fixed parameter
    //
    // TODO: ??? does this return the correct size?
    // TODO: test with char, byte, int, long, float, double
    arg_ptr->last_fixed_parameter_size = sizeof(last_fixed_param);
}
```

```
void* va_arg(va_list_t, data_type) {

    switch va_list_t.register_count {
        case 0: va_list_t.register_count++; return (* data_type) RCX;
        case 1: va_list_t.register_count++; return (* data_type) RDX;
        case 2: va_list_t.register_count++; return (* data_type) R8;
        case 3: va_list_t.register_count++; return (* data_type) R9;
        case 4: {
            va_list_t.register_count++;
            // first advance the pointer over the last fixed parameter
            va_list_t.stack_pointer += va_list_t.last_fixed_parameter_size;
            // return first variable parameter
            return (* (* data_type) va_list_t.stack_pointer);
        }
        default: {
            // remember address of current variable parameter
            void* temp = va_list_t.stack_pointer;
            // move past the current variable parameter in preparation of the next call
            va_list_t.stack_pointer += sizeof(data_type);
            // return current variable parameter
            return (* (* data_type) temp);
        }
    }
}
```
