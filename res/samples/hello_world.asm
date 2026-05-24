;Hello World

.code 
bl printHello 
swi 6 

printHello: 
	ldr r0,=hello
	swi 5
	bx lr

.data 
	 hello: .asciiz "Hello World"