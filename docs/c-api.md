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
fixtures, Cargo.toml and Cargo.lock are unchanged. The source-boundary audit initially reopened seven other native reader mappings.
The follow-up below verifies their source-compatible one-object reading. Their
parameter/data/statistics/inference and codec C bindings remain required. All WASM work and final combined native/binding acceptance remain
required. Current proof is Debug shared libraries; Release, Python32 and static
link verification are not claimed.

Primary I/O and header references:

- https://doc.rust-lang.org/stable/std/io/trait.Read.html
- https://doc.rust-lang.org/stable/std/io/trait.Write.html
- https://github.com/mozilla/cbindgen/blob/main/_autodocs/configuration.md

## Native stream-prefix parity follow-up

All seven remaining public reader families now have prefix readers that delegate
to the existing native Decoder without requiring EOF: GMM, stream, model,
observation, segmentation, observation set and segmentation set. Each exposes
caller-selected cumulative array limits. Set readers support both selected
precision and default binary32 entry points. Existing strict standalone readers
and every writer remain unchanged. Legacy segmentation header mismatches remain
supported, including nested sets.

`tests/serial_prefix.rs` checks original C fixture values, two consecutive objects,
exact consumed positions and a trailing sentinel for both precisions. A one-byte
reader that panics on any read beyond the fixture confirms no EOF probe occurs.
Zero-entry limits and truncation failures are checked; small fixtures check every
truncation position, while the 127,957-byte model checks header/interior/tail
positions to avoid quadratic test cost.

All five required native targets passed 43 suites /146 tests with zero failures
or ignored tests, retaining all previous tests and both E0502 compile-fail gates.
All five existing C callers passed on all five targets; Python callers passed on
all three 64bit targets. C exports and header remain unchanged at 91 functions.
Clippy with warnings denied, Rustdoc with warnings denied and formatting pass.
These seven native inventory rows are restored to implemented. Their C bindings,
all WASM bindings and final combined native/binding acceptance remain pending.

## Gaussian parameter ownership and transformations

Both precisions now expose 11 operations: flat-start `new`, complete borrowed
array `create`, atomic `replace`, `layout`, interval `copy_field`/`set_field`,
independent `clone`, consuming `release`, native `combine`, `from_durations`
and atomic `increase_mixtures`. Owners retain mixture count, dimensions, weights,
mixture-major means, variances and variance floors. Input descriptors declare
both shape values and all four array pointers; array lengths follow the shape.
Input arrays are copied during the call and are never retained. Editing uses
selectors 0/1/2/3 for weights/means/variances/floors and does not expose internal
pointers. Full replacement can change shape; interval edits preserve shape.
All scalar bits, including NaNs and signed zero, remain storable as in native
public parameter vectors. Numerical preparation/splitting validates its own
requirements separately. Flat-start zero variances remain supported.

Parameter arrays use address-space bounds; individual mixture/dimension counts
retain the native legacy-format limits. Checked intervals and empty copies allow
null buffers only for zero length at a valid offset. Unknown selectors return 3;
invalid intervals return 2. Failed creation preserves the output slot; failed
replacement/edit/splitting preserves the owner. Splitting stages a native clone
before applying `increase_mixtures`. Combining borrows an array of live owners,
which may repeat, and returns independent storage. Duration conversion retains
native behavior, including zero output variance floors. Empty combinations fail
through the same native validation. Release consumes unique ownership and clears
its initialized slot; it is a no-op for null ownership.

Evidence: two tests in `tests/c_api_gaussian.rs` exercise all 22 exports, exact
native parameter bits, original C splitting corpora, independent copies, every
field, NaN/signed-zero edits, shape replacement, invalid intervals and atomic
failure. Assert-enabled optimized C and Python callers each verify all 22 exports
and 70 original C parameter values across both precisions. All five required
native targets pass 44 suites /148 tests with zero failures/ignored tests,
retaining both expected E0502 compile-fail gates. All six C callers pass on all
five targets; all six Python callers pass on all three 64bit targets. Host C and
Python Gaussian callers also pass with serialization disabled. Enabled/disabled
Clippy, Rustdoc with warnings denied, formatting and generated-header verification
pass. The header has 113 exports, 15 descriptors and nine opaque owners; the
11 optional serialization declarations stay feature-guarded. Native sources,
Cargo manifests/lock and original fixtures are unchanged from 2e9aebe.

The function inventory has 19 implemented C mappings and 84 pending mappings.
Gaussian preparation/cache/densities, statistics/update and serialization,
stream/model builders and all remaining public operations still require bindings.
All WASM bindings and final combined native/binding acceptance remain required.
Debug shared-library evidence does not establish Release/static-link/Python32
acceptance. Full scope is unchanged.

## Owned prepared Gaussian snapshots

`GaussianMixtureParameters<T>::prepare_owned` consumes initialized parameters
into an immutable `OwnedPreparedGaussianParameters<T>` containing its own
parameter vectors and matching normalizers. Cloning copies both. `parameters()`
borrows immutable parameters; `into_parameters()` discards derived state and
returns editable ownership. Existing `prepare()` remains borrowed and prevents
source mutation while in use. Both paths build the same normalizers and delegate
density evaluation to a private temporary borrowed view. It contains references
to parameters and normalizers only during evaluation; it has no self-reference,
cache copy or unsafe lifetime extension. The evaluation and approximation kernel
bodies are unchanged from b03e2ba, including accumulation order and rounding.

The C interface adds 20 exports: per-precision `gaussian_prepare` plus prepared
`parameters`, `clone`, `release`, `cache_length`, `cache_copy`,
`component_log_probability`, `component_probability`, `log_probability` and
`probability`. Preparing snapshots the source; subsequent source edits/release
cannot invalidate its caches or results. Getting parameters returns an independent
editable owner. Cache reads expose sums before the factor of one half, through
checked interval copies. Zero-length valid intervals permit null output. Density
calls borrow observations synchronously and stage scalar outputs, retaining old
outputs on errors. Both probability functions return f64 in both precisions;
log-density outputs retain the selected precision. Native parameter/observation,
component and temperature validation stays unchanged. Single-component mixtures
continue to ignore weight; zero DAEM temperature retains native behavior.

Three native tests cover both precisions, component counts 1/2/5, dimensions
1/2/4, five observation ranges and four temperatures, with exact borrowed/owned
bits, independent clones and parameter recovery. Two boundary tests cover all
20 exports, original C densities/cache values, source edits/release, editable
parameter copies, invalid pointers/ranges/components/observations/temperatures,
atomic output failures and release/null ownership. Actual optimized assert-enabled
C and Python callers each exercise all 20 exports and 166 original C cached and
density values. Existing original fixtures and tolerance evidence are retained.

All five native targets passed 46 suites /153 tests, zero failed/ignored,
retaining the original 120 native tests, six owned-preparation tests, four prefix
tests, 23 boundary tests and both E0502 compile-fail gates. All seven C callers
passed on all five targets; all seven Python callers passed on all three 64bit
targets. Host C/Python prepared Gaussian callers pass with serialization disabled.
Enabled/disabled-feature Clippy, Rustdoc with warnings denied, formatting and
cbindgen verification pass. The header has 133 exports, 15 descriptors and
11 opaque owners; all 11 optional serialization declarations stay guarded.
Native changes are confined to shared Gaussian preparation/evaluation ownership;
Cargo manifests/lock and all original fixtures remain unchanged.

The complete Gaussian deletion mapping now includes editable and prepared-owner
release. The function inventory has 25 implemented C mappings and 78 pending.
Gaussian statistics/update/codecs, stream/model owners/builders, datasets,
inference/training/generation/pools and remaining configuration bindings remain
required. All WASM and final combined native/binding acceptance remain pending.
Debug shared-library evidence does not establish Release/static-link/Python32
acceptance. Full public scope is unchanged.

## Gaussian sufficient statistics and maximization

The C interface adds 24 functions: per-precision statistics `new`, `create`,
`replace`, `from_gaussian`, `layout`, `set_occupancy`, interval `copy_field` and
`set_field`, independent `clone`, native `clear`, consuming `release`, plus
`gaussian_update`. Complete descriptors carry mixture count, dimensions, state
occupancy and borrowed pointers to weighted sums, weighted squared sums and
component occupancies. Shape derives the array lengths. Constructors copy inputs;
full replacement can change shape and stages all allocation before publication.
Numeric values remain editable without premature validation, including negative
weighted sums, NaNs and signed zero. Layout reads all scalar metadata; selectors
0/1/2 access the three vector fields independently. Checked zero-length valid
intervals allow null buffers; unknown selectors return 3, invalid ranges 2.

Native `GaussianSufficientStatistics::new` allocates zeroed vectors directly.
`from_gaussian` retains structural validation and delegates to this constructor.
The parameter constructor's original shape checks are shared through a private
helper, avoiding temporary model vectors solely for shape validation. Original
shape checks and all maximization arithmetic remain unchanged.

`gaussian_update` reuses the existing complete update descriptor and native
`GaussianMixtureParameters::update`. The native operation already stages the
updated model, so the C interface does not add redundant model clones. It
validates the scalar output before invoking mutation. Errors leave model,
statistics and output unchanged; success reports 1, or 0 when state occupancy
is too low. Low component occupancy retains that component's parameters. Native
mean/variance arithmetic, per-parameter and common floors, weight convention and
all option validation remain unchanged. Statistics survive model release and
are cleared through the same native kernel without changing shape.

One native test verifies direct statistics against model-derived allocation in
both precisions and rejects invalid/address-overflow shapes without allocation.
Four boundary tests cover all 24 exports, metadata/vector edits, NaN/signed-zero
bits, clone independence, resizing, clear, checked intervals, output errors,
original C update values and overflow in a later component after earlier staged
updates. The latter preserves all model fields and the result slot. Actual
optimized assert-enabled C and Python callers exercise all 24 exports and 70
original C update values, including low state/component occupancy and floors.

All five required targets passed 48 suites /158 tests, zero failed/ignored,
retaining both E0502 compile-fail gates. All eight C callers pass on all five
targets; all eight Python callers pass on all three 64bit targets. Host C/Python
statistics callers pass with serialization disabled. Enabled/disabled Clippy,
Rustdoc with warnings denied, formatting and generated-header verification pass.
The header contains 157 functions, 19 descriptors and 13 opaque owners; all
11 optional codec declarations remain feature-guarded. Cargo manifests/lock,
original fixtures and native maximization source remain unchanged.

The function inventory has 29 implemented C mappings and 74 pending. Common
configuration coverage remains nine implemented and ten pending: standalone
duration/Gaussian floor usage is verified, while complete stream/model usage still
needs bindings. Gaussian codecs, remaining model/data/inference/training/generation/
pool C interfaces, all WASM and final combined native/binding acceptance remain
required. Debug shared-library proof does not establish Release/static-link/
Python32 acceptance. Full public scope is unchanged.

## Gaussian serialization and historical schemas

The optional `serialization` feature adds ten Gaussian codec exports: per precision
`read_bytes`, strict `read_stream`, one-object `read_stream_prefix`, `write_bytes`
and `write_stream`. They delegate the existing native decoder/writer and reuse
owned bytes, complete decode limits and synchronous borrowed callback adapters.
Bytes and strict streams reject trailing data; prefix streams consume one object
without EOF probing and preserve subsequent objects. Output-owner slots remain
unchanged on decoding errors. Read failures can consume a prefix of the input.

Writers take an explicit encoding selector: `WITH_VARIANCE_FLOORS` is zero and
`WITHOUT_VARIANCE_FLOORS` is one. Current and historical layouts preserve native
binary32 wire values for both storage precisions. Unknown selectors return 3
before writer callbacks; historical output with nonzero floors is rejected rather
than discarding parameters. Native floor validation happens before encoding bytes.
Current encoding retains signed-zero floor bits; historical encoding has no floor
field and reading it supplies native positive zero. The source owner is never
changed by encoding. Short transfers, interruption retry, I/O failures, overcounts,
write-zero and explicit flushing retain the existing adapter behavior. Encoding
never implicitly flushes, closes or retains foreign context; write failures may
leave a partial prefix in the output.

`tests/extract_historical_gaussian.py` extracts an unchanged byte span from
SHIRO's original bundled pretrained model, without reserializing values. The first
GMM is model bytes [11:143], one component and 12 dimensions. Its standalone
132-byte fixture SHA-256 is
`d75474b673811f9cb5797bf8bfd51d38bb7b559667556d4644976dbd6b1e7b2f`.
The independent native test checks that exact span, original model fields and
historical byte-for-byte re-encoding in both precisions. The existing current
fixture remains unchanged. Declared-array entry budgets are 18 for the current
fixture and 29 for the historical fixture; each is tested at the exact bound and
one below, with every truncation position checked.

Three boundary tests cover all ten new exports, full decoded parameter bits,
both original schemas, byte/stream round trips, two consecutive objects followed
by a sentinel, limits, callback failures and prevalidation without callback calls.
A binary64 test uses independent literal binary32 wire expectations to check
rounding and signed zero in both layouts without changing the stored parameters.
Actual optimized assert-enabled C and Python callers each exercise all ten new
exports, both precisions and both original formats with exact bytes and positions.

All five targets pass 50 suites /162 tests, zero failed/ignored, retaining both
expected E0502 compile-fail gates. All nine C callers pass on all five targets;
all nine Python callers pass on all three 64bit targets. Host C/Python callers
also verify the shared byte/I/O helpers with serialization disabled, and Python
confirms all ten Gaussian codec functions and decode defaults are absent.
Enabled/disabled Clippy, Rustdoc with warnings denied, formatting and cbindgen
verification pass. The header has 167 exports, 19 descriptors and 13 owners.
All 21 optional codec functions and both encoding selectors are feature-guarded.
Native sources, Cargo manifests/lock and existing fixtures are unchanged.

The inventory now has 31 implemented C source mappings and 72 pending. Stream/
model owners/builders/preparation/statistics/codecs, datasets/topology/inference/
training/generation/pools and remaining configuration bindings remain required.
All WASM and final combined acceptance remain pending. Debug shared-library proof
does not establish Release/static-link/Python32 acceptance. Full scope is unchanged.

## Model sufficient statistics and maximization

Twenty-four functions expose model-statistics create/replace/from-model/layout,
stream and duration child get/set, clone/clear/release and model update in both
precisions. The existing model-layout descriptor carries both child counts.
Construction and replacement prevalidate borrowed arrays and copy every nested
stream accumulator and duration accumulator. Repeated children are permitted;
getters and clones return independent owners. Editing retains arbitrary scalar
bits until native update validation. Complete replacement can resize either
child family, including empty or single-family collections.

From-model construction and clear delegate the existing native statistics
implementation. No model borrow survives construction. Stream children expose
all Gaussian accumulator fields through the existing stream/Gaussian-statistics
interfaces; duration children expose value sum, squared sum and occupancy.
Release frees nested storage and clears its initialized ownership slot.
Invalid inputs preserve existing owners and output storage.

Model update accepts the complete typed update settings and delegates the native
transactional implementation. It adds no outer model clone. A final duration
overflow after earlier staged stream and duration updates retains every model
parameter. Statistics remain unchanged, and replacing the invalid child permits
the same model to update successfully. Native update kernels, statistics,
serialization, manifests/lock and fixtures are unchanged. Existing duration C
statistics only gain sibling-module field visibility.

Four boundary tests verify independent copies, repeated children, child edits,
source release, arbitrary duration scalar bits, clear, replacement, invalid
inputs and both successful and failed updates. They check all parameter fields
of a complete stream-plus-two-duration model around a late update failure.
Both precisions also compare 90 original C Gaussian and duration update values
through whole-model updates, including low occupancy, variance floors and
geometric duration updates. Optimized assert-enabled C and Python callers each
invoke all 24 new exports and check ownership, replacement, release and late
duration update failure. These callers do not independently repeat the 90-value
corpus; that comparison is performed by the Rust boundary tests.

All five native targets pass 57 suites /181 tests with zero failed/ignored and
both expected E0502 compile-fail gates. All 15 C callers pass on all five targets;
all 15 Python callers pass on the three 64bit targets. Host model-statistics
callers also pass with serialization disabled. Enabled/disabled Clippy and
Rustdoc with warnings denied, formatting and cbindgen verification pass.
The header has 315 functions, 23 descriptors and 25 opaque owners.

The inventory has 47 implemented C source mappings and 56 pending. The four
model-statistics/update rows are complete. Stream/model owned preparation,
data/topology/inference/training/generation/pool/configuration bindings, all
WASM and final combined acceptance remain required. Debug shared-library proof
does not establish Release/static-link/Python32 acceptance. Full public scope
is unchanged.

## Stream parameters and incremental construction

Both precisions expose nine parameter-owner and ten incremental-builder functions,
38 exports in total. Parameter `new` delegates native flat-start construction.
`create` and `replace` copy the complete weight and borrowed Gaussian-owner array,
including repeated inputs and different component counts/dimensions per emission.
Layout, weight edits, child get/set, deep clone and release expose every stream
parameter. Child getters return independent editable owners; setters retain inputs
and copy every Gaussian field. Complete replacement can change the emission count.
Arbitrary scalar bits and zero variances remain editable storage.

Builders expose weight/count, slot presence, child get/set/clear, clone, build and
release. An unfilled slot returns status three when read or built; an invalid index
returns two. Native validation builds a cloned snapshot, retaining the source builder
on success and failure. All returned streams and children remain independent after
inputs, builders or other returned copies are edited or released. Empty arrays permit
null and empty streams retain native behavior. Errors preserve output slots, existing
parameters and populated builder slots. Release clears its initialized ownership slot
and a null owned value is a no-op. No internal child pointers escape.

Two boundary tests compare every parameter bit against native values in both
precisions, including NaN payloads, infinity and signed zero. They cover all 38
exports, mixed shapes, repeated inputs, shape changes, unfilled slots, independent
copies, source edits/release, empty containers, invalid indices/counts/pointers and
unchanged outputs/owners on failure. Optimized assert-enabled C and Python callers
exercise every new export through the generated header and actual shared libraries.

All five native targets pass 51 suites /164 tests, zero failed/ignored, preserving
both expected E0502 compile-fail gates and the original 120 native tests. All ten
C callers pass on all five targets; all ten Python callers pass on all three 64bit
targets. Host stream callers also pass with serialization disabled. Enabled/disabled
Clippy and Rustdoc with warnings denied, formatting and cbindgen verification pass.
The header contains 205 functions, 21 descriptors and 17 opaque owners. Native
sources, Cargo manifests/lock and existing fixtures remain unchanged.

The inventory has 33 implemented C source mappings and 70 pending. Only the two
stream constructor rows are completed here. Editable stream/builder release is
verified; the full deletion mapping remains pending while owned preparation is
completed. Stream preparation/statistics/codecs, model owners/builders and remaining
data/topology/inference/training/generation/pool/configuration bindings, all WASM
and final combined acceptance remain required. This is Debug shared-library proof;
Release/static-link/Python32 acceptance is not established. Full scope is unchanged.

## Model parameters and incremental construction

Both precisions add ten model-owner and thirteen model-builder functions, 46
exports in total. `LiblrhsmmRsStreamSpecification` contains all three native
flat-start stream dimensions. Model `new` copies the specifications using checked
allocation and delegates native construction of every stream and default duration.
`LiblrhsmmRsModelLayout` returns both independent child counts. Complete create/
replace operations copy borrowed stream/duration-owner arrays, permitting repeated
inputs and arbitrary valid nested Gaussian shapes. All stream/Gaussian parameters
and all six duration fields remain editable without exposing internal pointers.

Getters return independent child copies; setters retain and deep-copy the input.
Whole-model clones own separate nested storage. Complete replacement supports
changes to either child count and stages publication until all copying/validation
succeeds. Empty arrays permit null; stream-only, duration-only and completely empty
models retain native behavior. Arbitrary scalar bits and flat-start values remain
parameters until native preparation or computation validates them.

Builders expose both slot counts, presence/get/set/clear operations for streams
and durations, deep clone, build and release. Invalid indices return two; unfilled
get/build returns three. Native build validates a cloned snapshot and keeps the
source builder on success or failure. Returned models remain independent after
builders, source children or other returned copies are edited or released. Failure
preserves output slots and existing stored parameters; release clears initialized
ownership slots and accepts null owned values.

Two boundary tests exercise all 46 functions and compare every nested parameter
bit against native values in both precisions, including NaN payloads, infinity and
signed zero. Binary32 flat-start construction also matches the original C model
fixture field-for-field. Tests cover mixed emission shapes, repeated children,
resizing, independent get/set/clone/build, both types of unfilled slots, source
edits/release and invalid pointer/index/count/layout cases without changing outputs
or owners. Optimized assert-enabled C and Python callers each invoke every new
export through actual shared libraries and verify representative child fields,
ownership independence, builder errors and empty/one-family models.

All five targets pass 52 suites /166 tests, zero failed/ignored, retaining the
original 120 native tests and both expected E0502 compile-fail gates. All eleven C
callers pass on all five targets; all eleven Python callers pass on all three 64bit
targets. Host model callers also pass with serialization disabled. Enabled/disabled
Clippy and Rustdoc with warnings denied, formatting and cbindgen verification pass.
The header has 251 functions, 23 descriptors and 21 opaque owners. Native sources,
Cargo manifests/lock and existing fixtures remain unchanged.

The inventory has 35 implemented C mappings and 68 pending. Only the two model
constructor rows are completed here. Editable model/builder release is verified;
the whole-model deletion mapping still needs owned preparation/cache release.
Complete stream/model preparation/statistics/codecs, data/topology/inference/
training/generation/pool/configuration bindings, all WASM and final combined
acceptance remain required. Debug shared-library proof does not establish Release,
static-link or Python32 acceptance. Full public scope is unchanged.

## Stream and whole-model serialization

Twenty optional codec functions expose stream/model `read_bytes`, strict
`read_stream`, one-object `read_stream_prefix`, `write_bytes` and `write_stream`
in both precisions. They delegate the existing native codecs and share the same
encoding selectors, decode limits, byte ownership and borrowed callback adapters
as Gaussian serialization. Existing Gaussian behavior is unchanged; only the
private encoding-selection helper is shared with the two new modules.

Strict inputs reject trailing data; prefix readers consume one object without
probing EOF. Short transfers and interruption retry follow native Read/Write
behavior. Missing callbacks, I/O errors, overcounts and write-zero return errors;
output-owner slots are retained. Unknown encoding selectors and unrepresentable
historical variance floors are rejected before writer callbacks. No implicit
flush/close/context retention occurs. I/O failures may consume or emit a prefix.
Stored parameters are never changed by encoding, including encoding failures.

`tests/extract_historical_stream.py` copies the original pretrained model bytes
[2:28391] without re-encoding or normalizing values. The 28,389-byte stream fixture
SHA-256 is `f99c72e6ca0add29f6eb8861a9284f18d71e6e5f032208e3024a5c48851cf486`.
A native test verifies the exact source span, every decoded stream field and
byte-for-byte historical rewriting in both precisions. Existing fixtures remain
unchanged. Whole-model callers use the full original current-C model (127,957
bytes) and bundled historical model (89,257 bytes), without reducing model shape.

Exact cumulative array-entry budgets are 33 for the current standalone stream,
6,452 for the historical stream, 28,606 for the current full model and 20,866 for
the historical full model. Each exact budget succeeds and one below fails.
Every truncation is checked for the small stream; large inputs check header,
interior and tail positions to avoid quadratic exhaustive decoding. Whole-file
round trips and all decoded parameter bits are still checked completely.

Five boundary tests cover all twenty functions, all stream/Gaussian/duration
fields, both precisions/schemas, exact wire bytes, two consecutive objects plus a
sentinel, resource limits, callback failures and unchanged output slots. A model
test explicitly inserts a nonzero floor and verifies historical writes preserve
every model field and an existing byte owner without invoking the writer. Actual
optimized assert-enabled C/Python callers invoke all twenty functions, both full
models and stream fixtures, and verify exact bytes/positions/budgets and floor-loss
failure atomicity. Large C test buffers use static storage to avoid i686 stack limits.

All five targets pass 55 suites /172 tests, zero failed/ignored, retaining the
original 120 native tests and both expected E0502 compile-fail gates. All thirteen
C callers pass on all five targets; all thirteen Python callers pass on all three
64bit targets. Host feature-disabled C/Python callers verify shared byte/I/O helpers
and absence of all twenty new functions and decode defaults. Enabled/disabled
Clippy and Rustdoc with warnings denied, formatting and cbindgen verification pass.
The header has 271 functions, 23 descriptors and 21 opaque owners; all 41 optional
codec functions remain guarded. Native sources, manifests/lock and existing codec
implementations/fixtures remain unchanged.

The inventory has 39 implemented C source mappings and 64 pending. The four stream/
model reader/writer rows are complete. Stream/model owned preparation/statistics/
updates, data/topology/inference/training/generation/pool/configuration bindings,
all WASM and final combined native/binding acceptance remain required. Debug shared-
library proof does not establish Release/static-link/Python32 acceptance. Full public
scope is unchanged.

## Stream sufficient statistics and maximization

Twenty functions expose stream-statistics create/replace/from-stream/count/child
get/set/clone/clear/release and stream update in both precisions. Input child-owner
arrays are prevalidated and deeply copied, permitting repeated inputs and differing
Gaussian shapes. Each child retains dimensions, component count, state occupancy,
weighted sums, squared sums and component occupancies through the existing Gaussian-
statistics interface. Arbitrary scalar bits remain editable until native update
validation. Getters return independent owners; complete replacement can resize the
emission list and setters can change one child shape.

From-stream construction creates complete native zeroed accumulators without
retaining the model. Clear delegates native zeroing and keeps all shapes. Clones,
returned children and input copies survive source edits/release independently.
Invalid input arrays/indices/output pointers retain existing owners and outputs.
Empty arrays permit null; release clears its initialized slot and accepts null
owned values. No internal vector pointer is exported.

Stream update uses every existing typed update setting and delegates native
transactional maximization without an extra outer clone. All stream parameters
are retained if a later emission fails after earlier staged emission updates.
Valid low-occupancy/floor behavior is unchanged. Native stream update now validates
variance-floor options before its emission loop, fixing the empty-stream path
that previously skipped validation entirely. A native test rejects negative/NaN/
infinite values in either floor setting in both precisions, retains the empty
stream and confirms valid defaults still succeed. Native update source changes
by exactly that one validation call; numerical update kernels are unchanged.

Four boundary tests cover all twenty functions, every accumulator vector/shape/
occupancy bit including NaN payloads/signed zero/infinity, mixed shapes/repeated
children, clone/get/set independence, source edits/release, complete replacement,
native clear, invalid inputs, empty-option validation, successful updates and
failure in a later emission preserving every parameter field. Both precisions
also check 70 original C update values, including low occupancy and variance floors.
Optimized assert-enabled C/Python callers each invoke all twenty functions and
verify the same 70 original update values plus ownership/clear/error/empty cases.

All five native targets pass 56 suites /177 tests, zero failed/ignored, retaining
the original 120 native tests and both expected E0502 compile-fail gates. All
fourteen C callers pass on all five targets; all fourteen Python callers pass on
all three 64bit targets. Host stream-statistics callers also pass with serialization
disabled. Enabled/disabled Clippy and Rustdoc with warnings denied, formatting
and cbindgen verification pass. The header has 291 functions, 23 descriptors and
23 opaque owners; existing optional codecs remain guarded. Native statistics,
formats, manifests/lock and fixtures remain unchanged. Existing Gaussian C
statistics only gain sibling-module field visibility; their functions are unchanged.

The inventory has 43 implemented C source mappings and 60 pending. Four stream
statistics/update rows are complete. Model statistics/update, stream/model owned
preparation, data/topology/inference/training/generation/pool/configuration bindings,
all WASM and final combined acceptance remain required. Debug shared-library proof
does not establish Release/static-link/Python32 acceptance. Full scope is unchanged.

## Prepared stream and model snapshots

Thirty-six functions expose immutable stream/model preparations in both precisions.
Stream functions prepare, copy parameters, report layout, copy a prepared Gaussian,
clone and release. Model functions prepare all/selected durations, copy the model
or a stream, report layouts/options/duration availability, copy prepared Gaussian/
duration children, clone and release. The existing preparation descriptor supplies
all three native settings: duration cache length, duration weight and temperature.

Native owned snapshots retain complete parameters and matching caches without a
source borrow. Returned children copy existing derived state without recomputation.
All Gaussian/duration parameters, cached values and density operations remain
accessible through their existing child APIs. Source edits/release and parent
release cannot invalidate snapshots, clones or returned children. Checked layouts
and parameter copies expose stream weights and every nested shape/parameter field.

Selected preparation permits duplicate indices and empty selections; unused
uninitialized durations remain stored without density evaluation. Availability
returns zero for an unselected in-range duration; retrieving it returns status
three. Out-of-range indices return status two. Errors retain output storage.
Release clears initialized ownership slots and accepts null owned values.

Four boundary tests cover both precisions. Stream tests compare 166 original C
Gaussian cache/density values through the prepared-stream child interface. Model
tests compare complete parameter bits, including binary64 values beyond binary32
precision and signed-zero floors, every cache, settings, selected/uninitialized
states, cache fallback, source edits/release, copied children and errors.
Optimized assert-enabled C/Python callers invoke all 36 new functions and verify
layouts, options, copied children, cache lifetimes, selected states and errors.
The 166-value comparison is performed by Rust boundary tests; the new external
callers do not independently repeat that corpus.

All five targets pass 60 suites /190 tests, zero failed/ignored, retaining all
previous tests and both expected E0502 compile-fail borrow gates. All 16 C callers
pass on every target; all 16 Python callers pass on the three 64bit targets. Host
new callers and boundary tests also pass with serialization disabled. Enabled/
disabled Clippy and Rustdoc with warnings denied, formatting and cbindgen verification
pass. The header has 351 exports, 23 descriptors and 29 opaque owners. Existing
prepared Gaussian/duration fields gain sibling visibility only; their functions,
native sources, fixtures and manifests/lock remain unchanged.

The inventory has 50 implemented C source mappings and 53 pending. Whole-model
precompute and model/stream parameter/preparation deletion mappings are complete.
Data/topology/inference/posteriors/initialization/training/generation/pool/configuration
bindings, all WASM and final combined acceptance remain required. Owned-model
inference bindings remain pending. Full public scope is unchanged. This proof
establishes Debug shared-library behavior, not Release/static-link/Python32 acceptance.
