int main(void) {

    // VariableDeclaration(label:"userdef_var.0", data_type:"int")
    // Copy(src:Constant(1), dst:Variable(userdef_var.0))
    int a = 1;

    // VariableDeclaration(label:"userdef_var.1", data_type:"int")
    // Copy(src:Constant(2), dst:Variable(userdef_var.1))
    int b = 2;
    
    // VariableDeclaration(label:"userdef_var.2", data_type:"int") // variable for a_ptr
    // GetAddress(src:Variable(userdef_var.0), dst:Variable(userdef_var.2))
    int a_ptr = &a;

    // VariableDeclaration(label:"userdef_var.3", data_type:"int") // variable for b_ptr
    // GetAddress(src:Variable(userdef_var.1), dst:Variable(userdef_var.3))
    int b_ptr = &b;
    
    // dereference a_ptr into temp.
    // 
    // Emit:
    // VariableDeclaration(label:"userdef_var.4", data_type:"int") // variable for temp
    // Load("userdef_var.2", Variable("userdef_var.4")) // Load(a_ptr, temp)
    int temp = *a_ptr;

    // b_ptr first needs to go into a temp variable
    //
    // Emit:
    // VariableDeclaration(label:"userdef_var.5", data_type:"int")
    // Load(b_ptr, Variable("userdef_var.5")) // b_ptr ---> t_5
    // Store(Variable("userdef_var.5"), a_ptr) // t_5 ---> a_ptr
    //*a_ptr = *b_ptr;
    int x = *b_ptr;
    *a_ptr = x;

    // Store(Variable(temp), b_ptr) // temp ---> b_ptr
    *b_ptr = temp;

    return a;
}