// test
#define add(a, b) a+b

#define SQUARE(x) ((x) * (x))

#define _Analysis_mode_(mode) \
    typedef _Analysis_mode_impl_(mode) int \
        __GENSYM(__prefast_analysis_mode_flag);

#define max(a,b) ((a) >= (b) ? (a) : (b))

#define test test_def

#define ELEMENTS DIMENSION*DIMENSION

#define __crt_countof(_Array) (sizeof(_Array) / sizeof(_Array[0]))

#define _Raises_SEH_exception_         _SAL2_Source_(_Raises_SEH_exception_, (x), _Maybe_raises_SEH_exception_ _Analysis_noreturn_)

#define _Analysis_assume_

//printf("Hello World! %s %s %d", "a"/*a*/, "b"/*b*/, 5/*c*/);
