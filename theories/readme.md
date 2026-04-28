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