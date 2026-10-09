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
mappings are implemented. Together with the duration interface below, twelve
original mappings are implemented; the other 91 remain pending.

## Duration distributions, preparation, statistics and codecs

`DurationF32/F64` retains all six native parameter fields. `values()` and
`set_values()` transfer mean, variance and variance floor in the selected typed
array; `constraints()` and `set_constraints()` transfer minimum, maximum and
fixed mean as Int32Array. Scalar storage preserves arbitrary IEEE bits.
Invalid array shapes leave the owner unchanged. New owners use native defaults;
`cloned()` creates independent storage. Density methods are `log_probability`,
`annealed_log_probability` and `probability`, with native validation and
minimum/maximum/zero-temperature behavior.

`PreparationOptionsF32/F64` exposes every native field, duration_cache_length,
duration_weight and temperature, with the native defaults. `prepare(options)`
creates an independent immutable snapshot. `prepare_owned(options)` consumes
the editable owner. `PreparedDurationF32/F64` exposes parameters(),
into_parameters(), options(), log_normalizer(), cached_log_probabilities(),
log_probability(), probability() and cloned(). Returned parameters, options and
cache arrays are independent copies. Snapshots retain matching parameters and
cache after source edits or release. Borrowed preparation is represented by an
owned snapshot across the JavaScript boundary.

`DurationStatisticsF32/F64` exposes value_sum, squared_sum and occupancy,
typed values()/set_values(), cloned() and clear(). `UpdateOptionsF32/F64`
exposes duration_mode, duration_variance_floor and emission_variance_floor.
Mode0 selects Normal and mode1 Geometric; invalid codes throw before mutation.
All defaults and native update settings are retained. update(statistics,
options) returns the native occupancy/change flag, preserves failure atomicity,
and includes the corrected fixed-mean residual variance behavior.

With serialization enabled, Duration static read()/read_with_limits() reject
trailing bytes; read_prefix()/read_prefix_with_limits() return a
`DecodedDurationF32/F64` owner with the consumed byte position as BigInt.
parameters() copies the decoded value; into_parameters() consumes it. This
allows sequential readers to retain the exact original object boundary.
write() returns an independent Uint8Array in the original binary32 MessagePack
schema, including binary64-to-wire rounding. Native decode limits, truncation
and malformed-input errors remain active. The serialization feature disables
these methods and decoded owners independently of the remaining WASM interface.
OS IO callbacks remain available in the native/C interfaces.

Actual Node.js with and without serialization and browser checks consume all
144 original C preparation cases and ten original C duration update cases.
Binary32 preparation and both update precisions compare exactly; binary64
preparation uses the existing native normalized bound2e-14 and measured zero
error. Both precisions reproduce two C wire fixtures, totaling76 byte
comparisons, and verify every truncated prefix, exact prefix positions and
decode budgets. Additional checks cover complete fields/defaults, independent
ownership, source mutation/release, consuming conversions, copied caches,
IEEE payloads, invalid settings/statistics/codes, atomic errors, occupancy
flags, geometric updates and corrected fixed-mean variance. Related native/C
tests29 pass. Clippy passes in both serialization configurations; WASM
Rustdoc denies warnings. The actual feature-disabled module has none of the
twelve owner classes. This checkpoint adds nine original mappings; it does not
complete model, inference or training bindings.

## Primary references

- [Numeric slices](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/number-slices.html)
- [Returned numeric arrays](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/boxed-slices.html)
- [Exported owner classes](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/exported-rust-types.html)
- [Returned Result errors](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/result.html)
- [Deployment targets](https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html)
- [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [DoubleEndedIterator](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)
- [ExactSizeIterator](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)
