<p align="center"><img width="150" src="0x6b73746b.png" alt="0x6b73746b"></p>

# The 0x6b73746b Programming Language

Based on Robert Nystrom's "Crafting Interpreters" book https://craftinginterpreters.com/ and the Lox Language

## Log

This README is also a log of how the language was built and what I learn during this. It serves me as notes to memorize new things better and be able to get back to them easier

**self-hosting** - it's when a compiler of language X is written in language X

**bootstrapping** - it's when you use a language Y to compile a compiler of language X, which now can compile compilers for language X, so a former compiler of language Y is not needed anymore

language is too long word for this log, I replace it with lang since now

compilation is a process of transforming code from complex form to basic structure

steps of programming lang compilation:
1. **scanning** (lexing, lexical analysis) - take text, output text divided on meaningful tokens like "function", "(", "'A'", "=" and so on; we get syntax as an output here
2. **parsing** - organize tokens into a tree (AST; abstract syntax tree; parse tree), so it becomes a binary (?) tree with top operation as a root node and two (maybe more, not sure yet) children nodes; it is a structure of meaning of tokens; **parser** discovers **syntax errors**
3. **static analysis** - gives a meaning to the syntax; 
   1. **binding (resolution)** - take an **identifier** (i.e. variable) and find its declaration and connect them; **scope** becomes important here, a declaration needs to be in a scope of a usage
   2. **type checking** for statically typed langs; discover **type errors**
   3. storing a result of static analysis:
      1. attach it to AST as additional **attributes**
      2. store them in a separate table with i.e. variable names as keys
      3. transform AST to something different
   
everything up to this point is **frontend** of compiler; it's about lang
then there's **middle end**, which starts from **intermediate representation** (IR) 
last is **backend**, which is about something deeper but don't know it yet

4. **intermediate representation**
   1. **control-flow graph** - representation of all possible paths in a program, shown on a graph
   2. **static single assignment** - when a value is reassigned to a variable, a variable becomes a version of the original one, so in intermediate representation there will never be more than a single declaration and a single assignment to a given variable; **use-define chain** is a data structure storing uses and definitions of a variable; for static single assignment, use-define chain stores a single element
   3. **continuation-passing style** - it's a code written in a way that a function X has a callback (function Y) as a parameter; function X doesn't return a value Z; it calls function Y with parameter Z instead; "No procedure is allowed to return to its caller - ever." ~Matt Might https://matt.might.net/articles/by-example-continuation-passing-style/
   4. **three-address code** - `a := b [operation c]`, i.e. `a := 4 + c`; more complex code might be split into a sequence of multiple three-address code instructions; three-address code notation is easier to parse to assembly than a regular code
5. **optimization** - it's possible to do optimization now, because a desired logic behind code is known; the open issue is how to implement this code as a low-level instructions (?)
   1. **constant propagation** - replacing all occurences of a constant with its value;
   2. **constant folding** - when expression is a constant, evaluate them and replace all of their occurences with an evaluated value
   3. **dead-code elimination** - removing a code that can't be reached by a program's execution and a code that is related to variables which are never used but are declared (dead variables)
   4. **register allocation** - optimization by assigning variables to registers; not sure yet how 
   5. **instruction selection** - ran before register allocation; I think it's choosing a machine instruction for a IR code
   6. **instruction scheduling** - optimization for organizing the order of instructions;

here the backend starts

**intrinsic function** - a function that is known for a compiler ('built-in') and is used directly from compiler, instead of being linked to some library

6. **code generation** - generate assembly code for a real processor or pseudo-assembly code for a virtual processor (VM?); real assembly code is executed by a physical chip; it means that the assembly code is dedicated to a specific processor architecture; a code generated for a virtual machine is called **bytecode** and this code is universal because the target architecture is a virtual one; if **code generation** produces **bytecode**, you can either write multiple compilers to transform bytecode to a platform specific code; bytecode is an intermediate representation here or create **virtual machine**

7. **virtual machine** - it's either:
   1. (lang || process) vm - a program written in X lang that emulates a hypothetical processor and might be ran on any platform which has X compiler installed
   2. (system) vm - emulator for a full hardware and operating system
8. **runtime** - if code generation produces a machine code, operating system loads an executable; if code generation produces a bytecode, start virtual machine and load a program; things like garbage collector are in runtime; compiled langs might have copy of runtime inside of a compiled executable; langs with vm have runtime inside vm

types of compilers:
1. **single-pass compilers** - do all steps in a single pass, no going back with anything
2. **tree-walk interpreters** - slow; program runs by traversing through an abstract syntax tree and evaluating nodes during a traversal
3. **transpilers** - transcompiler; this is i.e. TypeScript I think; write frontend (scanner and parser) of X lang and compile it to a frontend of another Y lang; when X and Y are different, you might do more additional steps in between and generate Y code in code generation step
4. **just-in-time compilation** - compile to bytecode to machine code when a program is loaded, so a compiler known to which architecture it should compile the code; examples: HotSpot JVM, JS (V8 engine), Microsoft Common Language Runtime; it has HotSpot in name because it has an optimization technique of finding important places in code which affect performance

**compiling**: translate lang X to lang Y; transpiling is a compling as well
**compiler**: translate code but not execute
**interpreter**: execute code from source

something might be both compiler and interpreter, i.e. JS (V8)

## Author
Log is a bunch of my notes from Robert Nystrom's "Crafting Interpreters" book and a result of searching through web about compilers and trying to explain those things in a written form

© Copyright [Jędrzej Paweł Maczan](https://maczan.pl/). Made in [Poland](https://en.wikipedia.org/wiki/Poland), 2022
