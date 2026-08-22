int main(void) {

    // VariableDeclaration(label:"userdef_var.0", data_type:"int")
    // Copy(src:Constant(1), dst:Variable(userdef_var.0))
    int a = 1;

    // VariableDeclaration(label:"userdef_var.2", data_type:"int") // variable for a_ptr
    // GetAddress(src:Variable(userdef_var.0), dst:Variable(userdef_var.2))
    int* a_ptr = &a;

    // Store(src:Constant(2), dst_ptr:Variable(userdef_var.1))
    *a_ptr = 122;

    return a;
}