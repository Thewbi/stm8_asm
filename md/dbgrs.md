# dbgrs

dbgrs is a git repository for a windows debugger (COFF + PE) which is based on a sequence of blogposts by Tim Misiak.

https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-1/ - Attaching to a process
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-2/ - Register State and Stepping
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-3/ - Reading Memory
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-4/ - Exports and private symbols
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-5/ - Breakpoints
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-6/ - Stacks
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-7/ - Disassembly
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-8/ - Source and Symbols

The blog posts are save to doc/dbgrs

## Working with NASM and dbgrs

dbgrs.exe C:\Users\lapto\dev\nasm\hello_world\main.exe

The debugger will output the current address of rip: [41CC] 0x00007ffc8110ad40
This address can be used to display the next bytes that are executed:
db 0x00007ffc8110ad40
To dissassemble, use: u @rip

This works to set a breakpoint in main:
Go to the internet page: https://life45.github.io/pdb-view/
Upload the .pdb file to that page.
Delete the filter text so that all symbols are shown.
Search for the 'main' or 'start' symbol: it shows a RVA of 0x1010 for example
Now the question is how to make the relative RVA absolute so that a breakpoint
can be set to it? The answer is: The debugger will output the absolute module address
which the RVA can be added to as a relative offset to arrive at the absolute address
of the symbol!

```
Command line was: 'C:\Users\lapto\dev\masm\helloworld\main.exe'
LoadDll: 7FF75A810000   main.exe
[6748] 0x00007ffa1d2acac0
```

The second line in the output is: LoadDll: 7FF75A810000   main.exe
The hex number is the absolute module load address: 7FF75A810000

Now set a breakpoint:
bp <module load address>+<RVA>
bp 0x7FF75A810000+0x1010

Start the debugger: It will output:
bp 0x00007ffc8110ad40+0x1010

Next, run the debugger until the breakpoint is hit: g g g g g g g ....

Eventually, the debugger will output:

```
> g
Breakpoint 0 hit
[6748] main.exe!main
```

Now disassemble the code at the instruction pointer:

u @rip

The disassembled code should be the code in your source .asm file!

use the k command to output the call stack which should show main() as main.exe!main

```
> k
#   RSP              Call Site
00 0x000000307618F878 main.exe!main
[6748] main.exe!main
```

Now single step with t and output all registers with r


commands:
t                - step into
g                - go

bp <expr>        - set breakpoint
bl               - list breakpoints
bc <expr>        - clear breakpoints

r <expr>         - display specific registers
r                - display all registers

k                - stack walk
db <expr>        - display bytes
? <expr>         - evaluate
ln <expr>        - list nearest

u <expr>         - unassemble - example: u @rip
u                - unassemble continue

lsa <expr>       - list source
.srcpath <expr>  - set source path

q                - quit

https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-1/ - Attaching to a process
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-2/ - Register State and Stepping
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-3/ - Reading Memory
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-4/ - Exports and private symbols
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-5/ - Breakpoints
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-6/ - Stacks
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-7/ - Disassembly
https://www.timdbg.com/posts/writing-a-debugger-from-scratch-part-8/ - Source and Symbols

