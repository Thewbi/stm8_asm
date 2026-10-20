// Specification: N1570, Committee Draft - April 12, 2011, ISO/IEC 9899:201x

#include <stdarg.h>

// page 302
int remove(const char *filename);

// page 324
// restrict keyword on page 58 along with all other keywords
int printf(const char * /*restrict*/ format, ...);
