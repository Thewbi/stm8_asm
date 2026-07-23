use std::fs;

pub fn provide_sourcode_input() -> String {

    // NUMERIC PLUS NUMERIC
    // let str = "2 + 2";

    // NUMERIC PLUS NUMERIC SEMICOLON
    // let str = "2 + 2;";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET RETURN NUMERIC SEMICOLON
    // let str = "if (1 < 2) return 2;";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON
    // let str = "if (1 < 2) return 2 + 2;";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "if (1 < 2) { }";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "if (1 < 2) { return; }";

    // let str = "if (1 < 2) { return 0; }";

    // let str = "if (1 < 2) { return 2 + 2; }";

    // let str = "if (1 < 2) { return 2 + 2; } if (1 < 2) { return 2 + 2; }";

    // let str = "{ if (1 < 2) { return 2 + 2; } if (1 < 2) { return 2 + 2; } }";

    // https://github.com/nlsandler/writing-a-c-compiler-tests/blob/main/tests/chapter_1/valid/return_0.c
    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { return 2; }";
    // let str = "int main() { return void; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET INT IDENTIFIER SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { int abc; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET INT IDENTIFIER SEMICOLON RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { int abc; return 2; }";

    // let str = "int main() { if (1 < 2) return 2; }";
    // let str = "int main() { if (1 < 2) { return 2; } }";
    
    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { if (1 < 2) { return 2; } return 0; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET ELSE OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET CLOSING_CURLY_BRACKET
    // let str = "int main() { if (1 < 2) { return 2; } else { return 3; } }";
    // let str = "int main() { if (1 < 2) { return; } else { return; } }";
    // let str = "int main() { if (1 < 2) {} else {} }";
 
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET VOID CLOSING_BRACKET OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET ELSE OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET CLOSING_CURLY_BRACKET
    // let str = "void main() { if (void) {} else {} }";
    // let str = "void main() { if (1) {} else {} }";
    // let str = "void main() { if (1) { return; } else {} }";
    // let str = "void main() { if (1) { return; } else {return 0; } }";
    // let str = "void main() { if (1 < 2) { return; } else {return 0; } }";
    // let str = "void main() { if (1 < 2) { return 2 + 2; } else {return 0; } }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET EXPRESSION_STOP CLOSING_BRACKET OPENING_CURLY_BRACKET STATEMENT_STOP CLOSING_CURLY_BRACKET ELSE OPENING_CURLY_BRACKET STATEMENT_STOP CLOSING_CURLY_BRACKET CLOSING_CURLY_BRACKET
    // let str = "int main() { if (EXPRESSION_STOP) { STATEMENT_STOP } else { STATEMENT_STOP } }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET EXPRESSION_STOP CLOSING_BRACKET STATEMENT_STOP ELSE STATEMENT_STOP CLOSING_CURLY_BRACKET
    // let str = "int main() { if (EXPRESSION_STOP) STATEMENT_STOP else STATEMENT_STOP }";

    // IF OPENING_BRACKET EXPRESSION_STOP CLOSING_BRACKET STATEMENT_STOP ELSE STATEMENT_STOP
    // let str = "if ( EXPRESSION_STOP ) STATEMENT_STOP else STATEMENT_STOP";
    // let str = "if ( void ) void else void";
    // let str = "if ( void ) return; else return;";
    // let str = "if ( void ) return; else if ( void ) return;";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON CLOSING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { if (1 < 2) { return 2 + 2; } return 0; }";
    // let str = "int main() { int a; if (1 < 2) { return 2 + 2; } return 0; }";

    // let str = "int main() { int a; }";
    // let str = "int main() { int a = 0; }";
    // let str = "int main() { int a = 2; a *= 100; }";

    // let str = "int main() { int a = 1 + 1; }";
    // let str = "int main() { int a = 1 - 1; }";
    // let str = "int main() { int a = 1 * 1; }";
    // let str = "int main() { int a = 1 / 1; }";
    // let str = "int main() { int a = 1 % 1; }";

    // let str = "int main() { int a = 1; int b = 2; }";
    // let str = "int main() { int a = 1; int b = 2; a > b ? 1 : 2 ; }";
    // let str = "int main() { int a = 1; int b = 2; (a > b) ? 1 : 2 ; }"; // INVALID BUT SHOULD BE ALLOWED
    // let str = "int main() { int a = 1; int b = 2; a > b ? a++ : b++ ; }";
    // let str = "int main() { int a = 1; int b = 2; a > b ? a = 0 : b = 0; }"; // NOT WORKING YET

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET INT IDENTIFIER EQUALS_SIGN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { int a = 0; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET INT IDENTIFIER IDENTIFIER EQUALS_SIGN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { int a c = 0; }"; // This is accepted for some reason!!!!!
    // let str = "int main() { mullmull a c = 0; }"; // INVALID!

    // let str = "int main() { int a = 0; if (1 < 2) { return 2 + 2; } return 0; }";
    // let str = "int main() { int a = 0; if (1 < 2) { return 2 % 2; } return 0; }";
    // let str = "int main() { int a = 0; if (1 < 2) { return 2 % 2; } else { return 2 + 2; } return 0; }";

    // let str = "int main() { int a; int b; c = a && b; }";
    // let str = "int main() { int a; int b; c = a || b; }";

    // let str = "int main() { int a; int b; c = a | b; }";
    // let str = "int main() { int a; int b; c = a ^ b; }";

    // let str = "int main() { int a; a = a << 1; }";
    // let str = "int main() { int a; a = a >> 1; }";

    // let str = "int main() { if (1 == 2) { return; } }";
    // let str = "int main() { if (1 != 2) { return; } }";

    // let str = "int main() { if (1 < 2) { return; } }";
    // let str = "int main() { if (1 <= 2) { return; } }";
    // let str = "int main() { if (1 > 2) { return; } }";
    // let str = "int main() { if (1 >= 2) { return; } }";

    // let str = "int main() { const int abc = 3; }";
    // let str = "int main() { ; }";

    // let str = "int main() { int a; int b; a = (int) b; }";

    // let str = "int main() { while (a < b) { void; } }";
    // let str = "int main() { while (1) { void; } }";
    // let str = "int main() { while (1) { if (a < b) return 0; } }";

    // let str = "int main() { do { if (a < b) return 0; } while (1); }";

    // let str = "int main() { for ( i = 0; a < 10; i++ ) { return; } }";

    // let str = "int main() { int a = (float) b; }";
    // let str = "int main() { int a = (float) * b; }";

    // let str = "int main() { switch (data) { case const_1: break; case const_2: break; default: break; } }";

    // let str = "int main() { switch (data) { case const_1: if (1 < 2) { return; } break; case const_2: { int a = (float) b; int a = (float) b; } break; default: break; } }";

    // let str = "int main() { }";

    // let str = "enum days_enum { AA, BB };";
    // let str = "enum days_enum { MON, TUE, WED, THU, FRI, SAT, SUN };";

    // struct Person {
    //     char name[50];
    //     int alter;
    //     float gehalt;
    // };
    // let str = "struct Person { int alter; float gehalt; };";
    // let str = "struct Person { int data[50]; };";
    // let str = "struct Person { char name[50]; int alter; float gehalt; };";

    // let str = "int zahlen[5];";
    // let str = "int zahlen[5]; int main() { zahlen[0] = 15; }";

    // let str = "int main() { int alter = 25; int *zeiger = &alter; }";

    // let str = "int main() { data_struct.field = 4; }";

    // let str = "int main() { data_struct->field = 4; }";

    // let str = "union Data { int i; float f; char str[20]; }; int main() { union Data data; data.i = 10; data.f = 220; }";

    // let str = "int main(int x, int y) { return x + y; }";

    // let str = "\"aaa\"";
    // let str = "int main() { char *message = \"aaa\"; }";
    // let str = "int main() { char *message = \"This is a string literal.\"; }";
    // let str = "int main() { char *message = \"This is a string literal.\"; }";
    // let str = "int main() { printf(\"This is a string literal: %d.\", 199); }";

    // let str = "int main(int argc, char **argv) { int (*say)(const char *); }";
    // let str = "int main(int argc, char **argv) { int (*say)(const char *); say = puts; }";
    // let str = "int main(int argc, char **argv) { int (*say)(const char *); say = puts; say(\"hello world\"); }";
    // let str = "int main(int argc, char **argv) { int (*say)(const char *); say = puts; say(\"hello world\"); return 0; }";

    // INT IDENTIFIER SEMICOLON
    // let str = "int abc;";

    // let str = "float abc;";
    // let str = "float abc = 1.0;";

    // VOID VOID VOID VOID VOID
    // let str = "void void void void void";

    // INT PLUS VOID
    // let str = "int + void";

    // let str = "int main() { celsius = 5; }";
    // let str = "int main() { celsius = 5.0f; }";

    // STRUCT IDENTIFIER IDENTIFIER EQUALS_SIGN OPENING_CURLY_BRACKET VOID COMMA VOID CLOSING_CURLY_BRACKET SEMICOLON
    // let str = "struct point p1 = { void, void };";

    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IDENTIFIER DOT IDENTIFIER EQUALS_SIGN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "void main () { p1.x = 2; }";
    // let str = "void main () { struct point p1 = { 1, 2 }; p1.x = 2; }";
    // let str = "int main () { struct point p1 = { 1, 2 }; p1.x = 2; return p1.x; }";
    // let str = "typedef struct point point_t; int main () { struct point p1 = { 1, 2 }; p1.x = 2; return p1.x; }";
    // let str = "struct point { int x; int y; }; typedef struct point point_t; int main () { struct point p1 = { 1, 2 }; p1.x = 2; point_t pp; return p1.x; }";

    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IDENTIFIER IDENTIFIER SEMICOLON CLOSING_CURLY_BRACKET
    //let str = "void main () { point_t pp; }";

    // let str = "typedef struct point point_t; void main () { point_t pp; }";

    // let str = "typedef int int32_t; void main () { int32_t pp; }";

    // let str = "typedef char* STRING; void main () { STRING pp; }";

    // let str = "void main () { int *p; }";
    // let str = "void main () { struct tnode *p; }";

    // let str = "int myNumbers[] = { 25, 50, 75, 100 };";

    //
    // Kernighan & Ritchie
    //

    // let str: String = fs::read_to_string("res/C/samples/kernighan_ritchie/page_9.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/kernighan_ritchie/page_10.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/kernighan_ritchie/page_10_scratchpad.c").expect("file cannot be read!");

    //
    // Nora Sandler
    //

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/page_26.c").expect("file cannot be read!");

    //
    // C Samples
    //

    // let str: String = fs::read_to_string("res/C/samples/c_samples/hex_numeric_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/expression_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/expression_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/expression_2.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/if_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/if_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/if_else_if_0.c").expect("file cannot be read!");
    
    // let str: String = fs::read_to_string("res/C/samples/c_samples/function_call_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/for_loop_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/for_loop_1.c").expect("file cannot be read!");

    let str: String = fs::read_to_string("res/C/samples/c_samples/do_loop_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/main_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_3.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/switch_0.c").expect("file cannot be read!");

    str.to_string()
}