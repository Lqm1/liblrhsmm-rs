# C interface implementation status

The optional `c-api` feature adds an isolated C boundary to this existing package.
Native modules deny unsafe code; with the feature disabled the crate forbids it.
Only the feature-gated boundary permits unsafe operations, and it denies implicit
unsafe operations inside unsafe functions. Default serialization and dependency
versions are unchanged. Cargo emits rlib, shared and static libraries.

Build with `cargo build --locked --features c-api --lib`. Regenerate the checked-in
header using cbindgen 0.29.4:

```text
cbindgen --config cbindgen.toml --crate liblrhsmm-rs --output include/liblrhsmm_rs.h
```

The current header exports 54 functions, five descriptors, four opaque duration
owners and two table selectors. Of the 103 original functions in
`binding-inventory.csv`, eight have verified complete C mappings and 95 remain
pending. Both unprepared and prepared duration density/release endpoints are
verified. Gaussian and whole-model preparation remain pending.
`binding-configuration-inventory.csv` separately
records numerical globals/constants, all nine common configuration globals and
the build precision setting. Nine numerical configurations have verified C access;
the common settings and complete engine precision coverage remain pending.
Function and configuration counts do not establish structure-field coverage.
Every public field and macro in `header-coverage.md`, prepared caches, builders,
datasets, memory-pool state, inference views and training accumulators must also
receive usable C/WASM interfaces and executable evidence. All WASM work is pending.

The C names use the `liblrhsmm_rs_` prefix. Status zero means success, one an invalid
pointer, two an invalid range, three invalid configuration/calculation and four a
caught unwinding panic. Allocation validity, exclusive access and lifetime remain
caller obligations; pointer checks can only validate nullness/alignment and range
limits. Output storage must remain independent and exclusively writable. Results
are staged before publication; errors leave outputs unchanged. Panics are contained
at the boundary, including panic payloads with panicking destructors. Scalar output
pointers avoid i686 floating-point return-register precision changes.

## Numerical kernels and mutable coefficient tables

Both precisions expose source-compatible log-add, negative exponential and
exponential kernels. Table coefficients come from the unchanged original decimal
source data, rounded independently to each precision. Quantization, cutoff and
the source's float-only positive exponential branch remain native calculations;
the boundary does not duplicate them or introduce a finite-value filter. Defined
nonfinite behavior follows the existing safe Rust corrections.

Table zero selects log-add coefficients; table one selects negative-exponential
coefficients. Both have 7,000 entries. `table_get` reads one entry, `table_replace`
atomically replaces it and returns its previous bits, and `table_copy` copies a
checked interval into caller storage. Any scalar bits are accepted. Invalid
output storage is checked before replacement, so a failed request does not change
the table. Zero-length copies permit null output at a valid offset, including the
end. Each entry is atomic, but a table copy or inference call is not a whole-table
snapshot. Configure coefficients before computation for reproducible results.
Changes affect subsequent native helper/inference/training operations process-wide;
binary32 and binary64 tables remain independent.

`LiblrhsmmRsNumericalConstants` retains both table sizes, binary64 quantization
steps and occupancy epsilon, the selected-precision finite log-zero sentinels and
the standard binary64 PI value. It exposes the source constants without changing
their precision or recomputing table entries analytically.

## Verification checkpoint

Six boundary tests retain both complete unchanged NUM1 fixtures. Each precision
compares all 14,000 coefficient bits and every one of 42,008 original helper
results. Tests also verify coefficient mutation and bit preservation, native helper
effects, restoration, invalid-output nonmutation, invalid ranges, empty intervals,
constants, pointer alignment and IEEE limits. A separate unit test verifies unwind
containment with a panicking payload destructor. Actual optimized assert-enabled C
and Python callers retain both complete original corpora and invoke all 14 exports.

All five targets passed the complete package test suite with `c-api` enabled:
36 suites and 127 tests, with zero failures or ignored tests. The previous 120
native tests remain present; seven boundary tests were added. Both expected E0502
compile-fail borrow tests were checked explicitly. C callers passed on Windows
x86_64/i686 MSVC, Windows x86_64 GNU and Linux x86_64/i686 GNU; Python callers passed
on all three 64bit targets. The no-default-features C boundary also compiles,
preserving optional serialization. Clippy, documentation, formatting and generated
header checks passed. Native algorithms, fixtures and Cargo.lock are unchanged.

This checkpoint verifies Debug shared libraries. Release, Python32 and static-link
verification are not claimed. Model/stream/Gaussian ownership and editing,
data/segmentation/dataset operations, all codecs and borrowed I/O, preparation,
emissions, HMM/HSMM inference and posteriors, statistics/training, generation,
memory pools and all field/configuration mappings remain required. SHIRO C,
all three browser/Node WASM interfaces and final combined acceptance remain required.

Primary configuration and interface references:

- https://doc.rust-lang.org/cargo/reference/cargo-targets.html
- https://doc.rust-lang.org/cargo/reference/features.html
- https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-attributes.html
- https://doc.rust-lang.org/nomicon/ffi.html
- https://doc.rust-lang.org/std/panic/fn.catch_unwind.html
- https://doc.rust-lang.org/std/attribute.forbid.html

## Owned duration parameters and unprepared densities

Each precision provides an opaque duration owner and a complete descriptor with
`mean`, `variance`, `minimum`, `maximum`, `fixed_mean` and `variance_floor`.
`duration_default` writes the source flat-start values. `duration_create` copies a
descriptor, `duration_get` copies all fields, and `duration_set` replaces them.
Parameter storage preserves arbitrary scalar bits and zero flat-start variance;
density operations apply the native finite-value and positive-variance checks.
`duration_clone` creates an independent owner. `duration_release` consumes a unique
owner through an initialized pointer slot and clears that slot; a null owned
pointer is a successful no-op. Never overwrite a live owner slot with another
constructor result. Release or mutate only with exclusive access. No read may
overlap mutation or release. Descriptors and outputs must be independent storage.

`duration_log_probability` accepts selected-precision scalar frames and weight.
`duration_annealed_log_probability` takes an integer frame count and explicit
weight/temperature. `duration_probability` returns the native f64 density in both
precisions, retaining the source float-only exponential. All calculations delegate
to the unchanged native implementation. The maximum duration constrains searches,
not standalone densities. Zero temperature still excludes frames below the
minimum. Errors leave result storage unchanged and do not modify the parameters.

Two additional boundary tests compare all 81 duration results per precision from
the unchanged original `c-precision-float.txt` and `c-precision-double.txt` corpora,
using the existing normalized 2e-14 bound and exact agreement with native result
bits. They also verify every field, NaN payload readback, independent cloning,
flat-start storage, invalid inputs and output preservation, zero-temperature
minimum exclusion, evaluation beyond the maximum and owner clearing. Actual
optimized assert-enabled C and Python callers invoke all 18 duration exports and
compare all 162 original C values. C passed on all five native targets and Python
on all three 64bit targets.

The complete package suite now passes on every target: 37 suites /129 tests, zero
failures or ignored tests. All previous 120 native tests and seven numerical
boundary tests remain; both expected E0502 compile-fail borrow tests are verified.
Clippy, Rustdoc with warnings denied, optional-serialization compilation,
formatting, generated-header and native-source preservation checks passed.

At the unprepared checkpoint, duration preparation, normalizer/cache access,
statistics, updates and codecs remained required. The prepared checkpoint below
completes the preparation/cache and prepared density/release portions. Public Gaussian/stream/model parameters and builders remain required,
as do every other outstanding C mapping, all three WASM interfaces and final
combined acceptance. This checkpoint verifies Debug shared libraries; Release,
Python32 and static-link checks are not claimed.

## Independent prepared-duration snapshots

`DurationParameters::prepare_owned` consumes parameters into an immutable
`OwnedPreparedDurationParameters<T>`. Its normalizer, options, cache access and
probability methods use the same private `DurationPreparation<T>` state and
calculation implementation as the existing borrowed preparation. This factors
private storage without duplicating or changing arithmetic. Borrowed public
methods and lifetime restrictions remain intact. `Clone` independently duplicates
parameters and cache storage. `parameters()` borrows immutable parameters;
`into_parameters()` discards derived state and recovers editable parameters.
No self-reference, unsafe native code or additional dependency is required.

`duration_prepare` copies the source C owner into an independent snapshot using a
complete configuration descriptor: cache length, duration weight and temperature.
The settings default to 300, one and one. Successful preparation freezes these
values with its own parameters; later source edits or release cannot stale or free
the snapshot. Reprepare explicitly to use edited parameters. Global exponential
coefficient tables still affect probability evaluation through the native kernel.

`prepared_duration_parameters`, `prepared_duration_options`,
`prepared_duration_log_normalizer` and `prepared_duration_cache_length` expose the
complete immutable state. `prepared_duration_cache_copy` copies a checked interval
without borrowing an internal pointer. Zero-length copies permit null output at
valid boundaries, including an empty cache; overflow/out-of-range requests retain
output storage. `prepared_duration_log_probability` uses native cache lookup and
cached-normalizer fallback; `prepared_duration_probability` returns the native
f64 value in both precisions. `prepared_duration_clone` copies all state without
recomputation. `prepared_duration_release` clears the initialized unique-owner
slot and releases parameters/cache; a null owned pointer is a successful no-op.

Three new native tests cover both precisions, cache lengths 0/1/3/8/300,
temperatures 0/0.5/1/1.75 and frames 0 through 400. Owned/borrowed state and outputs
agree exactly; clone storage is independent and parameter recovery cannot alter
a clone. Original C normalizer/cache and density corpora retain the existing
2e-14 bound. Invalid parameters/settings remain errors. Two additional C boundary
tests check exact native bits, original cache fields, source editing/release,
clone/release ordering, empty fallback, retained outputs, null/misaligned storage
and checked cache intervals.

Actual optimized assert-enabled C and Python callers invoke all 22 new exports
and verify 270 original C values: both precisions' nine normalizer/cache values and
54 log-density/density values, plus all 144 values in the binary32 preparation
corpus. The initial C reader mistakenly applied the three-column cache format to
the two-column normalizer row; it was corrected to parse each original format.
The fixture, numerical tolerance and calculations were not changed.

All five targets passed the complete package suite: 39 suites /134 tests, zero
failures or ignored tests. The original 120 native tests remain, along with three
new owned-preparation tests and 11 total boundary tests. Both E0502 compile-fail
borrow checks passed. All three C callers were rerun on all five targets; all
three Python callers passed on all three 64bit targets. Native borrowed and owned
preparation also passed with default features disabled. Optional serialization,
Clippy, Rustdoc with warnings denied, formatting and generated-header checks pass.

Only private duration preparation storage was factored; all other native
algorithms, original fixtures, Cargo.toml and Cargo.lock are unchanged. Header
checks cover all 54 functions, complete descriptors and all four owners. Duration
statistics, training updates and codecs, Gaussian/stream/model parameters,
builders, whole-model preparation and every remaining C operation/configuration
are still required. All three browser/Node WASM interfaces and final combined
acceptance remain required. Current proof covers Debug shared libraries; Release,
Python32 and static-link verification are not claimed.

Primary ownership references:

- https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
- https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
