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
- Generic trait bounds: https://doc.rust-lang.org/reference/trait-bounds.html
- Scalar arithmetic traits: https://doc.rust-lang.org/std/ops/index.html
- Widening scalar conversion: https://doc.rust-lang.org/std/convert/trait.Into.html
- Low-level MessagePack operations: https://docs.rs/rmp/0.8.15/rmp/
- Exact-length mapped slice iteration: https://doc.rust-lang.org/std/iter/struct.Map.html
- Allocation-free coefficient decoding: https://doc.rust-lang.org/std/slice/struct.ChunksExact.html
- Bit-preserving coefficient updates: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU32.html and https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU64.html
- Independent coefficient ordering: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html

Context7 and the Mintlify documentation index were consulted before using rmp. The source schema comes from the pinned original C code, not from generic MessagePack conventions.
