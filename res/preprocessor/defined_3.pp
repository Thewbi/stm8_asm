#define _USE_DECLSPECS_FOR_SAL 1
#undef _USE_DECLSPECS_FOR_SAL

#define MIDL_PASS
#undef MIDL_PASS

#if _USE_DECLSPECS_FOR_SAL && ( defined(MIDL_PASS) || defined(__midl) || defined(RC_INVOKED) || !defined(_PREFAST_) )
    OK
#endif
