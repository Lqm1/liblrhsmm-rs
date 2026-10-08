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

The current header exports 91 functions, 12 descriptors, seven opaque owners
and seven table/mode/callback selectors. Eleven codec exports and the decoder
limit descriptor require the `serialization` feature. Of the 103 original functions in
`binding-inventory.csv`, 14 have verified complete C mappings and 89 remain
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

## Editable duration statistics and maximization

Both precisions now expose independent statistics owners and complete descriptors
with `value_sum`, `squared_sum` and `occupancy`. `duration_statistics_default` writes
all zero values. `duration_statistics_create` copies a descriptor, while
`duration_statistics_from_duration` constructs zero statistics from a live
parameter owner, including an uninitialized distribution. Neither retains the
source duration. `duration_statistics_get`/`set` copy all fields, allowing callers
the same editing and accumulation through public values as native Rust. Storage
retains arbitrary scalar bits; native update performs numerical validation.
`duration_statistics_clone` creates an independent owner, `clear` delegates native
clearing, and `release` consumes/clears a unique-owner slot and permits a null
owned pointer. Full model initialization/EM accumulation is still pending.

The complete update descriptor contains `duration_mode`,
`duration_variance_floor` and `emission_variance_floor`. Mode selectors zero and
one map to normal and geometric; unknown codes return status three. Defaults come
from native `UpdateSettings`, retaining normal mode and both selected-precision
variance floors. Duration update validates both floors, including the emission
floor, as the native implementation does.

`duration_update` stages a parameter copy, calls the existing native maximization
and publishes only success. Its u32 result is one for an update and zero for
insufficient occupancy. The result pointer is checked before mutation. Invalid
configuration/statistics, unknown modes, nonfinite or overflowing calculations
retain parameters, statistics and caller results. Native occupancy epsilon,
per-distribution floors, integer fixed mean and corrected residual variance are
unchanged. Prepared snapshots keep their independent parameters/caches.

Four new boundary tests cover both precisions' complete original duration update
corpora, exact native results and update flags. Binary32 checks round-trip the
source's nine-digit values to the same bits; binary64 retains the existing
normalized 2e-14 bound. Tests also cover all fields, NaN payload storage, independent
clone/source lifetimes, clearing, release/null slots, fixed-mean residual variance,
unknown mode, invalid floors/statistics, overflow, invalid geometric mean and
failure/output atomicity. Actual optimized assert-enabled C and Python callers
invoke all 20 new functions and compare all 20 original C duration update values.

All five targets passed the complete package suite: 40 suites /138 tests, zero
failures or ignored tests. The original 120 native tests and three owned-duration
preparation tests remain; 15 total boundary tests now pass. Both expected E0502
compile-fail borrow gates were verified. All four C callers passed on all five
targets; all four Python callers passed on all three 64bit targets. Together they
exercise all 74 exports. Clippy, optional serialization/c-api compilation,
Rustdoc with warnings denied, formatting and generated-header checks passed.
Native sources outside the opt-in C boundary, all fixtures, Cargo.toml and
Cargo.lock are unchanged from the prepared-duration checkpoint.

Duration codecs, full model statistics/initialization/EM accumulation,
Gaussian/stream/model parameter ownership/builders, their preparation and updates,
all other C operations/configuration/fields and all three browser/Node WASM
interfaces remain required. The settings inventory records duration-only floor
coverage while keeping the corresponding full-model mappings pending. Final
combined native/binding acceptance is still required. Current proof is Debug
shared libraries; Release, Python32 and static-link verification are not claimed.

## Byte ownership and synchronous codec I/O

`bytes_create` copies caller bytes into a new immutable owner, `bytes_clone` makes
an independent copy, `bytes_length` returns its length and `bytes_copy` copies a
checked interval into independent storage. Empty input/copies permit null pointers
at valid boundaries. Overflow/out-of-range requests retain output. Byte spans use
address-space bounds rather than the legacy i32 frame-count ceiling.
`bytes_release` consumes and clears initialized unique-owner storage, with a null
owned pointer as a successful no-op. Internal pointers are not borrowed by callers.

Read/write descriptors borrow callbacks and opaque context only for a synchronous
call. Status zero succeeds, one reports `Interrupted`, and any other callback code
is an I/O error. Transfer counts cannot exceed supplied capacity. Short transfers
and interruption retry follow the native `Read`/`Write` codecs; zero read means EOF
and zero write produces the native write-zero error. Callbacks must not unwind,
retain borrowed buffers or access owner/result storage. Independent callback
context may be modified. The library never closes or retains the context.

Encoding never flushes implicitly. `write_stream_flush` explicitly delegates the
optional flush callback; a missing, interrupted or failed flush returns status
three and callers decide whether to retry. A failed decode may have consumed an
input prefix; failed stream encoding may have emitted an output prefix. Parameter
owners and result slots remain unchanged on failures. Codec/I/O errors map to
status three, while pointer/range and contained-panic statuses are unchanged.

## Legacy duration codecs and object boundaries

The optional codec uses the existing native decoder/encoder, preserving the
original six-field binary32 wire layout in both storage precisions. Full native
`DecodeLimits::max_array_entries` and its default are exposed. `duration_read_bytes`
and `duration_read_stream` require exactly one complete object and reject trailing
data; the borrowed reader must reach EOF for the latter check.

Original C `lrh_read_duration` consumes one object and leaves subsequent bytes
unread. Added native `DurationParameters::read_prefix` and
`read_prefix_with_limits` call the same private decoder without its EOF check.
Existing strict methods stay unchanged. `duration_read_stream_prefix` exposes that
source-compatible boundary in each precision, allowing repeated objects through
the same context. This avoids narrowing the source function to standalone files.
`duration_write_bytes` returns a new byte owner; `duration_write_stream` uses
borrowed callbacks. No MessagePack schema or floating-point calculation is
reimplemented at the boundary.

Define `LIBLRHSMM_RS_SERIALIZATION` before including the generated C header only
when linking a build with Rust's `serialization` feature enabled. Default Cargo
features include it; `--no-default-features --features c-api` does not. cbindgen's
`[defines]` mapping guards all eleven codec declarations and the decoder limits
struct. Byte ownership and explicit flush remain available without serialization;
Cargo feature/dependency settings are unchanged.

Four new boundary tests verify exact original duration bytes, all truncation
positions, trailing data, limits at five/six entries, short transfers/interruption,
callback failures/overcounts/write-zero, missing callbacks, explicit flush behavior,
byte cloning/intervals and original binary64-to-binary32 wire rounding. Two native
prefix tests verify both precisions' two-object streams and remaining sentinel
bytes, strict-reader rejection, truncation and limits. Actual assert-enabled C
and Python callers invoke all 17 new exports, round-trip the original fixture
byte-for-byte and retain sequential boundaries.

All five targets passed the complete package suite: 42 suites /144 tests, zero
failures or ignored tests, retaining the original 120 native tests and three
owned-preparation tests and adding two prefix tests; 19 total boundary tests pass.
Both expected E0502 compile-fail borrow checks were verified. All five C callers
passed on all five targets; all five Python callers passed on all three 64bit
targets, exercising all 91 exports. Host C/Python callers also verified the six
new byte/I/O helpers with serialization disabled and Python confirmed that all
11 optional codec exports are absent. Both feature configurations passed Clippy;
Rustdoc with warnings denied, formatting and header verification pass.

Only two native duration prefix methods were added; existing native code paths,
fixtures, Cargo.toml and Cargo.lock are unchanged. The source-boundary audit
reopened the seven other public native reader mappings as partial: standalone
files are verified, but one-object stream-prefix parity must also be implemented
and tested before full native coverage is claimed. This is the next required
repair, followed by the remaining parameter/data/statistics/inference and codec
C bindings. All WASM work and final combined native/binding acceptance remain
required. Current proof is Debug shared libraries; Release, Python32 and static
link verification are not claimed.

Primary I/O and header references:

- https://doc.rust-lang.org/stable/std/io/trait.Read.html
- https://doc.rust-lang.org/stable/std/io/trait.Write.html
- https://github.com/mozilla/cbindgen/blob/main/_autodocs/configuration.md
