#define __DEFINE_CPP_OVERLOAD_STANDARD_FUNC_0_0(_ReturnType, _ReturnPolicy, _DeclSpec, _FuncName, _SalAttributeDst, _DstType, _Dst) \
    __DEFINE_CPP_OVERLOAD_STANDARD_FUNC_0_0_EX(_ReturnType, _ReturnPolicy, _DeclSpec, _FuncName, _FuncName##_s, _SalAttributeDst, _DstType, _Dst)

#define __DEFINE_CPP_OVERLOAD_STANDARD_FUNC_0_0_EX(_ReturnType, _ReturnPolicy, _DeclSpec, _FuncName, _SecureFuncName, _SalAttributeDst, _DstType, _Dst) \
            __inline \
            _ReturnType __CRTDECL __insecure_##_FuncName(_SalAttributeDst _DstType *_Dst) \
            { \
                _ReturnType __cdecl _FuncName(_SalAttributeDst _DstType *_Dst); \
                return _FuncName(_Dst); \
            } \
            extern "C++" \
            { \
            template <typename _T> \
            inline \
            _CRT_INSECURE_DEPRECATE(_SecureFuncName) \
            _ReturnType __CRTDECL _FuncName(_T &_Dst) _CRT_SECURE_CPP_NOTHROW \
            { \
                return __insecure_##_FuncName(static_cast<_DstType *>(_Dst)); \
            } \
            template <typename _T> \
            inline \
            _CRT_INSECURE_DEPRECATE(_SecureFuncName) \
            _ReturnType __CRTDECL _FuncName(const _T &_Dst) _CRT_SECURE_CPP_NOTHROW \
            { \
                return __insecure_##_FuncName(static_cast<_DstType *>(_Dst)); \
            } \
            template <> \
            inline \
            _CRT_INSECURE_DEPRECATE(_SecureFuncName) \
            _ReturnType __CRTDECL _FuncName(_SalAttributeDst _DstType * &_Dst) _CRT_SECURE_CPP_NOTHROW \
            { \
                return __insecure_##_FuncName(_Dst); \
            } \
            template <size_t _Size> \
            inline \
            _ReturnType __CRTDECL _FuncName(_DstType (&_Dst)[_Size]) _CRT_SECURE_CPP_NOTHROW \
            { \
                _ReturnPolicy(_SecureFuncName(_Dst, _Size), _Dst); \
            } \
            template <> \
            inline \
            _CRT_INSECURE_DEPRECATE(_SecureFuncName) \
            _ReturnType __CRTDECL _FuncName<1>(_DstType (&_Dst)[1]) _CRT_SECURE_CPP_NOTHROW \
            { \
                _ReturnPolicy(_SecureFuncName(_Dst, 1), _Dst); \
            } \
            }



MACRO:

__DEFINE_CPP_OVERLOAD_STANDARD_FUNC_0_0(
        _Success_(return != 0)
        wchar_t*, __RETURN_POLICY_DST, _ACRTIMP, _wtmpnam,
        _Pre_maybenull_ _Always_(_Post_z_), wchar_t, _Buffer
        )
