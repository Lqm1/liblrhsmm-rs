# WebAssembly interface

The optional `wasm` feature exposes browser and Node.js interfaces from this
same library crate. It uses version-matched wasm-bindgen 0.2.129 and js-sys
0.3.106 on wasm32-unknown-unknown, with the existing Rust 1.94 toolchain.
The existing serialization feature remains independent. Generated packages
are build artifacts, not additional Rust crates or repositories.

```sh
cargo build --locked --lib --features wasm --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/liblrhsmm_rs.wasm --target nodejs --out-dir pkg-node
wasm-bindgen target/wasm32-unknown-unknown/debug/liblrhsmm_rs.wasm --target web --out-dir pkg-web
node tests/wasm_numerics_node.cjs ./pkg-node
```

Serve tests/wasm_numerics_browser.html over HTTP, with /bindings pointing to
the generated web output and /tests to the test directory. Browser initialization
is asynchronous; Node output uses CommonJS. Typed arrays own independent storage.
Ordinary Result errors become JavaScript exceptions. A WASM panic abort is not
recoverable. Full model, inference, training and remaining public interfaces
are still being implemented; this numerical checkpoint is not full acceptance.

## Complete numerical helpers and mutable coefficient tables

`numerical_log_add_f32/f64`, `numerical_negative_exponential_f32/f64` and
`numerical_exponential_f32/f64` delegate directly to all native public helpers.
Both table precisions, quantization steps, cutoffs, finite log-zero sentinels,
IEEE domains and the float-only positive exponential approximation are retained.
`numerical_constants()` returns all seven native constants in an independent
Float64Array: log table length/step, exponential table length/step, occupancy
epsilon and the selected binary32/binary64 log-zero sentinels.

`NumericalTableF32/F64(code)` references a module-wide native table.
`NumericalTable.LogAdd=0` and `Exponential=1` select the table; invalid codes
fail inside Result. Every public table operation is exposed: length,
is_empty(), get(index), replace(index, value), values() and iter(). Invalid
indices return undefined without mutation. replace returns the previous value.
`get_storage(index)` and `replace_storage(index, values)` use typed arrays to
preserve arbitrary IEEE bits, including NaN payloads. Replacement requires
exactly one value; invalid shape leaves storage unchanged.

Cloned handles reference the same global table. The two tables and two
precisions remain independent. Returned value arrays are independent copies;
iteration reads each native atomic coefficient as it is requested, rather than
freezing a whole-table snapshot. Changes affect subsequent native computation
in that same WASM module instance. Different instances have independent memory.

`NumericalIteratorF32/F64` exposes remaining length, next(), next_back(),
next_storage() and next_back_storage(). It preserves live double-ended,
exact-size and exhausted-iterator semantics. Standalone
`numerical_log_add_table_f32/f64()` and `numerical_exponential_table_f32/f64()`
return equivalent iterators. Iterators remain usable after table-handle release.
All four owner classes provide generated free().

Actual Node.js and browser tests consume both complete NUM1 C corpora:
84,016 helper cases and 28,000 coefficients, all exact. Additional checks cover
all native constants, full forward/backward traversal, shared handles,
coefficient edits affecting helpers and existing iterators, typed NaN payloads,
shape/index/code errors, snapshot lifetimes, source nonfinite/cutoff behavior
and binary64 precision. Tests restore every modified global coefficient.
Eleven related native/C tests pass, including table changes reaching native
inference and training. WASM Clippy and Rustdoc deny warnings; feature-disabled
checks confirm absence of eleven functions and four owners. The native
algorithms and original fixtures are unchanged. Three original numerical
mappings are implemented; the other 100 public mappings remain pending.

## Primary references

- [Numeric slices](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/number-slices.html)
- [Returned numeric arrays](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/boxed-slices.html)
- [Exported owner classes](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/exported-rust-types.html)
- [Returned Result errors](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/result.html)
- [Deployment targets](https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html)
- [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [DoubleEndedIterator](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)
- [ExactSizeIterator](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)
