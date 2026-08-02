        .586p
        .model  flat,c
        .data           ;  initialized data
        .data?          ;uninitialized data
        .stack  4096
        .code
main    proc    near
        mov     eax,5
        add     eax,6
        xor     eax,eax
        ret
main    endp
        end