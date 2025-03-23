# 00 - The big idea

**kedi-lang** is a conceptual programming language that explores automatic property-testing as a means to provide compile-time guarantees without the need for a complex type-checker. I have a hunch that with property testing and simpler static checks we can achieve a lot of the same guarantees while allowing greater flexibility and expressiveness.

I am envisioning a language where:

- Functions define input and output properties as ordinary boolean expressions.
- The compiler uses property testing & fuzzing to verify the properties.

I believe this will allow:

- For application developers to being able to define very precise "types" without the mental overhead of a complex type-system.
- For the IDE's to be able to provide feedback for things like termination, exception-safety, integer overflows, out-of-bounds array accesses, excessive memory usage that is otherwise hard to prove.
- For mechanical processes like LLM's to be able to use write-test-rewrite loops without worrying about test harnesses or side effects.
- For the compiler to make better optimisation decisions using the information coming from running the property tests.
- For the library developers to be able to use foreign interfaces with compile-time safety guarantees.

To make this work, the language has a few key features:

- To allow property testing effectful code, has an effect system that asks for every side-effect to also implement a pure "mock" version.
- To efficiently discover the input space, the compiler uses fuzzing techniques to generate test-cases.
- To mitigate the excessive cost of property testing, every function has a content-addressable hash that is used to cache the results tests.
- To reduce the overall costs, the compiler can integrate with a shared cache of property-test results that developers can share. 


## With more words

We do like expressive programming languages, and also we like safe programming languages. The trend towards safer languages is obvious, and the want for more expressive languages pushes towards more complex type-systems.

However, even withthe most advanced type-systems the properties we can prove right now is trivial. To push the bounds we very quickly move towards a lot harder to use tools like dependent types, linear types, theorem provers, and none are silver-bullets. The limitations are so ingrained that we think of words like "safe" or "does not crash" to be defined within those bounds.

As an examples, Rust is said to be memory safe and not crash when we don't use panicking functions (like `.unwrap()`). But see: 


```rust
// Out of bounds access
fn main() {
    let arr = [1, 2, 3];
    println!("{}", arr[foo()]);
}

fn foo() -> usize {
    return 1000;
}
```

```rust
// Division by zero
fn main() {
    print!("{}", 1 / i());
}

fn i() -> i32 {
    return 0;
}
```

```rust
// Non-termination
fn foo(i: i32) -> i32 {
    loop {}
}
```

```rust
// Stack overflows
fn foo() {
    println!("{}", [1; 10_000_000_000].len());
}

fn bar(i: i32) -> i32 {
    bar(i)
}
```
