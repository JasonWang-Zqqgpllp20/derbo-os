# About the DerBo OS
DerBo OS, an x86-64 based operating system, writing in Rust. Besides the basic functions of the operating system, a collaborative multitasking is implemented throughout asynchronous programming in Rust.
# The Compiler in DerBo OS
A compiler is also embeded in DerBo OS. Basic components are implemented such as lexer, parser, binder, lowerer, and evaluator. Most importantly, the compiler takes user-written code into trusted applications, the novel trick solves the drawback of collaborative multitasking.