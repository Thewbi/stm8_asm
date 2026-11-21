# Problem

An array access such as

```
buffer[1] = 123;
int val = buffer[1];
```

looks the same independent of which context it is located in.

There are at least two contexts that matter.

## First Context - local variable

The first context is where the array appears as a local variable in the current function:

```
// res\C\samples\c_samples\array_0.c

int main() {
    int arr[3];

    arr[0] = 1;
    arr[1] = 2;
    arr[2] = 3;

    if (arr[2] == 3) {
        return 0;
    }
    return 1;
}
```

When the array is a local variable, accessing the array in a combined expression is possible:

```
mov eax, dword ptr [rbp-408+rax*4]
```

because rbp is the base pointer of the local stack frame and the array lives within that stack frame.
The expression will resolve correctly. rax stored the index at which the array is accessed.

## Second Context - Array address passed as parameter (Pointer Parameter)

The second context is where the array is used in the body of a function where it has been
passed in via it's address, in other words as a pointer.

```
// res\C\samples\c_samples\strlen_0.c

int strlen(char *s)
{
    int i = 0;
    while (s[i] != 0) {
        i++;
    }
    return i;
}

int main() {
    int result = strlen("msg_const_val"); // 6D 73 67 5F 63 6F 6E 73 74 5F 76 61 6C
    return result;
}
```

or without a string literal:

```
// res\C\samples\c_samples\function_call_5.c

#define ARRAY_LENGTH 100

int array_sum(char *lpBuffer, int length) {

    int result = 0;
    int i = 0;
    for (i = 0; i < length; i++) {
        result = result + lpBuffer[i];
    }

    return result;
}

int main() {

    char buffer[ARRAY_LENGTH];

    int i = 0;
    for (i = 0; i < ARRAY_LENGTH; i++) {
        buffer[i] = i;
    }

    int result = array_sum(buffer, ARRAY_LENGTH);

    return result;
}
```



## Second Context - Array address passed as parameter (Pointer Parameter)

If the array is used via a pointer to it's location, a combined expression is not a solution which
is going to work. The problem is that Nora Sandler in her book decided to copy every value from
a register used in the calling convention to memory as a first thing after a function is entered.

This means in Nora Sandlers world, the pointer is copied from a register into memory and needs
to be retrieved from memory before being used.

The pointer itself is local to the stack frame and can be accessed relative to
rbp. The problem is that the pointer itself does not matter, what matters is the address stored
inside the pointer. This means first, the address-value stored inside has to be retrieved using
a rbp relative expression:

```
mov rbx, qword ptr [rbp+-408]
```
Then, once the underlying address to the beginning of the array is known, the array will be
accessed using a combined expression. The combined expression is not rbp-relative but array
address relative. The base address of the pointer is stored in rbx by the first instruction
above!

```
mov eax, dword ptr[rbx+rax*4]
```