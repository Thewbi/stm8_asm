// This preprocessor script makes sure that _USE_ATTRIBUTES_FOR_SAL is defined (values differ)
//
// if _USE_DECLSPECS_FOR_SAL is not defined or defined without value or defined to 0,
// and _USE_ATTRIBUTES_FOR_SAL is not defined,
// _USE_ATTRIBUTES_FOR_SAL will be 1
//
// if _USE_DECLSPECS_FOR_SAL is not defined or defined without value or defined to 0,
// and _USE_ATTRIBUTES_FOR_SAL is defined without value or defined to 0,
// _USE_ATTRIBUTES_FOR_SAL will be 0
//
// if _USE_DECLSPECS_FOR_SAL is defined and the definition is non-zero,
// _USE_ATTRIBUTES_FOR_SAL will be 0
//
// if _USE_DECLSPECS_FOR_SAL is not defined or defined without value or defined to 0,
// and _USE_ATTRIBUTES_FOR_SAL is defined with value x,
// _USE_ATTRIBUTES_FOR_SAL will be x

//#define _USE_DECLSPECS_FOR_SAL
//#define _USE_DECLSPECS_FOR_SAL 0
//#define _USE_DECLSPECS_FOR_SAL 1

//#define _USE_ATTRIBUTES_FOR_SAL
//#define _USE_ATTRIBUTES_FOR_SAL 100

#if _USE_DECLSPECS_FOR_SAL // [

    #undef _USE_ATTRIBUTES_FOR_SAL
    #define _USE_ATTRIBUTES_FOR_SAL 0

#elif !defined(_USE_ATTRIBUTES_FOR_SAL) // ][

    #define _USE_ATTRIBUTES_FOR_SAL 1

#endif // ]

USE_ATTRIBUTES_FOR_SAL is: _USE_ATTRIBUTES_FOR_SAL
