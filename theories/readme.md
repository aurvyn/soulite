# Soulite Subset Formal Verification

This directory contains Rocq proofs that formally verify SimpleSoulite (a simple subset of Soulite) by compiling it to Iris [heap_lang](https://gitlab.mpi-sws.org/iris/iris) and proving a forward simulation between the small‑step semantics of SimpleSoulite and the weakest precondition semantics of HeapLang.

## Prerequisites

- Linux or WSL2 (macOS/Windows may work but are not tested)
- [Opam](https://opam.ocaml.org/) 2.0+
- Rocq 8.19+ and Iris 4.2+ (installed via opam)

## Setup

To run the Rocq files for formal verification, initialize the makefile:
```bash
coq_makefile -f _CoqProject -o makefile
```
This should generate the following three files:
- `.makefile.d`
- `makefile`
- `makefile.conf`

## Development

To compile all `.v` files:
```bash
make
```
For VsCode, install the VsRocq extension and set the `vsrocqtop` path to your opam‑installed `rocq` binary.

### Project structure

- `ast.v`, SimpleSoulite abstract syntax and types
- `notation.v`, custom syntax and notation for SimpleSoulite
- `semantics.v`, small‑step operational semantics of SimpleSoulite
- `compiler.v`, compiler from SimpleSoulite to heap_lang
- `correctness.v`, forward simulation proof

### Notable Syntactic Differences

Due to limitations of using Rocq's notation feature, SimpleSoulite's syntax has deviated from Soulite:

- Variable and function names: `var` -> `!"var"`
- Ternary condition: `if_true <- cond ; if_false` -> `if_true <- cond ;; if_false`
- Variable assignment: `var = val` -> `var ,= val`
- Newlines and tabs need to be typed out: `\n` and `\t`

### Challenges

I walked into this project thinking that defining the notation and semantics for this simple subset of Soulite should be less time-consuming than proving the forward simulation.
Although I wasn't wrong, writing and refining `ast.v`, `heaplang.v`, `notation.v`, and `semantics.v` took more time than I have expected.

Fighting with the notation syntax in Rocq was an interesting experience, but also goes to show its limitations.
I did not end up defining notations for most of the SimpleSoulite ast as the system is just too fragile; adding something is more likely to break another thing than actually working.

Defining the ast and semantics was just as difficult.
I only had a Soulite transpiler, so I did not deeply understand what Soulite programs are doing under the hood.
Going through and reasoning about the semantics of this small subset really made me think.

Compiling Soulite into heap_lang was the second-most challenging part.
SimpleSoulite is an imperative language, while heap_lang is functional.
There was many hoops that I had to jump through to get the right interpretation, and it involved a lot of digging through `lang.v` in the heap_lang repository.

`correctness.v` is the most difficult part of this project.
Due to time constraints, I could not finish it, and so some definitions were admitted.

- `closed_expr` ensures that the source SimpleSoulite expression has no free variables.
Without it, a variable `x` could appear without a declaration, and the compiler would produce `Var "x"`.
In heap_lang, `Var "x"` is a free variable, and `WP` does not assign it a value, causing a stuck program.

- `pure_expr` excludes mutable declarations and assignments because the current compiler translates them into heap_lang references and stores, but `sl_step` directly updates the environment.
A direct simulation between them would probably require a more sophisticated state relation that maps Soulite’s environment to HeapLang’s heap, which didn't seem doable under the time constraints.

Overall, working on this project has been an fruitful experience, and I have learned a lot about formal verification and using Rocq. I think the biggest take-away for me is realizing that while tools like Rocq are powerful, their limitation is that the verified program correctness is only as good as the specification itself. If the specification didn't address a certain edge case, then the verified program would still be prone to bugs.