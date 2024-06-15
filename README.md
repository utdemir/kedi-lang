# kedi-lang

### Components

* **Frontend**: Responsible for parsing the source code to `simple`
  * This runs on every compilation locally
  * Components:
    * **parser**: Source code to `parsed`
    * **renamer**: `parsed` to `plain`
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

## Roadmap

- [ ] Functions and int variables prototype (parser to wasm)
- [ ] Conditionals
- [ ] Loops

## Hacking

### Useful commands

Watch the output of a phase

<details>

```bash
cargo watch -x 'run compile ./compiler/example/id.kedi --out - --out-parsed -' --clear
```

</details>