# evscript

This is a re-write of [evscript](https://github.com/eievui5/evscript)

My main goals are:
1. ✓ ~~Feature-parity with the C++ version of evscript~~
2. ✓ ~~Expressions~~
3. ✓ ~~Scoped variables (remove `drop`)~~
4. Structures
5. ✓ ~~Bytecode return values~~
7. Codegen Optimizations

I am rewriting evscript to remove its dependency on Bison and Flex, and to make it easier to install, using `cargo`.
I also want to make the entire codebase much cleaner and easier to understand, including the lexer, parser, and compiler.
