#define MIDL_PASS
//#undef MIDL_PASS

#define __midl
//#undef __midl

#if defined(MIDL_PASS) || defined(__midl)
    OK
#endif
