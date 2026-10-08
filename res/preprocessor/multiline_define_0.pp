#define _Analysis_mode_(mode) \
    typedef _Analysis_mode_impl_(mode) int \
        __GENSYM(__prefast_analysis_mode_flag);
