#define MIDL_PASS
//#undef MIDL_PASS

#define __midl
//#undef __midl

#define RC_INVOKED
//#undef RC_INVOKED

#if defined(MIDL_PASS) || defined(__midl) || defined(RC_INVOKED)
    OK
#endif
