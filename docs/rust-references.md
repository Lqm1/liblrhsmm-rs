# Primary Rust implementation references

- Package generation: https://doc.rust-lang.org/cargo/commands/cargo-new.html
- Owned vectors and slice storage: https://doc.rust-lang.org/std/vec/struct.Vec.html
- Read and Write interfaces: https://doc.rust-lang.org/std/io/trait.Read.html and https://doc.rust-lang.org/std/io/trait.Write.html
- Fallible operations: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
- Error types: https://doc.rust-lang.org/std/error/trait.Error.html
- Safe IEEE 754 bit conversion: https://doc.rust-lang.org/std/primitive.f32.html#method.to_bits and https://doc.rust-lang.org/std/primitive.f32.html#method.from_bits
- Platform support: https://doc.rust-lang.org/beta/rustc/platform-support.html
- Binary64 bit conversion: https://doc.rust-lang.org/std/primitive.f64.html#method.from_le_bytes
- Generic parameter defaults for precision support: https://doc.rust-lang.org/reference/items/generics.html
- Type aliases: https://doc.rust-lang.org/reference/items/type-aliases.html
- Low-level MessagePack operations: https://docs.rs/rmp/0.8.15/rmp/

Context7 and the Mintlify documentation index were consulted before using rmp. The source schema comes from the pinned original C code, not from generic MessagePack conventions.
