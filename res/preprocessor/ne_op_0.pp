#define WINAPI_FAMILY_DESKTOP_APP 1
#define WINAPI_FAMILY_PC_APP 2
#define WINAPI_FAMILY_PHONE_APP 3
#define WINAPI_FAMILY_SYSTEM 4
#define WINAPI_FAMILY_GAMES 5
#define WINAPI_FAMILY_SERVER 6

#define WINAPI_FAMILY 123

#if WINAPI_FAMILY != WINAPI_FAMILY_DESKTOP_APP &&                              \
    WINAPI_FAMILY != WINAPI_FAMILY_PC_APP &&                                   \
    WINAPI_FAMILY != WINAPI_FAMILY_PHONE_APP &&                                \
    WINAPI_FAMILY != WINAPI_FAMILY_SYSTEM &&                                   \
    WINAPI_FAMILY != WINAPI_FAMILY_GAMES &&                                    \
    WINAPI_FAMILY != WINAPI_FAMILY_SERVER
#error Unknown WINAPI_FAMILY value. Was it defined in terms of a WINAPI_PARTITION_* value?
#endif
