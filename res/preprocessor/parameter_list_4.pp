#define __DEFINE_CPP_OVERLOAD_STANDARD_FUNC_0_0(_ReturnType, _ReturnPolicy, _DeclSpec, _FuncName, _SalAttributeDst, _DstType, _Dst) \
    ( _ReturnType <+> _ReturnPolicy <+> _DeclSpec <+> _FuncName <+> _SalAttributeDst <+> _DstType <+>_Dst )

__DEFINE_CPP_OVERLOAD_STANDARD_FUNC_0_0( \
        _Success_(return != 0) \
        wchar_t*, __RETURN_POLICY_DST, _ACRTIMP, _wtmpnam, \
        _Pre_maybenull_ _Always_(_Post_z_), wchar_t, _Buffer \
        )
