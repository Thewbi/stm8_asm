#define _USE_DECLSPECS_FOR_SAL
#define _USE_ATTRIBUTES_FOR_SAL 100

#if _USE_DECLSPECS_FOR_SAL

    int a = 0;

#elif !defined(_USE_ATTRIBUTES_FOR_SAL)

    int b = 0;

#endif

