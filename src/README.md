# Intermediate representation

## Design

The IR for the runtime is defined in the `expr` module. It is a simple, untyped, first-order language
in [ANF](https://en.wikipedia.org/wiki/A-normal_form) and [SSA](https://en.wikipedia.org/wiki/Static_single-assignment_form).
The language contains only top-level functions. It is assumed to be derived from a high-order functional
language by [lambda lifting](https://en.wikipedia.org/wiki/Lambda_lifting).

All function calls are to known top-level functions. Closures are represented by partial application
(`papp`) objects. Calls to closures unknown at compile-time are done by the special `apply` procedure.
The design is greatly inspired by [GRIN](https://grin-compiler.github.io/).

As of now, the language does not have data types apart from numbers and closures. Inductive data types
can be represented by lambda encodings, such as [Scott encoding](https://en.wikipedia.org/wiki/Mogensen%E2%80%93Scott_encoding).
No update method exists either, so we cannot mutate or implement updatable thunks. Match statements are
essentially switch statements, with no pattern allowed except for the default pattern. Eventually, other data types,
update statements and maybe match patterns, will be added.

## Syntax

The syntax was chosen for its simplicity of implementation. It is close to Rust's syntax, as seen in
`let` and `match` expressions, and `fn` declarations. Unlike Rust, it has a Lisp-like application form
`(f x1 x2 ..)` and all binary operations are in this prefix form.

Parsing is done in two steps, with an initial lexer pass, defined in the `lexer` module, and a [recursive descent](https://en.wikipedia.org/wiki/Recursive_descent_parser)
parser with no backtracking, defined in the `parser` module.

# Runtime

## Interpreter

The `interpreter` module defines an interpreter that works directly on the IR expressions. It is not intended
to be fast, but merely to show how expressions can be reduced.

The interpreter assumes each variable defines a unique register, and is thus represented by a `HashMap`. No register spilling is required.
However, since functions can reuse variable names (consider the case of a recursive call), a frame stack is defined, and a call will save
and restore stack frames. The stack has a limited and fixed size defined by `INIT_STACK_SIZE`

A heap is also defined, with a fixed size of `INIT_HEAP_SIZE`. The allocator is a simple bump allocator, much like a stack allocator.
It currently has no garbage collector or any other method of reclaiming dead objects. A simple [stop-and-copy](https://en.wikipedia.org/wiki/Cheney%27s_algorithm)
garbage-collector will eventually be implemented.

The initial design of the interpreter is recursive, thus also using Rust's own stack. An iterative version that runs on a single
Rust frame can be achieved by adding continuations to frames, which essentially work as return addresses. Tail-call
optimization can then be implemented.

## Compiler

A compiler to LLVM will eventually be implemented. The design choice of the IR makes programs almost straightforward to compile to LLVM IR code.
The most challenging aspect of the compiler is implementing the [runtime](https://en.wikipedia.org/wiki/Runtime_system) that will be run alongside the compiled program.
That is, the collection of services needed for the program to run, such as the allocator, garbage collector, thread scheduler, and so on.

# Other considerations

This project is mostly for educational purposes. It is designed to be comprehensive, implementing all components from scratch,
using few to no external libraries. The code structure is yet to be made clear, but it should be done in a way to make it
easier for people to read and collaborate. In particular, it should have a non-cyclical, tree-like, module dependency graph,
and all subdirectories should have a `README.md`, explaining the most important details of the module design, which files
should be read first and so on.
