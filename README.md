# 0x6b73746b
The 0x6b73746b Programming Language

Based on Robert Nystrom's https://craftinginterpreters.com/ and Lox Language

## Log

This README is also a log of how the language is built and what I learn during this. It serves me as a notes to memorize new things better and be able to get back to them easier

**self-hosting** - it's when a compiles of language X is written in language X

**bootstrapping** - it's when you use a language Y to compile a compiler of language X, which now can compile compilers for language X, so a former compiler of language Y is not needed anymore

language is too long word for this log, I replace it with lang since now

steps of programming language compilation:
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
then there's **middle end**, which is **intermediate representation** (IR) 
last is **backend**, which is about something deeper but don't know it yet
4. **intermediate representation** https://craftinginterpreters.com/a-map-of-the-territory.html#intermediate-representations TODO "There are a few well-established styles of IRs out there. Hit your search engine of choice and look for “control flow graph”, “static single-assignment”, “continuation-passing style”, and “three-address code”."