# kedi-lang

## Compiler architecture



* `simple`. 
  * It is an untyped TAC-like language.

### Codegen

* Takes simple and produces wasm using binaryen.

### Components

* **Frontend**: Responsible for parsing the source code to `simple`
  * This runs on every compilation locally
  * Components:
    * **parser**: Source code to `parsed`
    * **desugarer**: `parsed` to `plain`
      * Desugars some syntactic sugar.
      * Resolves the `import` statements.
      * Top-level values explicitly declare the non-local values they depend on.
    * **simplifier**: `plan` to `simple`
      * Simple is an untyped TAC-like language.
* **Backend**:
    * (later) **Analyzer**: Analyzes the `simple` code for errors and annotates found types.
    * **Codegen**: Responsible for generating the wasm output from `simple`
      * This can optionally be distributed and heavily cached.
    * (later) **Critic**: Takes the compiled wasm and property tests.