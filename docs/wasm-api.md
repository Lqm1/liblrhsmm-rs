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
original mappings were implemented at the duration checkpoint. With Gaussian
and container/statistics/data/dataset bindings, 67 original mappings are implemented; the other 36
remain pending.

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

## Gaussian mixtures, preparation, statistics and codecs

`GaussianF32/F64(mixtures, dimensions)` uses native dimension checks and
uniform/flat-start defaults. dimensions and mixtures expose the shape;
weights(), means(), variances() and variance_floors() return independent typed
arrays. replace(dimensions, weights, means, variances, variance_floors)
changes the complete shape and every field atomically after structural
validation. Arbitrary scalar bits remain permitted in stored parameters;
preparation, splitting and updates apply their native numeric validation.
validate(), cloned(), increase_mixtures(count, displacement), combine(list)
and from_durations(list) expose all native transformations.

`GaussianCollectionF32/F64` and `DurationCollectionF32/F64` provide new(),
length, is_empty(), push(value), get(index), replace(index, value), clear()
and cloned(). Entries and returned owners are independent copies. Repeated
inputs and source edits/releases do not alias collection storage. Out-of-range
get returns undefined; replace throws without modifying the collection.
These collections transfer complete owned distributions to native slice APIs.
Gaussian combination retains dimension order and floors, rejects multiple
components, and duration combination retains the original zero-floor behavior.

prepare() creates an independent `PreparedGaussianF32/F64` snapshot;
prepare_owned() consumes the editable distribution. Prepared owners expose
parameters(), into_parameters(), cloned(), cached_log_normalizers(),
component_log_probability(observation, component), component_probability,
log_probability(observation, temperature) and probability. Cached arrays and
parameters are copies; snapshots preserve matching parameters/normalizers
after source edits or release. Native DAEM, exponential approximation,
single-component weight convention and zero-temperature behavior are retained.

`GaussianStatisticsF32/F64(mixtures, dimensions)` and static from_gaussian()
allocate native zero statistics. dimensions, mixtures, weighted_sums(),
weighted_squared_sums(), component_occupancies() and state_occupancy() expose
every field. state_occupancy is a one-element typed array to preserve arbitrary
IEEE payloads. replace() accepts complete shape and four statistic arrays,
validates lengths atomically, and retains native numeric validation for update.
cloned() and clear() preserve native ownership and reset behavior.
Gaussian update(statistics, UpdateOptionsF32/F64) returns the native occupancy
flag and applies both configured/global and per-dimension variance floors.

With serialization enabled, Gaussian read()/read_with_limits() are strict,
read_prefix()/read_prefix_with_limits() return `DecodedGaussianF32/F64` with
BigInt position and parameters()/into_parameters(). write() uses the current
schema; write_with_encoding(0) includes variance floors and code1 selects the
historical schema. Unknown codes fail inside Rust Result. Historical output
retains native rejection of nonzero floors. Wire scalar rounding, exact object
boundaries, decode budgets, truncation and trailing-data errors remain native.
Codec methods and decoded owners are absent without serialization; OS IO
callbacks remain native/C interfaces.

Actual default/no-serialization Node.js and browser checks compare all230
Gaussian values from the two C precision corpora at the existing native
normalized bound2e-14, measured0. Seventy C update values and144 original C
single/multiple-component emission values match exactly. Both precisions
reproduce current/historical C fixtures, totaling414 exact bytes, and retain
native budgets18/29, every truncated prefix and BigInt positions. Checks also
cover defaults/all fields, mutable shapes, NaN payloads in every parameter and
statistic array, copied caches, source/collection release, consuming conversion,
repeated combination inputs, native weight/zero-temperature conventions,
invalid dimensions/shapes/modes/observations/temperatures/splits, failure
atomicity and historical floor rejection. Related native/C tests29 pass.
Clippy passes in both serialization configurations; WASM Rustdoc denies
warnings. Actual feature-disabled checks confirm all twelve new owners absent.
The numerical and duration Node verifiers also pass on this final module.
This adds fifteen original mappings; complete model/inference/training and
the final cross-crate acceptance remain required.

## Stream/model containers, builders, preparation and codecs

`StreamF32/F64(emissions, components, dimensions)` retains native flat-start
construction, and empty() uses native defaults. emissions exposes the count;
weight()/set_weight() use a one-element typed array to preserve arbitrary IEEE
bits. emission(index) copies a complete Gaussian owner; set_emission() replaces
one child. replace(weight, GaussianCollection) changes the complete shape and
parameters after structural validation. validate() and cloned() are native.
`StreamCollectionF32/F64` has the same copied-entry operations as Gaussian and
duration collections. `StreamBuilderF32/F64(emissions)` exposes every optional
slot through emission(), has_emission(), set_emission(), clear_emission(),
weight()/set_weight(), emissions and cloned(). build() consumes the builder,
rejects unfilled slots and permits native empty containers.

`ModelF32/F64(layouts, durations)` accepts Uint32Array triples for every
StreamLayout field: emissions, components, dimensions. Malformed triples fail
before construction. empty() returns the native empty model. streams and
durations expose counts; stream()/duration() copy complete children;
set_stream()/set_duration() replace children. replace(StreamCollection,
DurationCollection) changes the complete model shape after structural
validation. validate() and cloned() retain native behavior.
`ModelBuilderF32/F64(streams, durations)` exposes every optional slot through
stream()/duration(), has_stream()/has_duration(), set_stream()/set_duration(),
clear_stream()/clear_duration(), counts and cloned(). build() consumes the
builder and rejects incomplete models. Copies remain independent after source
edits, releases or repeated child inputs. Invalid slot indices throw atomically.

Stream prepare()/prepare_owned() return `PreparedStreamF32/F64`; emission()
copies an already prepared Gaussian view with its original matching cache.
parameters(), into_parameters() and cloned() preserve snapshot/consumption
semantics. Model prepare(options)/prepare_owned(options) prepare all durations;
prepare_selected_durations(options, states)/prepare_owned_selected_durations()
accept Uint32Array state indices, including duplicates and empty selections.
`PreparedModelF32/F64` exposes parameters(), into_parameters(), cloned(),
options(), duration(state) and emission(stream, state), copying complete
matching prepared views. Unselected or missing states return undefined.
These parameter/preparation bindings do not yet expose full emission-table,
inference, occupancy or training methods; those remain required.

Stream/Model codecs have strict read()/read_with_limits(), exact-boundary
read_prefix()/read_prefix_with_limits(), write() and write_with_encoding(0/1).
`DecodedStreamF32/F64` and `DecodedModelF32/F64` expose the consumed BigInt
position, copied parameters() and consuming into_parameters(). Current and
historical native schemas, scalar wire rounding, floor preservation checks,
truncation/trailing-data errors and decode budgets are retained. All codec
methods/decoded owners remain absent without serialization.

Actual Node.js and browser verify491793 exact bytes across the original C
flat-start construction, current/historical stream fixtures and complete
current/historical HSMM models in both precisions. Exact native decode budgets
33/6452/28606/20866, prefix positions, trailing-input rejection and truncation
checks pass. Every prefix truncation of the small stream fixture is tested;
larger fixtures use selected beginning/middle/end truncations. Both precisions
read the historical215 durations, find205 initialized states and prepare only
those states, totaling410 checked selected durations. Construction, builder
population/clear/unfilled errors, native empty models, full children, shape
replacement, independent copies/consumption/source release, NaN stream-weight
storage, duplicate/empty selected states and prepared-child lifetime checks
pass. Related native/C tests23 pass; default and no-serialization Clippy and
default WASM Rustdoc deny warnings. Actual no-serialization Node and feature-off
18-owner absence checks pass. Numerical/duration/Gaussian Node verifiers also
pass on this final module. This checkpoint adds eleven original mappings;
stream/model statistics, inference/training and cross-crate acceptance remain
required.

## Complete stream/model statistics and maximization

`StreamStatisticsF32/F64` exposes new(), from_stream(), emissions,
emission(index), set_emission(index, GaussianStatistics),
replace(GaussianStatisticsCollection), cloned() and clear().
`ModelStatisticsF32/F64` exposes new(), from_model(), streams, durations,
stream()/duration(), set_stream()/set_duration(),
replace(StreamStatisticsCollection, DurationStatisticsCollection), cloned()
and clear(). new() permits an empty native statistics structure; from_stream()
and from_model() allocate all matching zeroed child statistics. Entire child
shapes and every statistic field remain editable. Native validation is retained
at maximization/estimation boundaries, rather than rejecting arbitrary stored
moments. get operations return independent copies or undefined for missing
indices; invalid child replacements fail without modifying the parent.
clear() resets every child sum, squared sum and occupancy, retaining shapes.

`GaussianStatisticsCollectionF32/F64`, `StreamStatisticsCollectionF32/F64`
and `DurationStatisticsCollectionF32/F64` expose new(), length, is_empty(),
push(), get(), replace(), clear() and cloned(), copying complete entries.
Collections support repeated input owners, independent nested copies and
source release. Duration statistic copying preserves arbitrary IEEE payloads
through typed values(), including nested model snapshots.

Stream and Model update(statistics, UpdateOptionsF32/F64) delegate to native
maximization. Model updates validate all shapes/moments/options and commit only
after every child update succeeds. Stream updates likewise preserve earlier
emissions if a later emission fails. Per-distribution floors, fixed-duration
means, normal/geometric mode, low occupancy and empty-container option
validation retain the native behavior. Statistical collection, initialization
and automatic HMM/HSMM estimation methods remain to implement.

Actual Node.js with/without serialization and browser tests compare160 exact
C-derived parent update values across both precisions. They check every stream
and model moment shape, nested replacement, recursive clear, independent
clones/collections/child lifetimes, arbitrary duration statistic bits and
invalid indices/options/statistics. Deliberate late Gaussian and duration
failures retain all earlier model parameters. Both precisions complete manual
moment update then preparation, with save/read/re-prepare density equality
when serialization is enabled. This is a maximization pipeline, not final
automatic training/inference acceptance. Twenty related native/C tests pass;
Clippy passes in both feature configurations and WASM Rustdoc denies warnings.
Actual feature-disabled checks confirm ten new owners absent. All four prior
Node verifiers also pass on the final module. Eight original mappings are added;
full data/inference/training/generation interfaces and final acceptance remain
required.

## Observation, segmentation and transition data

`ObservationStreamF32/F64(dimensions, values)` stores the complete native raw
stream fields; dimensions, values(), replace() and cloned() expose them.
Standalone raw streams permit arbitrary stored shapes/bits, while a parent
Observation validates the shared frame/storage shape before accepting one.
`ObservationF32/F64(frames, dimensions)` accepts Uint32Array stream dimensions,
uses native zero initialization and exposes frames, streams, stream(index),
set_stream(index, value), replace(frames, ObservationStreamCollection),
frame(stream, time), validate() and cloned(). Frame arrays and child owners are
independent copies; missing frames/streams return undefined. Shape changes are
atomic, and arbitrary IEEE feature bits remain stored without numeric filtering.

`SegmentationF32/F64(streams, segments)` retains unaligned/default next-state
construction and checked legacy/addressable shapes. boundaries(),
duration_states(), output_states(stream) and corresponding set methods expose
every integer field. outgoing(state) and set_outgoing(state, JumpGroup) expose
every jump group. `JumpGroupF32/F64(deltas, probabilities)` has length,
deltas(), probabilities() and cloned(), retaining complete selected-precision
storage. Segmentation replacement/cloning copies the complete shape; validate()
retains native final-delta1 group rules. Numeric probability validation remains
in add_jump()/transition building. Native add_jump(state, delta, probability)
retains order and updates the ordinary next-state weight for delta1.

build_transitions() and consuming build_transitions_owned() return
`TransitionGraphF32/F64` with complete independent segmentation and caches.
incoming()/outgoing() return copied groups; segmentation(),
into_segmentation() and cloned() retain cache ownership/lifetime semantics.
Borrowed/view APIs are represented by owned snapshots across JavaScript.
resegment(Uint32Array state/end pairs) exposes every StateOccurrence field,
permits native repeated/skipped/backward states and resets jumps as upstream.
ObservationStreamCollection, ObservationCollection and SegmentationCollection
in both precisions have the same copied-entry methods as other collections.
Sample-set builders, complete set codecs and paired dataset operations remain
required separately.

Observation/Segmentation strict/prefix/limited codecs and
`DecodedObservationF32/F64`/`DecodedSegmentationF32/F64` expose bytes and BigInt
object positions using the original native schema. Segmentation writing retains
the two historical array-header mismatches for C compatibility. Native resource
limits, truncation/trailing errors and scalar wire rounding remain intact.
Codec methods/decoded owners are absent without serialization.

Actual default/no-serialization Node.js and browser checks compare64 complete
repeated original C transition records and remap eight states. Both precisions
deliberately receive exact widened binary32 C transition inputs for this corpus;
additional tests retain binary64 epsilon precision, extreme offsets and signed
zero in native graph storage. Ten original/corrected C data fixtures plus the
generated resegmentation comparison cover970 exact bytes. Every truncated prefix, exact calculated
native array-entry budget and BigInt cursor is verified. Full fields, feature
order/frame bounds, mutable shapes, raw IEEE bits, nested copies and parent
release, missing ordinary transitions, invalid shapes/probabilities/paths,
failure atomicity, empty data/graphs and state resets are covered. Twenty-seven
related native/C tests pass. Clippy passes in both configurations and WASM
Rustdoc denies warnings; actual feature-disabled checks prove20 owners absent.
All five prior Node verifiers pass on the final module. Eleven original mappings
are added; set/dataset, inference/training/generation and final acceptance remain
required.

## Complete sample sets and aligned datasets

`ObservationSetBuilderF32/F64` and `SegmentationSetBuilderF32/F64` expose every
optional sample slot through length, has/get/set/clear, resize, copied ownership
and consuming build. Resizing preserves the retained slots and leaves new slots
empty. Build uses native validation and rejects missing samples.

`DatasetF32/F64` retain every observation and segmentation independently,
including raw mismatched sample counts. Construction, complete replacement,
indexed editing, copied child/collection access and cloning preserve the native
data representation. `regroup` and `regroup_data_f32/f64` apply native alignment
validation and splitting. They retain utterance boundaries, repeated zero-frame
segments, final short groups, all state fields and the native ordinary-jump reset.

With serialization enabled, existing observation and segmentation collection
owners provide strict/prefix reads, both limited forms, and original wire writes.
`DecodedObservationSetF32/F64` and `DecodedSegmentationSetF32/F64` expose BigInt
cursor positions, copied samples and consuming extraction. All four decoded
owners and codec methods are absent without serialization.

Actual default/no-serialization Node.js and default browser checks compare all
150 corrected-C double regroup records, twelve grouping checks across both
precisions and six unit sizes, and 878 exact original set/split bytes with
serialization. Tests cover full builder slots, resizing and failed construction,
source release and independent nested copies, multiple utterances, IEEE feature
bits, empty sets, raw mismatched datasets, invalid alignment and atomic errors.
Strict/prefix codecs retain exact nested budgets, BigInt positions, trailing
rejection and rejection of every truncated prefix. Twenty related native/C
tests pass. WASM Clippy passes in both feature configurations and Rustdoc denies
warnings. Actual feature-disabled checks prove ten owners and two functions
absent. All six prior Node verifiers pass on the final module. Ten original
mappings are added; inference, automatic training, generation, configuration
and final combined acceptance remain required.

## Primary references

- [Numeric slices](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/number-slices.html)
- [Returned numeric arrays](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/boxed-slices.html)
- [Exported owner classes](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/exported-rust-types.html)
- [Returned Result errors](https://wasm-bindgen.github.io/wasm-bindgen/reference/types/result.html)
- [Deployment targets](https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html)
- [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [DoubleEndedIterator](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)
- [ExactSizeIterator](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)
