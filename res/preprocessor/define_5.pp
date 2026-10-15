#define _SAL2_Source_(Name, args, annotes)          _SA_annotes3(SAL_name, #Name, "", "2") _GrouP_(annotes _SAL_nop_impl_)

#define _In_reads_or_z_opt_(size)      _SAL2_Source_( \
                                            _In_reads_or_z_opt_, \
                                            (size), \
                                            _In_opt_ _When_(_String_length_(_Curr_) < (size), _Pre_z_) _When_(_String_length_(_Curr_) >= (size), _Pre1_impl_(__count_impl(size))) \
                                       )

Macro: _In_reads_or_z_opt_(100)
