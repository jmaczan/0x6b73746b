<p align="center"><img width="150" src="0x6b73746b.png" alt="0x6b73746b"></p>

# 0x6b73746b

Partial tree-walk interpreter based on Robert Nystrom's "Crafting Interpreters" book https://craftinginterpreters.com/ and Lox Programming Language

## Docs
Directory `lang/src` contains source code of the interpreter. In order to run the interpreter, you need `cargo` installed. Obtain a copy of this repository by i.e. cloning it, go to `lang` folder and in a terminal type `cargo run`. If you want to have an executable file to distribute it, run `cargo build --release`. The output file `0x6b73746b` is present in `lang/target/release/` directory. To run it, invoke it in a terminal `./0x6b73746b`.

The interpreter is written in Rust. It doesn't use any external crates. From the language's standard library, it uses `fs` for file system operations, `io` for handling standard input and output, `path` for cross-plaform path manipulation, `collections` for `HashMap` data structure and `env` to handle program arguments.

This project is not finished. What has been already implemented is:
1. Lexical analysis
2. Abstract Syntax Tree
3. Visitor pattern
4. AST Prettyprint
5. REPL
6. Recursive Descent Parser with error handling - in progress

## Notes

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

types of memory management (a spectrum of it):
1. **reference counting** - it's when a lang keeps track on amount of references and pointers to each variable 
2. **garbage collection** - it's when program frees those parts of memory to which there are no references anymore; prevents **dangling pointers** - it's when you empty a memory but there are still some pointers to this memory and you try to get access to this memory (dereference a pointer) using this pointer; prevents **double-free** - trying to empty a memory that is already emptied - and some **memory leaks**

types of operators:
1. **infix** - between two operands, i.e. `a + b`
2. **prefix** - before an operand, i.e. `!a`
3. **postfix** - after an operand, i.e. `a!`

**short-circuit evaluation** - when compiler omits evaluation of right side of `and` - `a and b` - when `a` is false or right side of `or` - `a or b` - when `a` is true

**declaring a function** is when you assign a type to a function's name

**defining a function** is when you add a body with code to a function

**parameter of a function** - a formal parameter; variable declared as a parameter of function, i.e. `a` and `b` in `func(a, b) { do_something(); }`

**argument of a function** - an actual parameter; value of a parameter which you use a inside function's body, i.e. value of `a`, like `5` or `"word"`

**class vs prototype** - in classes we have inheritance; an instance refers to a class for a method; in prototypes there are only instances (objects) and their relation is that one can **delegate** to another to invoke a method

**maximal munch** - a principle saying that when you decide about how to parse a lexeme, you should pick the one that matches as many characters as possible; it's the best match in other words, where 'best' is when more characters fit to the template than to others

**postorder traversal** - it's kind of traversing a graph (i.e. tree); depth-first; do recursively traversal to the left subtree, then recursively to the right subtree; https://medium.com/data-structure-and-algorithms/binary-tree-post-order-traversal-9e7174b87cda

in formal languages, which programming languages like this one belongs to, there are two (?) types of symbols. 
1. **non-terminal symbols** - like variable; it can be replaced with some other change; a concept of digit is non-terminal, because it might take different terminal values (like 0, 1, 2, etc)
2. **terminal symbols** - a defined value of variable, i.e. '0'; it can't be changed in something else, so that's why it's terminal

**chomsky hierarchy** - hierarchy of formal grammars; it consists of: 
1. **type-0** - "**recursively enumerable**"; turing machine; non-empty string of terminals and/or non-terminals produces possibly empty string of terminals and/or non-terminals
2. **type-1** - "**context-sensitive**"; non-deterministic turing machine; 
3. **type-2** - "**context-free**"; non-deterministic pushdown automation; non-terminal produces string of maybe empty terminals and/or non-terminals
4. **type-3** - "**regular**"; finite state automaton; non-terminal produces terminal and non-terminal produces terminal with non-terminal

the language's grammar consists of
1. **expression**, which may be literal, unary, binary or grouping
2. **literal**, which may be any number, string, true, false or nil
3. **grouping**, which is an expression inside parenthesis 
4. **unary**, which consists of ! or - followed by an expression
5. **binary**, which has infix operator, surrounded by expressions
6. **operator**, which might be one of allowed operators for math and comparing

**visitor pattern** is one of possible ways to traverse the AST. For each type of expression (binary, literal, unary, grouping, etc.) we implement the same interface which has a visitor's method, which takes a visitor as a param, i.e. `accept(visitor)` and invokes a proper method from Visitor interface, like visit_binary_expr when visiting binary expression etc. 

**pretty-print** utilizes visitor pattern and each node (?) with `accept` method invokes a proper visitor's method which produces a `string`.

**recursive descent parser** starts parsing the code from from highest level rules from lang's grammar (in case of this lang it's expression/equality) and recursively applies them until on bottom (in this case primary, like number, string, boolean, nil or expression in parenthesis)

## Author
Notes is bunch of my notes from working through Robert Nystrom's "Crafting Interpreters" book and a result of searching through web to learn about programming languages and compilers and trying to explain those things in a written form

Design of 0x6b73746b lang is derived from Lox lang and modified

Implementation of 0x6b73746b lang is based on C and Java implementations of Lox lang and written from a scratch in Rust

© Copyright [Jędrzej Paweł Maczan](https://maczan.pl/). Made in [Poland](https://en.wikipedia.org/wiki/Poland), 2022
