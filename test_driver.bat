@ECHO OFF

REM https://stackoverflow.com/questions/3942265/errorlevel-in-a-for-loop-windows-batch
setlocal EnableDelayedExpansion



call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64

cls
for %%L in (
    res\C\samples\c_samples\array_0.c
    res\C\samples\c_samples\array_1.c
    res\C\samples\c_samples\array_2.c
    res\C\samples\c_samples\array_3.c
    res\C\samples\c_samples\array_4.c
    res\C\samples\c_samples\array_5.c
    res\C\samples\c_samples\array_6.c
    res\C\samples\c_samples\array_declaration_with_size.c
    res\C\samples\c_samples\array_declaration_without_size.c

    res\C\samples\c_samples\assign_add_assign_0.c
    res\C\samples\c_samples\assign_add_assign_1.c
    res\C\samples\c_samples\assign_mul_assign_0.c
    res\C\samples\c_samples\assign_sub_assign_0.c
    res\C\samples\c_samples\assign_sub_assign_1.c

    res\C\samples\c_samples\binary_0.c
    res\C\samples\c_samples\binary_1.c
    res\C\samples\c_samples\binary_left_shift_0.c
    res\C\samples\c_samples\binary_right_shift_0.c

    res\C\samples\c_samples\bubblesort_1.c
    res\C\samples\c_samples\bubblesort_2.c
    res\C\samples\c_samples\bubblesort_3.c
    res\C\samples\c_samples\bubblesort_broken.c

    res\C\samples\c_samples\do_loop_0.c
    res\C\samples\c_samples\do_loop_1.c

    res\C\samples\c_samples\enum_0.c

    res\C\samples\c_samples\expression_0.c
    res\C\samples\c_samples\expression_1.c
    res\C\samples\c_samples\expression_2.c

    res\C\samples\c_samples\for_loop_0.c
    res\C\samples\c_samples\for_loop_1.c
    res\C\samples\c_samples\for_loop_2.c

    res\C\samples\c_samples\function_call_0.c
    res\C\samples\c_samples\function_call_1.c

    res\C\samples\c_samples\hex_array_0.c
    res\C\samples\c_samples\hex_numeric_0.c
    res\C\samples\c_samples\hex_variable_0.c
    res\C\samples\c_samples\if_0.c
    res\C\samples\c_samples\if_1.c
    res\C\samples\c_samples\if_else_if_0.c
    res\C\samples\c_samples\if_else_if_1.c

    res\C\samples\c_samples\linked_list_0.c
    res\C\samples\c_samples\linked_list_1.c
    res\C\samples\c_samples\linked_list_2.c
    res\C\samples\c_samples\linked_list_3.c
    res\C\samples\c_samples\linked_list_4.c


    res\C\samples\c_samples\main_0.c
    res\C\samples\c_samples\multiplication_0.c
    res\C\samples\c_samples\parameter_pointer_0.c
    res\C\samples\c_samples\pointer_0.c


    res\C\samples\c_samples\return_0.c
    res\C\samples\c_samples\return_1.c

    res\C\samples\c_samples\sqrt_0.c

    res\C\samples\c_samples\struct_0.c
    res\C\samples\c_samples\struct_1.c
    res\C\samples\c_samples\struct_2.c
    res\C\samples\c_samples\struct_3.c
    res\C\samples\c_samples\struct_4.c
    res\C\samples\c_samples\struct_6.c
    res\C\samples\c_samples\struct_7.c
    res\C\samples\c_samples\struct_8.c
    res\C\samples\c_samples\struct_9.c
    res\C\samples\c_samples\struct_10.c
    res\C\samples\c_samples\struct_11.c

    res\C\samples\c_samples\struct_field_access_arrow_0.c




    res\C\samples\c_samples\struct_field_assignment_dot_0.c
    res\C\samples\c_samples\struct_field_assignment_dot_1.c
    res\C\samples\c_samples\struct_field_assignment_ptr_0.c
    res\C\samples\c_samples\struct_field_assignment_ptr_1.c
    res\C\samples\c_samples\struct_field_getAddress_0.c



    res\C\samples\c_samples\sub_0.c
    res\C\samples\c_samples\subscript_0.c
    res\C\samples\c_samples\subscript_1.c

    res\C\samples\c_samples\switch_0.c
    res\C\samples\c_samples\switch_1.c
    res\C\samples\c_samples\test_0.c
    res\C\samples\c_samples\test_1.c
    res\C\samples\c_samples\typedef_0.c
    res\C\samples\c_samples\typedef_1.c
    res\C\samples\c_samples\typedef_2.c

    res\C\samples\c_samples\while_loop_0.c
    res\C\samples\c_samples\while_loop_1.c
    res\C\samples\c_samples\while_loop_2.c
    res\C\samples\c_samples\while_loop_3.c
) do (
    if exist build\main.asm del build\main.asm
    if exist build\main.obj del build\main.obj
    if exist build\main.exe del build\main.exe
    call "target/debug/rust_pest_asm.exe" -i %%L
    REM https://learn.microsoft.com/en-us/cpp/assembler/masm/ml-and-ml64-command-line-reference?view=msvc-170
    REM /c only assemble, no linking
    REM /Fl build\main.lst
    call ml64.exe /Fo build\main.obj /c build\main.asm >NUL 2>&1
    call link.exe /SUBSYSTEM:console /LARGEADDRESSAWARE:NO /ENTRY:main /OUT:build\main.exe build\main.obj >NUL 2>&1
    call build\main.exe
    REM IF "!errorlevel!" == "0" ( echo [SUCCESS] %%L ) ELSE ( echo [FAILURE] %%L & EXIT /b )
    IF "!errorlevel!" == "0" ( echo [SUCCESS] %%L ) ELSE ( echo [FAILURE] %%L )
)

if exist build\main.asm del build\main.asm
if exist build\main.obj del build\main.obj
if exist build\main.exe del build\main.exe

echo [DONE]



REM res\C\samples\c_samples\function_call_0.c
REM    res\C\samples\c_samples\function_call_1.c
REM
REM     res\C\samples\c_samples\linked_list_0.c
REM     res\C\samples\c_samples\linked_list_1.c
REM     res\C\samples\c_samples\linked_list_without_malloc.c
REM
REM     res\C\samples\c_samples\printf_0.c
REM    res\C\samples\c_samples\printf_1.c
REM    res\C\samples\c_samples\printf_2.c
REM
REM
REM
REM
REM
REM es\C\samples\c_samples\variadic_parameter_0.c
REM
REM
REM
REM    res\C\samples\c_samples\quicksort_2.c
REM    res\C\samples\c_samples\quicksort.c
REM
REM
REM    res\C\samples\c_samples\struct_0.c
REM    res\C\samples\c_samples\struct_1.c
REM    res\C\samples\c_samples\struct_2.c
REM    res\C\samples\c_samples\struct_3.c
REM    res\C\samples\c_samples\struct_4.c
REM    res\C\samples\c_samples\struct_6.c
REM    res\C\samples\c_samples\struct_7.c
REM    res\C\samples\c_samples\struct_8.c
REM    res\C\samples\c_samples\struct_9.c
REM    res\C\samples\c_samples\struct_10.c
REM    res\C\samples\c_samples\struct_11.c
REM    res\C\samples\c_samples\struct_12.c
REM    res\C\samples\c_samples\struct_field_access_arrow_0.c
REM    res\C\samples\c_samples\struct_field_access_arrow_1.c
REM    res\C\samples\c_samples\struct_field_access_dot_0.c
REM    res\C\samples\c_samples\struct_field_access_dot_1.c
REM    res\C\samples\c_samples\struct_field_access_dot_2.c
REM    res\C\samples\c_samples\struct_field_assignment_dot_0.c
REM    res\C\samples\c_samples\struct_field_assignment_dot_1.c
REM    res\C\samples\c_samples\struct_field_assignment_ptr_0.c
REM    res\C\samples\c_samples\struct_field_assignment_ptr_1.c
REM    res\C\samples\c_samples\struct_field_getAddress_0.c
REM    res\C\samples\c_samples\struct_initializer_0.c
REM    res\C\samples\c_samples\struct_initializer_1.c
REM    res\C\samples\c_samples\struct_pointer_0.c
REM    res\C\samples\c_samples\struct_recursive_0.c
REM
REM
REM
REM res\C\samples\c_samples\swap.c
REM
REM
REM
REM
REM
REM
REM
REM
REM