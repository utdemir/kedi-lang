# kedi-lang

## Features

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
  * **simplifier**: `plan` to `simple`
    * Simple is an untyped TAC-like language.
  * **codegen**: `simple` to `fragment`
    * Fragment is a set of functions with bodies compiled to WASM.
  * **linker**: `fragment` to `wasm`
    * Links the fragments together and emits the final wasm module.
* Frequently used utilities:
  * **runner**: Calls the phases one after another to produce the final wasm module from source code.
  * **error**: Sum type that aggregates all compiler errors, and a pretty printer.
  * **util::loc**: Data types to track source locations.
  * **util::pp**: Utilities to pretty-print intermediate representations.

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