use std::fs;

use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;

use std::io::BufWriter;
use std::io::Write;

// first element: text data
// second element: filename which is the source of the text data
pub fn provide_sourcode_input() -> ( String, String ) {

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

    // let str = "int numbers[] = { 25, 50, 75, 100 };";

    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IDENTIFIER OPENING_ANGULAR_BRACKET NUMERIC CLOSING_ANGULAR_BRACKET EQUALS_SIGN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "void main () { numbers[0] = 5; }";

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
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/listing_1_1_page_4.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/unary_0.c").expect("file cannot be read!"); // return -8; Result: "-8" (0xfffffff8)
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/unary_1.c").expect("file cannot be read!"); // return ~8; Result: "-9" (0xfffffff7)
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/unary_2.c").expect("file cannot be read!"); // return ~(-8); Result: "7" (0x7)
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/unary_3.c").expect("file cannot be read!"); // return -(~(-8)); Result: "-7" (0xfffffff9)
    // let filename = "res/C/samples/nora_sandler/unary_4.c";
    let filename = "res/C/samples/nora_sandler/unary_5.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_0.c").expect("file cannot be read!"); // return 1 + 2; Result: 3
    // let filename = "res/C/samples/nora_sandler/binary_0.c";
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_1.c").expect("file cannot be read!"); // return 1 + (2 * 3); Result: 7
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_2.c").expect("file cannot be read!"); // return (1 + 2) * 3; Result: 9
    // TODO: somehow no negative result is computed
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_3.c").expect("file cannot be read!"); // return (1 - 2) * 3; Result: -3
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_4.c").expect("file cannot be read!"); // return 5 / 2; Result: 2
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_5.c").expect("file cannot be read!"); // return 20 % 17; Result: 3
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_6.c").expect("file cannot be read!"); // return 10 - 3; Result: "7" (0x7)
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_7.c").expect("file cannot be read!"); // return 10 * 3; Result: "30" (0x1e)
    // let filename = "res/C/samples/nora_sandler/binary_8.c";
    // let filename = "res/C/samples/nora_sandler/binary_9.c";
    // let filename = "res/C/samples/nora_sandler/binary_10.c";

    // // &	bitwise AND
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_bitwise_0.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/binary_bitwise_0.c";

    // // |	bitwise inclusive OR
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_bitwise_1.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/binary_bitwise_1.c";

    // // ^	bitwise XOR (exclusive OR)
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_bitwise_2.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/binary_bitwise_2.c";

    // // <<	left shift
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_bitwise_3.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/binary_bitwise_3.c";

    // // >>	right shift
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_bitwise_4.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/binary_bitwise_4.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_3.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_4.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_5.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_6.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_7.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_8.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_9.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_10.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_11.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/declaration_12.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/declaration_13.c";
    // let filename = "res/C/samples/nora_sandler/declaration_14.c";
    // let filename = "res/C/samples/nora_sandler/declaration_15.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/assignment_0.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/assignment_0.c";
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/assignment_1.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/assignment_1.c";
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/assignment_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/assignment_3.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/assignment_4.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/assignment_5.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_assignment_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_assignment_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_assignment_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_assignment_3.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/binary_assignment_4.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/prefix_postfix_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/prefix_postfix_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/prefix_postfix_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/prefix_postfix_3.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/double_variable_decl_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/initialization_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/initialization_1.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/block_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/block_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/block_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/block_3.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_0.c").expect("file cannot be read!"); // Result: 123
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_1.c").expect("file cannot be read!"); // Result: 0
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_2.c").expect("file cannot be read!"); // Result: 0
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_3.c").expect("file cannot be read!"); // Result: 0
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_4.c").expect("file cannot be read!"); // Result: 0
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_5.c").expect("file cannot be read!"); // Result: 0
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_6.c").expect("file cannot be read!"); // Result: 0
    // let filename = "res/C/samples/nora_sandler/if_0.c";
    // let filename = "res/C/samples/nora_sandler/if_1.c";
    // let filename = "res/C/samples/nora_sandler/if_2.c";
    // let filename = "res/C/samples/nora_sandler/if_3.c";
    // let filename = "res/C/samples/nora_sandler/if_4.c";
    // let filename = "res/C/samples/nora_sandler/if_5.c";
    // let filename = "res/C/samples/nora_sandler/if_6.c";
    // let filename = "res/C/samples/nora_sandler/if_7.c";

    // let filename = "res/C/samples/nora_sandler/scopes_0.c";
    // let filename = "res/C/samples/nora_sandler/scopes_1.c";
    // let filename = "res/C/samples/nora_sandler/scopes_2.c";
    // let filename = "res/C/samples/nora_sandler/scopes_3.c";

    // let filename = "res/C/samples/nora_sandler/scopes_illegal_0.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_else_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_else_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_else_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_else_3.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_else_4.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/if_else_5.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/while_0.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/while_0.c";
    // let filename = "res/C/samples/nora_sandler/while_1.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/for_0.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/for_0.c";
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/for_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/for_2.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/for_2.c";
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/for_3.c").expect("file cannot be read!");

    // let filename = "res/C/samples/nora_sandler/return_0.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/do_while_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/conditional_0.c").expect("file cannot be read!");

    // TODO: check if a fall-through is converted to a valid AST!

    // TODO: The discriminator "test_val" is not output to the AST! check if information is just not printed or if information is missing!
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/switch_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/switch_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/switch_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/switch_3.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_declaration_0.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/function_declaration_0.c";
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_declaration_1.c").expect("file cannot be read!");
    // The file function_declaration_2.c contains an syntactically invalid application according to the C-grammar used!
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_declaration_2.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/function_declaration_2.c";
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_declaration_3.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/function_declaration_3.c";
    // let filename = "res/C/samples/nora_sandler/function_declaration_4.c";
    // let filename = "res/C/samples/nora_sandler/function_declaration_5.c"; // two declarations both with bodies, is not allowed.
    // let filename = "res/C/samples/nora_sandler/function_declaration_6.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_prototype_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_prototype_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_prototype_2.c").expect("file cannot be read!");
    // TODO: contains struct
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_prototype_3.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_call_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/function_call_1.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/function_call_2.c";
    // let filename = "res/C/samples/nora_sandler/function_call_3.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/storage_class_static_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/storage_class_static_1.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/storage_class_extern_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/data_type_int_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/data_type_long_0.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/data_type_long_0.c";
    // let filename = "res/C/samples/nora_sandler/data_type_long_1.c";
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/data_type_signed_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/data_type_unsigned_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/data_type_unsigned_1.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/cast_0.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/cast_0.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/const_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/const_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/const_2.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/pointers_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/pointers_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/pointers_2.c").expect("file cannot be read!");
    // let filename = "res/C/samples/nora_sandler/pointers_3.c";
    // let filename = "res/C/samples/nora_sandler/pointers_4.c";
    // let filename = "res/C/samples/nora_sandler/pointers_5.c";

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/array_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/array_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/array_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/array_3.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/struct_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/struct_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/struct_2.c").expect("file cannot be read!");
    // todo: TACKY visitor fails
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/struct_3.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/struct_4.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/struct_5.c").expect("file cannot be read!");
    // todo: AST broken
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/struct_6.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/struct_linked_list_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/sizeof_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/sizeof_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/sizeof_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/sizeof_3.c").expect("file cannot be read!");

    //
    // C Samples (Base Constructs)
    //

    // let str: String = fs::read_to_string("res/C/samples/c_samples/hex_numeric_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/expression_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/expression_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/expression_2.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/if_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/if_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/if_else_if_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/function_call_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/function_call_3.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/for_loop_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/for_loop_1.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/do_loop_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/main_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_3.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/switch_0.c").expect("file cannot be read!");

    //
    // C Samples (larger units)
    //

    // let str: String = fs::read_to_string("res/C/samples/c_samples/swap.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/linked_list_without_malloc.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/scratchpad.c").expect("file cannot be read!");

    //
    // Preprocessor - step 1 - remove single-line comments
    //

    // 1. Create or overwrite the file
    let preprocessed_file = File::create("preprocessed.c").expect("Create file failed!");

    // 2. Wrap the file in a BufWriter
    let mut writer = BufWriter::new(preprocessed_file);

    // 3. Write data
    let f = match File::open(filename) {

        Ok(f) => {

            let mut line_num = 0;

            let file = BufReader::new(&f);
            for line in file.lines() {

                let curr_line = line.unwrap();

                // // DEBUG
                // println!("{}", l);

                // start adding linebreaks after the first line
                if line_num > 0 {
                    write!(writer, "{}", "\n");
                }

                // remove single-line comments by a space
                if curr_line.trim().starts_with("//") {
                    write!(writer, "{}", " ");
                } else {
                    write!(writer, "{}", curr_line);
                }

                line_num = line_num + 1;
            }
        }

        Err(e) => {
            // fallback in case of failure.
            // you could log the error, panic, or do anything else.
            panic!("{}", e);
        }

    };

    // 4. Explicitly flush the remaining data to disk
    writer.flush().expect("flush failed!");

    // output C-application translation unit into a fixed file called "preprocessed.c".
    // The C-compiler will read input from "preprocessed.c"
    let str: String = fs::read_to_string("preprocessed.c").expect("file cannot be read!");

    return ( str.to_string(), filename.to_string() )
}