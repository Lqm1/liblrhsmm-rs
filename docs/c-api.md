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

The current header exports 591 functions, 39 descriptors and 49 opaque owners,
with ten table/mode/callback selectors. Codec exports and the decoder-limit
descriptor require the `serialization` feature. Of the 103 original functions in
`binding-inventory.csv`, 72 have verified complete C mappings and 31 remain
pending. Duration/Gaussian/stream/model parameter storage, preparation, statistics,
updates and codecs have verified interfaces, as do complete observation storage
and codecs, and complete editable segmentation parameters and resegmentation.
Independent incoming-cache ownership and copying, typed inference-table storage,
and forward/backward likelihood totals are verified as well.
Geometric and HSMM forward/backward/Viterbi calls now have complete score/path
results, including model-to-log/density emission sampling with every pruning
mode. Segmentation codecs, sets/datasets,
posteriors, initialization, generation and memory-pool interfaces remain pending.
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

## Observation stream storage

Twenty functions expose independently owned time-major observation streams in
both precisions: native zeroed construction, complete create/replace, layout,
scalar-range copy/set, complete-frame copy/set, clone and release. The shared
layout reports dimensions, frame count and scalar count. Positive dimensions
and complete frames must fit the native legacy limits; zero frames are supported.
Scalar arrays use address-space bounds rather than restricting their product
length to a single legacy frame count. Arbitrary scalar bits remain editable.

Input arrays are copied and never retained. Full replacement stages the new
shape/storage before publication. Range/frame writes validate the complete
operation before mutation. Errors retain owners and output buffers. Empty
scalar ranges permit null buffers at valid offsets. Clones are independent,
and release clears initialized ownership slots. Frame access requires an exact
dimension-sized buffer. No internal vector pointer is exposed.

Two boundary tests verify every scalar bit, including NaN payloads, signed zero,
infinities, subnormals and binary64 values beyond binary32 precision; zeroed
construction, shape replacement, independent input/clone storage, frame/range
access, empty frames and invalid shapes/pointers/indices/overflow. With
serialization enabled, every feature bit in the original C observation fixture
survives create/copy unchanged. Assert-enabled C/Python callers invoke all 20
functions and independently verify exact scalar bits and ownership/error behavior.

All five targets pass 61 suites /192 tests, zero failed/ignored, retaining all
previous tests and both expected E0502 borrow gates. All 17 C callers pass every
target; all 17 Python callers pass the three 64bit targets. New boundary tests
and host callers also pass with serialization disabled. Enabled/disabled Clippy
and Rustdoc with warnings denied, formatting and cbindgen verification pass.
The header has 371 exports, 24 descriptors and 31 opaque owners. Native sources,
existing C implementations, formats, fixtures and manifest/lock remain unchanged.

The original C source inventory remains 50 implemented /53 pending. Complete
observation ownership/building/copy/deletion/codecs, segmentation and all remaining
data/inference/training/generation/pool/configuration bindings still require
implementation and proof. A standalone stream interface does not complete those
source rows. All WASM and final combined acceptance remain pending.

Primary Rust unwind-boundary references for the generic scalar-copy helper:

- https://doc.rust-lang.org/stable/std/panic/fn.catch_unwind.html
- https://doc.rust-lang.org/stable/std/panic/trait.UnwindSafe.html
- https://doc.rust-lang.org/stable/std/panic/trait.RefUnwindSafe.html

## Complete observation ownership and feature access

Twenty-six additional functions, thirteen per precision, expose complete
observations: native zeroed construction with per-stream dimensions, construction
and complete replacement from borrowed stream owners, observation/stream layouts,
independent stream getters/setters, scalar-range and complete-frame copies/updates,
deep clone and release. The shared frame count must match every stream; stream
dimensions may differ and may change through a validated replacement. Repeated
borrowed child owners produce independent storage. Zero frames are supported;
empty stream collections are rejected by native structural validation.

Every field in `lrh_observ` and the `lrh_obm` indexing functionality has a checked
counterpart. Scalar range operations preserve arbitrary feature bits, validate
address-space bounds and check the entire interval before copying. Frame operations
require exactly one dimension-sized vector and use native checked frame access.
Failed shape replacement leaves the complete original observation intact. Borrowed
children and returned copies remain independent after either owner is edited or
released. No internal vector pointer is exported. Empty scalar ranges permit null
buffers because neither raw-slice construction nor pointer copying is performed.

Two boundary tests verify both precisions with different stream dimensions, NaN
payloads, signed zero, infinities, subnormals, binary64 values beyond binary32
precision, all layouts, every access/update operation, complete reshaping,
repeated child inputs, source/parent release, empty frames and error atomicity.
The unchanged original C observation fixture passes through copied stream owners
into a whole observation, survives child release, and retains every feature bit.
Optimized assert-enabled C and Python callers invoke all 26 functions and verify
scalar bits, full shapes, ownership, frame access and failed-operation behavior.
The external callers do not independently repeat the original wire fixture;
original-C fixture comparison is performed by the Rust boundary tests.

All five native targets pass 62 suites /194 tests, zero failed/ignored, retaining
all previous tests and both expected E0502 borrow gates. All 18 C caller families
pass every target; all 18 Python families pass the three 64-bit targets. New host
boundary tests and C/Python callers also pass without serialization. Clippy and
Rustdoc with warnings denied pass with serialization enabled/disabled, as do
formatting, whitespace and cbindgen verification. Native sources, existing C
implementations, fixtures, formats and manifest/lock remain unchanged.

Observation construction/deletion source rows are complete. The C source
inventory is 52 implemented /51 pending. Observation/segmentation codecs,
segmentation/topology, sets/datasets, inference/posteriors/initialization,
remaining training/generation/pool/configuration interfaces, all WASM and final
combined acceptance remain required. This checkpoint establishes Debug shared
library behavior, not Release/static-link/Python32 acceptance.

Primary references checked through Context7 and Mintlify:

- https://doc.rust-lang.org/stable/std/ptr/fn.copy_nonoverlapping.html
- https://doc.rust-lang.org/stable/std/slice/fn.from_raw_parts.html

The copy operation preserves bits and requires independent aligned storage even
for an empty copy; therefore the boundary skips it for zero counts. Empty borrowed
owner arrays use an empty Rust slice instead of constructing a null raw slice.

## Observation byte and callback codecs

Ten optional functions expose the unchanged observation schema in both storage
precisions. Byte and callback strict readers reject trailing data and use the
caller-selected cumulative array-entry budget. Callback prefix readers consume
exactly one original C object and leave subsequent bytes unread without an EOF
check. Byte writers return an independent byte owner; callback writers support
partial transfers through the existing shared I/O adapter. No callback, context
or caller buffer is retained. Failed reads preserve initialized output slots.
Writes do not flush or close the stream; a write error can leave an emitted prefix.

Observation wire features are binary32 in both storage precisions. There is one
observation schema and no model-encoding selector. All parsing and writing use the
native serializer. The native `ObservationData::read_with_limits` method adds a
strict budgeted entry point; existing `read_from` delegates with default limits.
The existing decoder, structural validation and EOF check remain unchanged.

Two native tests check both precisions, exact budget 16 for the original C fixture,
all lower budgets, every truncated prefix, unchanged wire bytes, sequential object
boundaries and propagation of an EOF-check I/O error. Two Rust boundary tests
compare every decoded field/feature bit and exact encoded bytes, exercise
one-byte transfers, interruption retries, two sequential objects and guarded
prefix EOF behavior, missing callbacks, oversized callback counts, zero writes,
invalid pointers/limits/shapes and preservation of outputs on failure. A writer
that fails after seven bytes retains that exact emitted prefix and the source
observation unchanged. Optimized assert-enabled C and Python callers invoke all
ten new functions and independently verify original wire bytes, one-byte callbacks,
sequential boundaries, truncation, resource limits and I/O errors. The guarded
EOF and seven-byte late-failure tests are Rust boundary evidence.

All five native targets pass 64 suites /198 tests, zero failed/ignored, preserving
all previous tests and both expected E0502 borrow gates. All 19 C caller families
pass every target; all 19 Python families pass the three 64-bit targets. Native
limits tests also pass without the C API enabled. Serialization-disabled C/Python
callers verify shared byte/I/O functionality and absence of the new optional codec
functions. Enabled/disabled Clippy and Rustdoc with warnings denied, formatting,
whitespace and cbindgen verification pass. Native source changes are restricted
to the strict budget wrapper; formats, fixtures, numerical kernels, existing
C implementations and manifest/lock remain unchanged.

The source observation read/write rows are complete. C inventory is 54 implemented
/49 pending. Observation sets, segmentation/topology, datasets, inference/
posteriors/initialization, remaining training/generation/pool/configuration
interfaces, all WASM and final combined acceptance remain required. Evidence
establishes Debug shared libraries, not Release/static-link/Python32 acceptance.

Primary I/O references checked through Context7 and Mintlify:

- https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_exact
- https://doc.rust-lang.org/stable/std/io/trait.Write.html#method.write_all
- https://doc.rust-lang.org/stable/std/io/trait.Write.html#tymethod.flush

## Complete segmentation parameter ownership and transformations

Seventeen unconditional exports per precision provide zeroed construction, full
construction/replacement, layout, interval copies and setters for boundaries,
duration states and stream-wise output states, outgoing group counts/copies/
replacement, native jump insertion, resegmentation, deep parameter cloning and
release. Two opaque owners retain independent binary32/binary64 parameter storage.
Six new descriptors retain layout, state occurrences, selected-precision jumps
and synchronous borrowed jump slices. No internal storage pointer escapes.

Full inputs use an array of stream-wise state pointers, each with the segment
count entries. Repeated pointers are permitted and become independent copied
rows. Ragged outgoing groups retain every delta and probability bit. Construction
checks all borrowed ranges, reserves copied storage and validates complete native
shape before publishing an owner. Replacement stages the entire new value before
mutation. Shape validation requires exactly one final delta-1 edge per outgoing
group and accepts arbitrary stored integer labels and scalar bits. It does not
apply the inference builder's finite/nonnegative weight rules to raw storage.
The original native shape validation condition was moved unchanged to a shared
helper, used by the C outgoing setter. Other native implementations are unchanged.

Interval errors leave caller buffers and parameters unchanged. Outgoing copies
map descriptor fields individually, independent of struct padding. Empty arrays
permit null. Jump insertion uses the existing native method, retaining its source
ordering and checks; ordinary delta1 updates the final weight. Resegmentation
copies labels by explicit state occurrence, keeps exclusive right boundaries
including equal adjacent boundaries, and resets outgoing ordinary edges using
the original native implementation. Source and resulting owners are independent.
This API does not invent standalone start/end fields absent from upstream data.h.

Four Rust boundary tests cover both precisions, every field, NaN payloads,
subnormals, signed zero, infinities, f64 precision beyond f32, source mutation/drop,
independent repeated rows/clones, shape changes, empty data, native jump edits,
resegmentation and invalid/overflow/misaligned ranges. The unchanged C segmentation
and copy fixtures retain every integer and probability bit through create/clone/
copy/release. Native sample resegmentation through the C boundary matches every
field in the unchanged original C resegmentation fixture. This is parameter-field
evidence, not a claim that segmentation wire codecs have C bindings yet.

Actual optimized assert-enabled C and ctypes Python callers invoke every one of
the 34 exports. They check exact scalar fields without comparing struct padding,
repeated-row independence, source changes, clone lifetime, whole-value failed
replacement, failed result publication, invalid ranges and empty construction.
All five targets pass 66 suites /207 tests with zero failed/ignored. All20 C caller
families pass all five targets and all20 Python families pass the three64bit
targets. Both previous E0502 borrow gates and the E0505 graph ownership gate remain
verified. The two non-codec segmentation tests and both actual callers also pass
without serialization. Enabled/disabled Clippy and Rustdoc deny warnings;
formatting, whitespace and generated header verification pass.

Source create/delete/shuffle rows now have complete parameter/operation mappings.
The upstream lrh_seg_copy function additionally copies djump_in/pjump_in caches.
Although the new segmentation clone copies all parameters and outgoing groups,
lrh_seg_copy remains pending until C graph owners can independently copy incoming
caches as well. lrh_seg_buildjumps also remains pending. The inventory is
57 implemented /46 pending; all WASM mappings remain pending. Graph/cache C
ownership and inference integration, segmentation codecs, sets/datasets,
remaining inference/training/generation/pool/configuration bindings and final
combined acceptance remain required. Evidence establishes Debug shared libraries,
not Release/static-link/Python32 acceptance.

The established primary references remain applicable:

- https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html
- https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
- https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact

## Independent transition graph owners and complete cached copying

Eighteen unconditional exports, nine per precision, add independent immutable
transition graph owners. Building from a borrowed segmentation copies every
parameter field and invokes the unchanged native incoming-jump builder once.
Source edit/drop cannot invalidate the graph. Layout exposes complete stream/
segment counts. Incoming/outgoing group counts and checked interval copies retain
every delta and selected-precision probability field, native ordering and final
ordinary-edge sentinels. Invalid intervals preserve output buffers. Empty copy
intervals permit null storage, including the end of a valid group.

Graph clones independently copy complete parameters, outgoing groups and the
already-derived incoming cache. They never invoke graph construction. Reads use
the existing immutable graph storage directly without cloning or recomputing the
cache. get_segmentation produces an independent editable parameter owner; edits
or release of that child do not affect the graph. The child's result does not
include cached incoming jumps; copy the graph itself to retain those. Unique graph
release clears the caller's ownership slot, and null ownership is a no-op. Every
operation retains the declared precision and no internal storage pointer escapes.

The original native graph constructor retains its finite/nonnegative transition
weight checks, exclusion of out-of-range destinations and source ordering. Raw
segmentation storage continues accepting arbitrary stored scalar bits. Failed
graph construction leaves initialized output slots untouched. No native algorithm,
serializer, parameter codec, fixture, dependency or numerical tolerance changed.
The C module reuses the existing OwnedTransitionGraphParameters implementation.
It adds two opaque owners, no new descriptors or selectors. Header totals are
459 functions /31 descriptors /37 owners.

Two Rust tests check both precisions, all segmentation metadata and incoming/
outgoing groups, source/child changes and release, independent cached clones,
parent release, individual and complete intervals, null/misaligned/overflow ranges,
invalid graph weights, unchanged failed outputs and empty graphs with zero or two
streams. Signed-zero weights, extreme ignored deltas, a binary32 subnormal and
binary64 precision beyond binary32 retain their bits through construction and
clone. The unchanged16-record binary32 C transition corpus is compared directly
in f32 and by exact widening in f64. This is not independent C double evidence.

Actual optimized assert-enabled C and ctypes Python callers invoke all18 exports
and compare all16 original C transition records in both precisions with the same
explicit widening rule. They inspect complete metadata, outgoing groups and
independent children, edit/release the source, copy already-built graphs, release
parents, check late invalid-weight construction retains its output sentinel and
cover empty graphs and failed intervals. Scalar fields are compared independently
of descriptor padding. All five native targets pass67 suites /209 tests, zero
failed/ignored, retaining all previous tests and the two E0502 plus one E0505
borrow gates. All21 C caller families pass all five targets; all21 Python families
pass the three64bit targets. The two new Rust tests and both actual callers also
pass without serialization. Enabled/disabled Clippy and Rustdoc deny warnings;
formatting, whitespace and generated-header verification pass.

Complete graph/cache C copying now covers the incoming fields copied by upstream
lrh_seg_copy. Its source mapping and lrh_seg_buildjumps are complete, together with
the previously verified segmentation parameter owners. Current inventory is
59 implemented /44 pending. All WASM mappings remain pending. Existing inference
still builds local graphs; reuse through TransitionGraphView remains required.
Segmentation codecs, sample sets/datasets, remaining inference/posterior/training/
generation/pool/configuration interfaces, all WASM families and final combined
acceptance remain required. Evidence establishes Debug shared libraries, not
Release/static-link/Python32 acceptance.

Established primary references for owned copies and graph views:

- https://doc.rust-lang.org/std/clone/trait.Clone.html
- https://doc.rust-lang.org/std/marker/trait.Copy.html
- https://doc.rust-lang.org/std/primitive.slice.html
- https://doc.rust-lang.org/std/boxed/struct.Box.html

## Native inference integration with immutable graph caches

ModelParameters and borrowed PreparedModelParameters now expose forward_hsmm_with_graph,
backward_hsmm_with_graph and viterbi_hsmm_with_graph. ModelParameters also exposes
duration_occupancies_with_graph. These accept TransitionGraphView borrowed from
either the original graph or an independent owned graph. Internal selection
chooses the supplied view directly; it does not rebuild or clone incoming jumps.
The original public methods still construct a graph at exactly their previous
position in validation/calculation. Their original signatures remain available.

OwnedPreparedModelParameters now supports both the original segmentation-based
and graph-view-based forward, backward and Viterbi entry points. A reference-only
duration cache selection delegates options and duration lookup to the existing
borrowed prepared distribution or owned prepared distribution view. It does not
copy model parameters, copy caches or run preparation again. The existing owned
model interface has per-distribution views; this change does not invent a full
model view. Missing selected duration caches and incompatible preparation options
retain the existing errors. Short/empty duration caches retain their fallback.

Source comparison verifies exact unchanged search, backward, Viterbi trace and
occupancy recurrences. Duration preparation retains its previous validation and
numerical implementation; only cache lookup dispatch changed to call the existing
borrowed or owned implementation. Legacy occupancy still validates scores, total
and preparation before graph construction. Legacy forward/backward/Viterbi still
validate topology first. Both error orders have executable evidence. Existing
graph construction and every graph getter remain byte-for-byte unchanged apart
from the new internal reference-only input selection.

Four tests in tests/graph_inference.rs check both precisions, all scores/path
metadata/duration posterior bits, borrowed and owned graph inputs, repeated cache
use, independent source mutation/drop, raw and both prepared model kinds, zero
and nonzero temperature, fractional pruning radii and empty/short/long duration
caches. They check invalid weights/options, mismatched score dimensions,
nonfinite total, missing model duration states and unprepared selected states.
Independent C comparisons retain the unchanged float, double and fractional
double HSMM corpora, 768 score comparisons each and every path occurrence.
Float matches bits; double retains the existing 2e-14 normalized bound. All192
records in the existing corrected-C duration posterior corpus retain slice
presence, minimum, length and every probability bit. No fixture or tolerance
changed. Cross-path Rust comparisons require bit identity in both precisions.

All five native targets pass 68 suites /213 tests, zero failed/ignored, retaining
all previous tests and the two E0502 plus one E0505 borrow gates. All21 existing
C caller families pass all five targets; all21 Python caller families pass the
three64bit targets. New tests also pass with no C API, without serialization,
and with C API but no serialization. Enabled/disabled Clippy and Rustdoc deny
warnings; formatting, whitespace and unchanged generated header verification pass.

The C header remains459 exports /31 descriptors /37 owners. Inventories and
manifests are unchanged, C59 implemented /44 pending and every WASM mapping pending.
This checkpoint establishes native graph/cache reuse, including owned prepared
models. The C inference interfaces still need to expose these paths together with
complete score, path and posterior containers. Segmentation codecs, sample sets/
datasets, remaining C/WASM families and final combined acceptance remain required.
Current evidence establishes Debug shared libraries, not Release/static/Python32.

Primary Copy/Clone guidance checked through Context7 and Mintlify applies to the
reference-only duration selector, with manual implementations avoiding extra
generic bounds:

- https://doc.rust-lang.org/stable/std/marker/trait.Copy.html
- https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
- https://doc.rust-lang.org/reference/special-types-and-traits.html

## Typed inference tables and likelihood totals

The C boundary adds52 unconditional exports,26 per precision, and six independent
owners for log emissions, emission densities and forward/backward state scores.
The layout descriptor exposes frames, states and the complete time-major scalar
count. Every owner supports checked element/frame/interval copies, deep cloning
and unique release. No pointer into Rust storage escapes. Log-emission and score
owners support complete copied import and staged replacement, including reshaping.
Density owners are produced by the existing native log-to-probability conversion;
they are distinct from log-emission inputs. This does not add model-to-emission
computation or an arbitrary density constructor.

Dimensions must be positive, within the legacy signed dimension limit, and match
the checked product. Flat storage uses address/byte limits, not the signed scalar
count limit. Log imports follow the native finite-value requirement. Score imports
preserve all scalar bits, including NaN payloads and infinities. The only native
addition is StateScoreTable::from_values, which validates positive matching shape.
Existing native calculations, fixtures, formats and dependencies remain unchanged.

The three score totals delegate to native total_log_probability,
total_through_state and backward_log_probability. These cover all four original
lrh_total functions, including the two identical backward-total bodies. Scalar
outputs preserve the selected precision without floating return-register rounding.
Probability conversion retains the existing binary32-compatible exponential in
both precision variants. Failure preserves owner state and initialized outputs.
Zero-length interval copies allow null storage; release clears unique owner slots
and accepts an already-null owner.

Four Rust ABI tests cover all52 exports, shapes, field copies, independent lifetimes,
cloning, valid replacement, late invalid replacement, null/alignment/range failures,
and native probability/total comparisons. Two of these tests also import unchanged
original C geometric forward/backward score tables and check14 total records per
precision,42 total results each. Float retains exact numerical equality; double
retains the existing2e-14 normalized bound. No corpus or tolerance was changed.
Actual optimized C and ctypes callers use all52 exports in both precisions, check
raw scalar bits and ownership, and compose forward-total expectations using the
previously verified log-add interface. Those actual C/Python callers do not load
the original total corpus themselves; that evidence comes from the Rust ABI tests.

All five native targets pass70 suites /218 tests with no failed or ignored tests,
including all existing tests and the three compile-fail borrow gates. All22 C
caller families pass all five targets; all22 Python families pass the three64bit
targets. Native-only score import and graph tests, disabled-serialization Rust/C/
Python callers, Clippy/Rustdoc with warnings denied, formatting, whitespace and
generated-header verification pass. Clippy initially rejected two redundant f64
conversions in the corpus test; those were removed and the full matrix rerun.

The header now has511 exports /32 descriptors /43 opaque owners. C inventory is
63 implemented /40 pending, with every WASM entry still pending. C model inference,
path/posterior owners, segmentation codecs, datasets, training, generation and
remaining interfaces still require implementation. This evidence is Debug/shared;
Release/static linking and Python32 are not established by this checkpoint.

## Complete geometric and HSMM inference results

The new inference boundary adds70 unconditional exports,35 per precision, six
immutable result owners and five descriptors. Per-call geometric and HSMM settings
copy every native field and expose native defaults. Geometric forward/backward
calls return complete score owners; Viterbi returns frame states, segment boundaries,
likelihood and every search score. HSMM calls return both begin/end tables, or a
path with every occurrence, likelihood and both score tables. Occurrences preserve
state repeats, skips and exclusive right boundaries. Complete score-pair import,
deep cloning and independent child copies permit using caller-supplied tables.
All path arrays support checked interval copies; no internal pointer escapes.

Raw and independently prepared models expose all three HSMM operations with either
segmentation parameters or a cached transition graph. Cached calls borrow existing
graph views and duration preparations without cloning or rebuilding those caches.
Geometric calls use the raw native model methods. Scalar outputs avoid i686 return
register rounding. Initialized outputs remain unchanged on failure, complete
intervals are validated before any write, and release consumes unique ownership.
No native algorithm, fixture, format or dependency changed. The only modification
to the previous table boundary is module visibility for its unchanged copy helper.

Four Rust ABI tests cover both precisions, all70 functions, all4 native HSMM sample
scenarios and duration cache lengths0/2/300. Every forward/backward/path score,
likelihood bit and occurrence agrees with the unchanged native calculations.
Score-pair import preserves NaN payloads, signed zero and infinities and rejects
dimension mismatch without publishing a result. Child owners survive releasing
parents; geometric clones remain usable after releasing all inference inputs.

Actual optimized C and ctypes callers compare768 float HSMM score values against
unchanged original C fixtures, and1536 double values against ordinary and fractional
radius corpora. Float uses bit identity; double retains the existing2e-14 normalized
bound. Every original path occurrence is checked separately for each corpus.
All other model/cache paths require score bit identity with the corresponding raw
call. Geometric caller checks cover usable scores, complete monotone states and
boundaries, finite likelihood and independent lifetime; their bit comparisons with
native geometric calculations are in the Rust ABI tests. These callers do not
claim independent original-C geometric or Viterbi-search-table corpus comparisons.

All five native targets pass71 suites /222 tests, no failed/ignored tests, preserving
all previous tests and three compile-fail borrow gates. All23 C caller families
pass all five targets; all23 Python families pass the three64bit targets. Feature
variants, Native-only regressions, no-serialization C/Python inference callers,
Clippy/Rustdoc with warnings denied, formatting, whitespace and generated-header
verification pass. Header581 exports /37 descriptors /49 opaque owners. C inventory
69 implemented /34 pending; every WASM entry remains pending. Settings evidence
was extended while configuration rows remain pending for their remaining uses.
Model emission computation, posterior owners/calls, dataset/training/generation,
segmentation codecs and other public bindings remain required. This checkpoint
establishes Debug/shared execution, not Release/static linking or Python32.

Primary references checked through Context7 and Mintlify:

- https://doc.rust-lang.org/stable/std/boxed/struct.Box.html
- https://doc.rust-lang.org/stable/std/ptr/fn.copy_nonoverlapping.html

## Weighted multistream emission computation

Ten unconditional functions, five per precision, expose the complete native raw
and owned-prepared model emission methods and emission-setting defaults. Two
descriptors retain temperature and the selected precision's radius/slope; three
constants select no pruning, inclusive boundary pruning, or the exclusive linear
band. Unselected radius/slope values are ignored, matching the native enum. The
default descriptor chooses no pruning, with native temperature/radius/slope values.
It is an explicit new descriptor default; original boundary/linear functions map
to the corresponding selected mode rather than to this unpruned default.

Settings use TryFrom to reject unknown modes. Native methods retain validation,
weighted stream calculations, pruning inclusivity, legacy log-zero sentinels and
binary32-compatible non-log conversion in both storage precisions. Prepared calls
directly reuse owned normalizers, even when evaluation temperature differs from
duration preparation. Model, observations and segmentation are borrowed only
during the call. Returned typed table owners remain usable after input release.
Failure preserves initialized outputs and does not mutate the owners. No native
algorithm, fixture, file format, dependency or previous C boundary was changed.

Two Rust ABI tests use both weighted streams, mixed mixture/dimension counts,
reversed stream-state mapping, all three pruning modes and fractional radii.
Raw/prepared logs and densities match every native scalar bit at temperature0,
0.75,1.75 and the next representable value after1.0. Tests check invalid modes,
temperature/band values, missing/misaligned outputs, bad state mapping and lifetime
independence. Each precision additionally checks216 original C log values using
unchanged ordinary/fractional corpora: float bit identity, double's existing
2e-14 normalized bound.

Actual optimized C and ctypes callers use all10 exports and independently compare
the same432 original C log values plus36 original float non-log values. Double
non-log results are compared bitwise with the unchanged native implementation;
this checkpoint does not add an independent original-double non-log corpus.
Prepared/raw caller tables require bit identity in every pruning mode. C header
constants and default descriptors are exercised through these callers; every
default scalar field is additionally compared with native settings in Rust ABI tests.

All five native targets pass72 suites /224 tests, no failed/ignored tests and all
three compile-fail borrow gates retained. All24 C caller families pass all five
targets; all24 Python families pass the three64bit targets. Serialization on/off
Rust/C/Python emission checks, native regressions, codec absence, Clippy/Rustdoc
with warnings denied, formatting, whitespace and generated-header verification
pass. Header591 exports /39 descriptors /49 opaque owners. All three original
sample_outputprob mappings complete: C72 implemented /31 pending; every WASM
entry pending. Three configuration rows gained emission evidence but remain
pending for remaining uses. Posterior/occupancy, dataset/training/generation,
segmentation-codec and other public bindings remain required, with SHIRO C and
all WASM/final combined acceptance still open. Evidence is Debug/shared, not
Release/static linking or Python32.

Official conversion guidance was fetched through Context7 and Mintlify:

- https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html


## Complete posterior storage and computation

Fifty-one additional C exports expose full state, duration, transition and
mixture occupancies. State weights have f32/f64 owners. Duration, transition and
mixture storage remains binary32 in both model precisions, matching the native
and original C slice representations. Three descriptors provide copied slice
inputs, complete slice metadata and optional stay/advance weights. Tags distinguish
absent entries from present empty slices. Duration slices retain their absolute
minimum; mixture components start at zero. Absent input fields are ignored.

Storage constructors and replacements copy all supplied fields and weights before
publication. Invalid tags, shapes, scalar values or slice ranges preserve existing
owners and initialized outputs. Finite nonnegative weights above one, negative
zero and subnormal values are retained without normalization. Empty interval copies
accept null storage. Deep clones and calculation results remain independent after
input release; no internal pointers escape. Checked copies and scalar queries cover
all state frames, duration slices, transition pairs and mixture components.

Both precisions expose native HSMM duration occupancies with segmentation or a
cached transition graph, HSMM state occupancies, geometric state and transition
occupancies, and HSMM/geometric mixture occupancies with both pruning conventions.
Calculations borrow existing native owners and directly invoke native methods.
The corrected HSMM initial-state calculation is retained; outputs of the upstream
bug are not asserted as compatible. No native algorithm, format, fixture or
dependency changed in this checkpoint.

Five Rust ABI tests cover complete storage bits, presence metadata, atomic failed
replacement, independent clones and every native calculation in both precisions.
Mixed weighted streams, dimensions/components, reversed output labels and both
mixture pruning modes are exercised. Computed values match unchanged native bits.
The existing native original/corrected C corpus tests remain in the full matrix,
including all192 corrected-C duration records. These new ABI tests do not add an
independent original-C posterior corpus.

Actual optimized C and ctypes callers use all51 new symbols. They exercise complete
external storage, failed replacements, cloned lifetimes and a multistream
emissions-to-forward/backward-to-posterior pipeline. Likelihoods are obtained via
existing C score methods. Duration outputs from segmentation and cached graphs
match bitwise. Computed slices contain finite nonnegative weights with positive
durations and mixture contributions; retained state results survive input release.
These caller assertions supplement native bit comparisons, not an independent C
numerical oracle. The existing emission ABI tests keep every previous assertion;
their unchanged C input factories now share one native fixture module.

All five targets pass73 suites /229 tests, zero failed/ignored tests and all three
compile-fail borrow gates. All25 C families pass five targets; all25 Python families
pass three64bit targets. Serialization on/off Rust/C/Python checks, native
regressions, codec absence, Clippy/Rustdoc with warnings denied, formatting and
generated-header verification pass. Header642 exports /42 descriptors /54 owners;
ABI version remains1. Six original posterior mappings complete: C78 implemented /
25 pending. Every WASM entry remains pending. Configuration entries remain pending
for training and other remaining uses. Dataset/training/generation/pool bindings,
segmentation codecs, SHIRO C, all WASM and final combined acceptance are still
required. Evidence is Debug/shared, not Release/static or Python32.


## Complete initialization, collection and estimation

Ten additional C exports expose complete initialization collection, HSMM and
geometric posterior collection, and HSMM/geometric expectation steps in both
precisions. They use the existing model-statistics owners, full input model/data
and segmentation owners, posterior owners, and complete inference descriptors.
Every native statistic is accumulated, including shared labels and the native
inclusive HSMM boundary visits or excluded geometric endpoint frames. Estimators
return the native likelihood through a checked scalar output. Output validation
precedes accumulation. Native transactional validation preserves both statistics
and initialized likelihood outputs on failure. Inputs remain caller-owned.

Collectors accept a borrowed array of complete mixture-posterior owners, one per
stream. All owner pointers are checked before copying. The existing native
expectation interface takes a contiguous owned mixture-table slice, so these
external arrays are temporarily deep copied for that interface. Direct native
Rust callers have no new copies. All native calculations, formats, dependencies,
reference corpora and previous C boundaries remain unchanged.

Four Rust ABI tests compare every statistic field and likelihood bit against native
initialization, repeated external posterior collection and both estimators.
Mixed component/dimension counts, weighted streams, reversed labels, fractional
pruning, nondefault duration settings and temperatures including the next value
after1.0 in the selected precision are exercised. Non-normalized external weights,
nonzero prior statistics, missing arrays/late child pointers, invalid counts and
settings, and missing likelihood outputs are covered. Successful statistics remain
usable after inputs are released. Model maximization connects to exact native
encoding, decoding and inference likelihood/path comparisons.

Actual optimized assert-enabled C and ctypes callers use all10 new functions.
For both HSMM and geometric modes they compare the complete statistics and
likelihood bits from manual posterior collection with the automatic estimator,
including two successive accumulations. Flat-start initialization connects to
maximization and inference. Trained models use their corresponding normal or
geometric update mode, then encode, decode and infer again. Native original and
corrected C regression corpora remain in the full matrix. These new ABI/caller
tests add no independent original-C learning corpus.

The unchanged legacy model format stores floating-point fields as binary32 even
for f64 models. Therefore learned f64 values are rounded on serialization. The
caller tests require exact encoded/reencoded bytes, finite decoded inference and
repeatability; f32 additionally retains the pre-save inference likelihood bits.
Rust ABI tests compare decoded inference likelihood bits and every occurrence to
the native model decoded from the same bytes. This preserves the original wire
format without inventing a tolerance or promising lossless f64 persistence.

All five targets pass74 suites /233 tests, zero failed/ignored tests and the three
compile-fail borrow gates. All26 C caller families pass five targets; all26 Python
families pass three64bit targets. Serialization on/off Rust/C/Python checks,
unchanged native regressions, codec absence, Clippy/Rustdoc with warnings denied,
formatting, generated-header and whitespace verification pass. Header652 exports /
42 descriptors /54 owners; ABI version1. Five original learning mappings complete:
C83 implemented /20 pending, every WASM entry pending. Configuration entries retain
pending status for remaining uses. Dataset/generation/pool bindings, segmentation
codecs, SHIRO C, all WASM and final combined acceptance remain required. Evidence
is Debug/shared, not Release/static or Python32.

## Sampling and complete observation generation

The generation API exposes uniform sampling, Gaussian sampling, cumulative
component selection and complete multistream model generation in f32 and f64.
Generation accepts either explicit scalar draws or caller-supplied random words.
The callbacks and context are borrowed for one synchronous call. Return zero
after writing the requested value; any nonzero callback status produces status3.
Callbacks must not unwind, reenter the participating owners or retain output
pointers. They may update their caller-owned context. A failed callback is never
called again during that operation.

`LiblrhsmmRsRandomSource` requires only `next_u32` for f32 and only `next_u64`
for f64; the other callback may be null. The selected callback must be present.
The native OpenClosed01 conversion uses the upper24 or53 bits respectively,
adds one and scales by2^-24 or2^-53. This preserves the native interval(0,1]
without introducing an OS entropy source or a new dependency. The private
adapter implements the pinned rand0.10.3/rand_core0.10.1 `TryRng` with an
infallible error type, retains callback failure and checks it before publishing
any result. See the primary [TryRng documentation](https://docs.rs/rand_core/0.10.1/rand_core/trait.TryRng.html)
and [OpenClosed01 implementation](https://docs.rs/rand/0.10.3/src/rand/distr/float.rs.html).

`LiblrhsmmRsUniformSourceF32` and `LiblrhsmmRsUniformSourceF64` provide scalar
draws directly. Gaussian radial draws must be in(0,1]; angular and component
draws may include zero. Native Gaussian arithmetic and component choice are
retained, including the binary32 cosine kernel, unnormalized cumulative weights
and final-component fallback. Generation retains native duration rounding and
clamping; minimum constraints and segmentation jumps are not applied by this
native generation algorithm.

Model generation stages the sequence and allocates an independently owned complete
observation before publishing the sequence and result. Validation, callback or
native calculation errors preserve the previous sequence and initialized output
slot. Previously consumed draws and caller context mutations are not rolled back.
Generated observations remain usable after model and sequence release. Release
them with the corresponding existing observation-release function.

The generated header includes both random-word callback typedefs. The export
filter explicitly includes `typedefs`, as described in the primary
[cbindgen user guide](https://github.com/mozilla/cbindgen/blob/v0.29.4/docs.md).
Six Rust ABI tests compare every generated value, complete sequence, draw count
and successful random-word result with unchanged native Rust APIs and retained
original C corpora. Actual host C and Python callers exercise all10 exports,
all three original generation corpora and failure transactions. All five targets
pass75 suites /239 tests with zero failed/ignored tests and the three compile-fail
borrow gates. All27 C families pass five targets; all27 Python families pass
three64bit targets. Serialization on/off, Clippy/Rustdoc with warnings denied,
formatting, generated-header and whitespace checks pass. Header662 exports /
45 descriptors /54 owners and two random callback aliases; ABI version1.
Four original generation mappings are complete: C87 implemented /16 pending;
every WASM entry remains pending. Dataset/pool bindings, segmentation/set codecs,
SHIRO C, all WASM and final combined acceptance remain required. Evidence is
Debug/shared, not Release/static or Python32.

## Segmentation serialization

Ten optional exports provide strict byte/stream decoding, one-object stream-prefix
reading and byte/stream writing in f32 and f64. They use existing segmentation,
byte, I/O and decode-limit descriptors. The native decoder retains both original
array-header mismatches: the outer arity is3 despite four children, and each
jump array declares the number of jumps while emitting twice as many flat scalar
values. Jump probabilities remain binary32 on the wire in both precisions.

Strict reads reject trailing data and require EOF. Prefix reads consume exactly
one complete object, without reading even an EOF byte after its boundary. Resource
limits apply cumulatively to declared array entries and the extra jump scalar
values. The native strict-read limit helper uses the same decoder and EOF check
as the previous default reader. No schema or native calculation changed.

Callbacks are synchronous and borrowed. Short reads/writes and interruption
retries retain native I/O behavior. Constructor errors preserve initialized output
slots; output validation precedes any callback. A failed writer may have emitted
an incomplete prefix. Writing does not flush or close the caller stream. Byte
owners remain independent after segmentation release.

Four Rust ABI tests cover all fields and jump bits, six original/corrected C
segmentation fixtures, both storage precisions, exact bytes, binary32 rounding,
prefix boundaries, every truncation, exact resource budgets and malformed data.
Callback tests include one-byte transfers, interruptions, a reader that rejects
EOF requests, late writer failure and output retention. Actual host C and Python
callers invoke all10 exports and check complete fields from the original fixture,
exact wire, independent retained bytes and I/O/range/limit failures. Focused tests
and Clippy pass. All five targets pass76 suites /243 tests, zero failed/ignored
tests and the three compile-fail borrow gates. All28 C families pass five targets;
all28 Python families pass three64bit targets. Serialization on/off Rust/C/Python,
optional export absence, Clippy/Rustdoc with warnings denied, formatting,
generated-header and whitespace verification pass. Header672 exports /
45 descriptors /54 owners; ABI version1. Two original segmentation-codec mappings
complete: C89 implemented /14 pending. Every WASM entry remains pending.
Dataset/pool/set-codec bindings, SHIRO C, all WASM and final combined acceptance
remain required. Evidence is Debug/shared, not Release/static or Python32.

## Memory pools and allocation identities

Eleven unconditional exports expose native pool construction, complete accounting,
per-page usage, ordinary/zeroed allocation, checked byte copying/replacement,
allocation length/clone/release and pool release. They use two opaque owners and
layout/page descriptors. Page growth, requested capacities, initial empty pages
and allocation identity are delegated directly to the existing native pool.
No borrowed pointer to internal storage escapes the interface.

Cloning an allocation shares its region. Mutations through either identity are
visible to the other. Releasing an allocation does not reclaim its region; pool
release frees all pages. Remaining allocation handles retain metadata only: their
length and identity may be cloned after pool release, but a different pool rejects
access to that identity. All ordinary and zeroed allocations contain initialized
zero bytes, matching native Rust behavior. Original C uninitialized allocation
contents are not a defined input/output guarantee.

Byte operations validate complete ranges before copying. Empty intervals permit
null buffers at valid offsets. Cross-pool identities return status3, invalid ranges
return2, and invalid pointers return1. Failed allocations preserve existing
accounting, regions and initialized result slots. Output validation precedes
allocation. Pool reads cannot overlap mutation/release, and all mutations require
exclusive access. Release clears initialized unique-owner slots; a null owned
pointer is a no-op. The private release helper retains an explicit RefUnwindSafe
bound at the panic boundary.

Two Rust ABI tests compare every page and accounting field, region contents and
original C growth/checksum records for page sizes8 and0. Shared identities,
post-release metadata, cross-pool rejection, overflow, invalid intervals and
output/buffer/state retention are covered. Existing native pool tests remain.
Actual optimized assert-enabled C and ctypes callers use all11 exports and repeat
original C accounting and shared-region/error checks. Host tests and Clippy pass.
All five targets pass77 suites /245 tests, zero failed/ignored tests and the
three compile-fail borrow gates. All29 C families pass five targets; all29 Python
families pass three64bit targets. Serialization on/off Rust/C/Python pool checks,
unchanged native regressions, Clippy/Rustdoc with warnings denied, formatting,
generated-header and whitespace verification pass. Header683 exports /
47 descriptors /56 owners; ABI version1. Four original pool mappings complete:
C93 implemented /10 pending. Every WASM entry remains pending. Dataset/set-codec
bindings, SHIRO C, all WASM and final combined acceptance remain required.
Evidence is Debug/shared, not Release/static or Python32.

## Complete sample sets and incremental builders

Seventy-two unconditional exports expose observation and segmentation collections
in both precisions. Each collection supports construction from ordered borrowed
children, length, independent child retrieval, sample replacement, append, whole
set replacement, deep clone and release. Children may repeat in input arrays;
each resulting sample owns a separate complete copy. All frames/features or
boundaries/labels/outgoing jumps retain their native selected-precision values.
Heterogeneous sample shapes and empty collections are supported.

Each builder supports ordered slot construction and length, presence inspection,
independent sample retrieval/replacement, clearing, resizing, complete-state
cloning, validated construction and release. Missing slots remain present in the
builder's ordering; they are not removed during copying or construction. Growing
retains the existing prefix and appends missing slots; shrinking retains the
surviving prefix. Build copies the native builder and invokes its complete native
validation without consuming the caller's builder. Missing slots produce status3
and retain initialized constructor outputs. Empty builders build empty sets.

Private generic helpers share pointer-array validation, complete sample copying,
resize allocation and owner release. Every child pointer is checked before array
copies; every native sample is validated. Replacement stages all copies before
publishing. Failures preserve existing sets/slots and initialized outputs.
Getter results and cloned sets remain usable after sources/builders/parents are
released. No borrowed internal vector storage escapes the interface. Existing
native algorithms, wire formats, reference corpora and dependencies are unchanged.

Four Rust ABI tests use mixed shapes and selected-precision epsilon values,
compare every feature or label/jump bit, inspect all storage/builder operations
and verify failed replacements, unfilled/empty builders and independent lifetimes.
Actual optimized assert-enabled C and ctypes callers invoke all72 exports and
verify complete fields, partial-builder cloning, missing slots, retained outputs
and child lifetimes. Present zero-frame observations and zero-segment
segmentations retain their shape and slot presence across cloning, building and
parent release; they are distinct from unfilled slots. These additional actual
caller cases pass all five targets and the host without serialization.

All five targets pass78 suites /249 tests with no failed or ignored tests and all
three compile-fail borrow gates. All30 C families pass five targets; all30 Python
families pass the three64bit targets. Serialization on/off Rust/C/Python checks,
Clippy/Rustdoc with warnings denied, formatting, generated-header verification
and whitespace checks pass. Header755 exports /47 descriptors /64 owners, ABI1.
Five original storage mappings complete: C98 implemented /5 pending. Every WASM
entry remains pending. Dataset regrouping, set codecs, SHIRO C, all WASM and final
combined acceptance remain required. Debug/shared evidence only, not Release,
static libraries or Python32.

## Complete paired dataset regrouping

Twelve unconditional exports expose regrouping and independent paired result
owners in both precisions. `liblrhsmm_rs_regroup_data_f32` and its f64 counterpart
borrow complete observation and segmentation sets directly, invoke the native
regrouping implementation and publish one owner only after both result sets have
been constructed. Positive unit sizes, equal sample counts, matching stream
counts, ordered in-range boundaries and final frame agreement follow native
validation. Empty aligned inputs and samples with repeated boundaries are
supported; a final short group retains its frames and labels. As upstream,
regrouped outgoing jumps become ordinary next-state transitions.

Dataset length reports the paired sample count. Retrieval returns complete
independent editable observation or segmentation sets. Clone preserves both
sets, and release clears the unique owner slot. Previously retrieved sets and
children remain usable after input/dataset release. Pointer, configuration and
native failures preserve initialized outputs. No internal vector pointers escape
and no input-set copies are added before native regrouping.

Two Rust ABI tests compare every selected-precision feature, boundary, label and
jump against the native implementation for six unit sizes, repeated/mixed-shape
samples, empty samples and result lifetimes. They verify zero-unit, count/stream
mismatch, late final-boundary failures and retained output slots. Actual optimized
assert-enabled C and ctypes callers invoke all12 exports, compare all result
fields and verify all150 independent corrected-C double dataset records.
All five targets pass 79 suites and 251 tests with no failures or ignored
tests and all three compile-fail borrow gates. All 31 C caller families pass five
targets; all 31 Python families pass the three 64-bit targets. Serialization on/off
Rust/C/Python checks, Clippy and Rustdoc with warnings denied, formatting,
generated-header verification and whitespace checks pass. The header contains
767 exports, 47 descriptors and 66 owners, with ABI version 1. The regroup mapping
is complete: C99 implemented /4 pending. Every WASM entry remains pending.
Set codecs, SHIRO C bindings, all WASM and final combined acceptance remain
required. Evidence is Debug/shared, not Release/static or Python32.

## Complete observation and segmentation set serialization

Twenty optional exports expose strict byte and callback-stream decoders, one-set
prefix callback decoders, and byte/callback-stream encoders for complete sets in
both precisions. They reuse the existing set, byte, stream and limit owners or
descriptors. Each decoded sample retains its full shape and fields. Original
MessagePack array schemas, including both historical segmentation header
mismatches, are preserved. Wire features and jump probabilities remain binary32
when the in-memory owner uses f64; writing does not modify the source owner.

The entry limit is cumulative across the outer collection and every nested
sample and array. Strict decoders reject trailing bytes and require EOF through
callbacks. Prefix decoders leave the next object unread without an EOF request.
Short reads/writes and interruption retry use the existing native adapters.
Constructor failures preserve initialized outputs; callback contexts may have
consumed/emitted a prefix. Writers do not implicitly flush or close callbacks.
The functions are absent when serialization is disabled.

The native strict set readers now delegate to public strict-limit helpers with
unchanged default limits. Each helper extracts the existing decoder body; the
native decoder, writers, formats, corpora and previous C boundaries are unchanged.
Six Rust ABI tests compare all native fields and exact original wire, all nested
budget failures, every truncation, sequential sets, guarded prefix boundaries,
callback failures and retained output slots. They include original observation,
segmentation and copied-segmentation set fixtures, empty sets, generated selected
precision fixtures and mixed sets containing present empty children. Two f64
writer tests verify binary32 rounding/subnormal preservation and unchanged source
precision across encoding and reload.

Actual optimized assert-enabled C and ctypes callers invoke all20 exports, compare
all fields of both samples in the three original C set files, check exact wire,
empty collections, cumulative budget boundaries, every truncation, callback IO
errors and sequential object boundaries. All five targets pass 80 suites and
257 tests with no failures or ignored tests and all three compile-fail borrow
gates. All 32 C caller families pass five targets; all 32 Python families pass
the three 64-bit targets. Serialization on/off Rust/C/Python checks, Clippy and
Rustdoc with warnings denied, formatting, generated-header verification and
whitespace checks pass. The header contains 787 exports, 47 descriptors and
66 owners, with ABI version 1. The C caller symbol audit covers all 787 exports,
including the two invoked numerical macro expansions.

All four original set-codec mappings are complete. Every one of the 103 original
public liblrhsmm functions has a verified C ABI mapping; none remain pending.
Every WASM entry remains pending. SHIRO C, all three WASM interfaces and final
combined acceptance remain required. Debug/shared evidence only, not
Release/static/Python32.
