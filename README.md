# kedi-lang

## Language features

- Language:
  - Dynamically typed
- Verification:
  - Functions specify arguments and return invariants similar to contracts.
  - All contracts are property-checked at build time.
- Targets:
  - Interpreter
  - JavaScript

## Implementation status

- [x] functions
- [x] i32
- [x] variables
- [x] conditionals & loops
- [x] online playground
- [ ] other stack types
- [ ] structs 
- [ ] arrays
- [ ] validation w/property-based testing
- [ ] validation w/fuzzing
- [ ] standard library
- [ ] io

### Components

* Phases:
  * **parser**: Source code to `parsed`
  * **renamer**: `parsed` to `plain`
    * Desugars some syntactic sugar.
    * Top-level values explicitly declare the non-local values they depend on.
  * **binder**: `plain` to `bound`
    * Resolves all references to their definitions.
  * **simplifier**: `plan` to `simple`
    * Simple is an untyped TAC-like language.
  * **codegen**: `simple` to `fragment`
    * Takes simple and generates an output.
  * **interpret**: `simple` to `Value`
    * Takes simple and interprets it. Useful for testing.

## Hacking

### Useful commands

<details>

```bash
# Watch the output of a phase
cargo watch -x 'run compile ./compiler/example/id.kedi --out - --out-parsed -' --clear

# Run the tests
cargo xtask test

# Build the compiler-web project and put it to appropriate location on `website` project
cargo xtask build-compiler-web-artifacts

# Given a wasm file, optimise with wasm-opt to compare
wasm2wat out.wasm > out.wat && wasm-opt out.wasm -o opt.wasm -O && wasm2wat opt.wasm > opt.wat
```

</details>