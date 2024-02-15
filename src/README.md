# Intermediate representation

## Design

The IR for the runtime is defined in the `expr` module. It is a simple, untyped, first-order language
in [ANF](https://en.wikipedia.org/wiki/A-normal_form) and [SSA](https://en.wikipedia.org/wiki/Static_single-assignment_form).
The language only contains top-level functions. It is assumed to be derived from a high-order functional
language by [lambda lifting](https://en.wikipedia.org/wiki/Lambda_lifting).

All function calls are to known top-level functions. Closures are essentially represented by partial application
(`papp`) objects. Calls to unknown at compile-time closures are done by the special `apply` procedure.
The design is greatly inspired by [GRIN](https://grin-compiler.github.io/).

## Syntax

The syntax has been chosen for the simplicity of implementation. It is close to Rust's syntax, as you see in
`let` and `match` expressions, and `fn` declarations. Unlike Rust, it has a Lisp-like application form
`(f x1 x2 ..)` and all binary operations are in this prefix form.

Parsing is done in two steps, with an initial lexer pass, defined in the `lexer` module, and a [recursive descend](https://en.wikipedia.org/wiki/Recursive_descent_parser)
parser with no backtracking, defined in the `parser` module.
