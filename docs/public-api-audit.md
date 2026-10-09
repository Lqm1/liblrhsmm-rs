# Public API audit

This audit supplements the upstream-symbol inventory. It does not infer complete
coverage from symbol counts. The functional mapping audits below are complete;
final cross-platform and combined end-to-end acceptance remains open. Model/statistic field
coverage below records the resize gap found during this audit and its fix.

## Verified configuration fields

The following native definitions were inspected directly together with their
WASM constructors, mutable fields and native conversion methods. Both binary32
and binary64 owners expose every listed field.

| Native definition | Complete public fields | WASM owner | Validation evidence |
| --- | --- | --- | --- |
| `PreparationSettings<T>` in `src/preparation.rs` | `duration_cache_length`, `duration_weight`, `temperature` | `PreparationOptionsF32/F64` | `wasm_configuration_checks.mjs`; `wasm_duration_checks.mjs` |
| `HsmmSettings<T>` in `src/hsmm.rs` | `temperature`, `duration_weight`, `state_radius`, `duration_extra`, `duration_extra_factor` | `HsmmOptionsF32/F64` | Configuration checks; `wasm_hsmm_checks.mjs` |
| `GeometricSettings<T>` in `src/geometric.rs` | `temperature`, `pruning_slope` | `GeometricOptionsF32/F64` | Configuration checks; `wasm_geometric_checks.mjs` |
| `UpdateSettings<T>` in `src/update.rs` | `duration_mode`, `duration_variance_floor`, `emission_variance_floor` | `UpdateOptionsF32/F64` | Configuration checks; duration/Gaussian/statistics/estimation checks |

The four native Default implementations supply the checked defaults. Integer
fields remain integer fields; scalar values retain their selected precision,
including signed zero and binary64 values above binary32 resolution. Copies
remain independent after mutation and source release. Invalid settings are
stored as ordinary editable fields and validated by native calculations, as
verified in the corresponding computational families.

## Verified enum alternatives and numerical configuration

`DurationMode` retains Normal/Geometric as update mode 0/1. Emission pruning
retains None/AroundBoundaries/Linear as modes 0/1/2 with the selected-precision
radius/slope parameter. Mixture occupancy retains Hsmm/Geometric as modes 0/1
with the selected-precision state radius/pruning slope. The existing emission,
posterior and update tests exercise actual calculation and reject unknown modes.

All seven native numerical constants in `src/numerics.rs` are returned by
`numerical_constants()`. The four mutable coefficient tables and their iterator
semantics are covered by the numerical family, including bit-preserving edits,
shared updates, independent returned arrays and restoration before later tests.
Per-call settings replace upstream global calculation settings; coefficient
table changes remain process-wide within one module as documented by native code.

The explicit configuration verifier checks 26 precision-specific fields, ten
pruning/occupancy alternatives and seven constants. It is part of the shared
module aggregate, complementing the tests that use these settings in actual
calculations. No configuration fields were removed or default values changed.

## Model and sufficient-statistic field audit

Native definitions in `src/model.rs` and `src/statistics.rs` were compared with
`src/wasm_api/duration.rs`, `gaussian.rs`, `containers.rs` and
`container_statistics.rs`. Every scalar-bearing owner has both precisions.

| Native definition | Complete public fields | WASM access and mutation | Evidence |
| --- | --- | --- | --- |
| `DurationParameters<T>` | `mean`, `variance`, `minimum`, `maximum`, `fixed_mean`, `variance_floor` | Typed values/setter; constraints/setter | Duration and ownership verifiers |
| `GaussianMixtureParameters<T>` | `dimensions`, `weights`, `means`, `variances`, `variance_floors` | Getters; complete replacement | Gaussian and ownership verifiers |
| `StreamParameters<T>` | `weight`, `mixtures` | Typed weight; copied emission get/set; complete collection replacement | Container verifier |
| `ModelParameters<T>` | `streams`, `durations` | Counts, copied child get/set; complete collection replacement | Container verifier |
| `StreamLayout` | `emissions`, `components`, `dimensions` | Ordered u32 triples in model constructor | Container verifier |
| `StreamParametersBuilder<T>` | `weight`, `emissions` | Weight, optional slot get/set/presence/clear, resize, clone, consuming build | Container and builder-resize verifiers |
| `ModelParametersBuilder<T>` | `streams`, `durations` | Optional slot get/set/presence/clear, both-list resize, clone, consuming build | Container and builder-resize verifiers |
| `DurationSufficientStatistics<T>` | `value_sum`, `squared_sum`, `occupancy` | Mutable fields, complete typed values/setter, clear/clone | Duration and ownership verifiers |
| `GaussianSufficientStatistics<T>` | `dimensions`, `weighted_sums`, `weighted_squared_sums`, `component_occupancies`, `state_occupancy` | Full getters and complete shape replacement, clear/clone | Gaussian/statistics/estimation/ownership verifiers |
| `StreamSufficientStatistics<T>` | `mixtures` | Copied emission get/set; collection replacement, clear/clone | Statistics verifier |
| `ModelSufficientStatistics<T>` | `streams`, `durations` | Copied child get/set; collection replacement, clear/clone | Statistics and estimation verifiers |

Native public vectors permit length changes. The original WASM builders exposed
optional slot contents but fixed their lengths at construction. This was a
coverage gap. `StreamBuilderF32/F64.resize(emissions)` and
`ModelBuilderF32/F64.resize(streams, durations)` preserve the overlapping prefix,
append empty slots and remove truncated slots. Native checked constructors
validate and allocate replacement lists before assigning either field. Invalid
lengths preserve every existing list, slot and weight.

The resize verifier covers growth, shrinkage, zero lengths, filled and empty
slots, removed slots staying absent after regrowth, clone independence, invalid
lengths for either model list, source release and consuming build with retained
parameters. The shared-module aggregate includes these checks. Inspection of
`src/c_api/model_builder.rs` and `src/c_api/stream_builder.rs` initially found no
equivalent length-change exports. Four new exports in `src/c_api/builder_resize.rs`
now provide both operations at both precisions. The generated header declares
them. Actual C and Python callers verify them on Windows x86_64. Cross-platform
validation of these additions remains part of the final matrix.

Validated parameter replacements enforce native invariants. Statistics
collection replacements retain raw native shapes; native training/update methods
validate them at use. This audit verifies functional field access, without
promising that arbitrary invalid Rust struct literals can be constructed as
validated JS model owners.

## Observation, segmentation and dataset field audit

Native definitions in `src/data.rs` and `src/data_operations.rs` were compared
with the full implementations in `src/wasm_api/data.rs` and `datasets.rs`.
All scalar-bearing definitions have binary32 and binary64 owners.

| Native definition | Complete public fields | WASM access and mutation | Evidence |
| --- | --- | --- | --- |
| `ObservationStreamData<T>` | `dimensions`, `values` | Dimension getter, copied values, complete replacement with arbitrary raw storage | Data verifier |
| `ObservationData<T>` | `frames`, `streams` | Counts, copied stream get/set, complete shape replacement, copied frame access | Data verifier |
| `JumpParameters<T>` | `delta`, `probability` | Ordered signed delta and typed probability arrays in `JumpGroupF32/F64`; construct a replacement group and set the outgoing group | Data verifier |
| `SegmentationData<T>` | `boundaries`, `duration_states`, `output_states`, `outgoing` | Complete getters/setters; construct and replace all fields to change stream/state shape; arbitrary state occurrence resegmentation | Data verifier |
| `AlignedDataset<T>` | `observations`, `segmentations` | Separate counts, copied collections/children, child setters, complete collection replacement | Dataset verifier |
| `SampleSetBuilder<T>` | `samples` | Optional slot presence/get/set/clear, checked resize, clone, consuming validated build | Dataset verifier |

The observation verifier changes both frame and stream counts, checks each
stream's dimensions and time-major values, then shrinks the shape. Empty-stream
and mismatched-storage replacements fail without changing the preceding value.
Raw stream storage preserves IEEE bits; native observation validation controls
which shapes can become observations. Segmentation setters retain signed state
indices and boundaries. Structural validation checks array lengths and the final
ordinary transition. Transition construction additionally checks finite,
nonnegative probabilities. The raw group setter does not silently normalize
weights or reject raw IEEE storage before that native calculation boundary.

All public methods of the native transition owners and views in
`src/transitions.rs` were also inspected. WASM exposes independent owned graphs
through borrowed-source and consuming-source construction, clone, copied
segmentation/incoming/outgoing access and consuming recovery of the segmentation.
Its HSMM and posterior calculations pass native graph views and reuse the stored
incoming transitions. WASM does not expose Rust lifetime parameters. The data
verifier checks all segmentation fields and both transition directions after
editing/replacing/releasing the source and editing a returned copy. Existing
HSMM and posterior verifiers cover calculation through these graphs.

Dataset replacements preserve raw independent observation and segmentation
counts. Regrouping and estimation validate pairing when used. The dataset
verifier exercises replacement, independent clones, copied collection lifetime,
optional builder slots and resizing, and full regrouping against the corrected C
reference. This closes the data-field audit; the remaining audits and final
cross-platform/end-to-end acceptance gates remain open.

## Prepared parameter, cache and view audit

The public definitions and methods in `src/preparation.rs`,
`src/preparation/owned.rs` and `src/probability.rs` were compared with the duration,
Gaussian and container bindings. Native borrowed preparations and views keep
Rust lifetime guarantees. Their WASM equivalents own independent snapshots and
return copied children, parameters, options and cache arrays.

| Native preparation family | Complete accessible data and operations | WASM equivalent | Evidence |
| --- | --- | --- | --- |
| Borrowed/owned/view duration | Parameters where exposed by native type, options, log normalizer, cached log probabilities, log/probability evaluation; view-to-owned and owned-to-parameters | `PreparedDurationF32/F64`, copied/consuming parameters, options and cache; clone | Duration, ownership and container verifiers |
| Borrowed/owned/view Gaussian | Parameters where exposed by native type, all cached normalizers, component log/probability and mixture log/probability evaluation; view-to-owned and owned-to-parameters | `PreparedGaussianF32/F64`, copied/consuming parameters and cache; clone | Gaussian, ownership and container verifiers |
| Owned stream | Parameters, consuming recovery, optional prepared emission view | `PreparedStreamF32/F64`, copied/consuming parameters and copied prepared emission; clone | Container verifier |
| Borrowed/owned model | Complete model, options, optional prepared duration/emission, emission log/probability tables | `PreparedModelF32/F64`, copied/consuming model, options and copied prepared children; both emission table methods | Container and emission verifiers |

Both ordinary and consuming preparation are exposed. Model preparation also
offers both forms of selected-duration preparation, preserving empty selections,
duplicate indices and absent unselected caches. Prepared HSMM inference methods
are checked by the HSMM family, including graph reuse and multiple cache lengths.
No prepared child silently returns an ordinary unprepared parameter owner.

The extended container verifier compares every child Gaussian normalizer and
duration cache entry with a standalone preparation, together with the duration
normalizer and all three settings. Mutating returned cache arrays or model/child
options does not mutate their owners. Existing checks evaluate children after
source and parent release, recover parameters by consuming a clone and prepare
them again. Borrowed native views reuse existing storage; WASM child conversion
uses native `to_owned` to copy existing caches without recomputing them.

## Codec operations and format choices

Every public codec in `src/serial.rs` was compared with the duration/Gaussian
bindings and `container_codec.rs`, `data_codec.rs` and `set_codec.rs`. The eight
families are duration, Gaussian, stream, model, observation, segmentation,
observation set and segmentation set. Each has both scalar precisions.

All families expose strict reading, strict reading with limits, prefix reading,
prefix reading with limits and writing. Prefix owners expose the consumed byte
position as a JS bigint, copied values and consuming recovery. Set prefix owners
use `samples`/`into_samples`; standalone owners use
`parameters`/`into_parameters`. JS supplies byte arrays and receives byte arrays;
native readers/writers remain generic over Rust `Read`/`Write`.

`DecodeLimits.max_array_entries` is the explicit limit argument. Default readers
retain the native default of 16,777,216 cumulative entries. The two
`ModelEncoding` choices are code 0 with variance floors and code 1 without
variance floors, exposed for Gaussian, stream and model output. Invalid codes
fail. Historical output rejects nonzero floors through native validation.
Duration and data formats do not acquire unrelated encoding choices. The native
binary32 set convenience functions map to the F32 collection methods; generic
set functions map to the corresponding precision-specific collection methods.

Existing verifiers compare output bytes with original C fixtures, including the
bundled historical model, current models, standalone objects and nested sets.
They verify strict trailing-data rejection, prefix positions and copied values,
exact entry budgets and one-entry-short rejection, truncated inputs, empty sets,
both format choices and disabled serialization exports. Data codecs retain the
legacy segmentation array-header mismatches because correcting those bytes
would break compatibility. Binary64 native computation still uses binary32
fields in the legacy wire format. The codec wrappers call the same native codecs
and do not introduce a new serialization schema.

Prepared/view and codec functional mapping audits are complete. These mappings
do not prove final cross-platform/end-to-end acceptance.

## Public modules and precision mapping

The 22 native public modules declared in `src/lib.rs` were reconciled with the
binding implementations and computational verifiers. The optional `c_api` and
`wasm_api` modules are language interfaces, separately covered by their own
header/export inventories. `model_operations` and `pruning` are private modules;
public methods implemented in `model_operations` are included in the model
mapping below. The private preparation implementation exports its public owned
types and views through the preparation module.

| Native public module | WASM implementation and functional evidence |
| --- | --- |
| `data` | `data.rs`; data fields, IEEE storage, frames and ownership |
| `data_operations` | `data.rs`, `datasets.rs`; resegmentation, sample builders, paired datasets and regrouping |
| `duration_occupancy` | `posterior_storage.rs`, `posteriors.rs`; duration posteriors and state occupancy calculation |
| `emission` | `emissions.rs`; editable pruning choices and both emission table calculations |
| `estimation` | `estimation.rs`; complete automatic HMM/HSMM expectation collection and training pipelines |
| `expectation` | `estimation.rs`; every expectation field, copied children/replacement and both manual collection methods |
| `generation` | `generation.rs`; precision-specific helpers, explicit draws, live uniform/raw-word generators and atomic sequence updates |
| `geometric` | `geometric.rs`; settings, all three inference methods, score tables and all total-probability methods |
| `hsmm` | `hsmm.rs`; settings, scores, complete paths, all three inference methods with ordinary and cached graphs |
| `initialization` | `estimation.rs`; aligned initialization collection and resulting parameter updates |
| `memory_pool` | `memory_pool.rs`; allocation, zeroed allocation, copied byte access/edit and all page/accounting fields |
| `mixture_occupancy` | `posterior_storage.rs`, `posteriors.rs`; both occupancy modes and all optional mixture storage |
| `model` | `duration.rs`, `gaussian.rs`, `containers.rs`; every parameter/builder/layout field and public model operation |
| `numerics` | `numerics.rs`; every constant, precision helper, mutable coefficient table and complete iterator |
| `occupancy` | `posterior_storage.rs`, `posteriors.rs`; precision-specific state tables, optional transitions and both geometric occupancy calculations |
| `preparation` | `duration.rs`, `containers.rs`, `emissions.rs`, `hsmm.rs`; ordinary/owned/selected preparation, caches, children and prepared calculations |
| `probability` | `gaussian.rs`; all prepared component/mixture log and ordinary probability methods |
| `scalar` | Concrete F32/F64 owners/functions use the native sealed `Scalar` implementations; scalar conversion and numerical operations remain native |
| `serial` | All eight codec families, strict/prefix/limited reading, both model encodings and writing |
| `statistics` | `duration.rs`, `gaussian.rs`, `container_statistics.rs`; all four statistic families, raw field storage and clearing |
| `transitions` | `data.rs`, `hsmm.rs`, `posteriors.rs`; arbitrary jumps, owned snapshots and cached native views |
| `update` | Duration/Gaussian/container statistics bindings; every update setting, both duration modes and all four parameter update methods |

Native `Scalar` is sealed to f32 and f64; bindings instantiate those two storage
precisions instead of exposing a user-implemented scalar trait. Public binary32
type aliases retain the F32 equivalent. Frame/state indices remain integers.
The `StateOccurrence` fields are ordered u32 state/end pairs, preserving repeated,
skipped and backward state references. Rust closures, generic RNGs, iterators,
borrowed slices and I/O traits map to the documented callback, owned iterator,
copied-array and byte-array operations. They do not require a third-party Rust
implementation to be constructible from JavaScript.

Transition, duration and mixture posterior storage is binary32 in the native
implementation for both calculation precisions. It intentionally uses shared
WASM owners rather than nonexistent binary64 storage variants. Numerical
constants, memory pools and integer fields are also shared where native types
are precision-independent. Generic model, observation, settings, expectation,
state-table, preparation and statistic owners expose both precisions.

`tests/wasm_precision_exports.cjs` checks actual generated Node exports in both
directions. It verifies 67 pairs with serialization and 59 without, comparing
constructor/function arity, static/instance method names and getter/setter
presence. This detects missing precision counterparts and asymmetric operations.
The shared-module computational families separately verify each precision's
behavior against C references, selected-precision values and complete native
training/inference workflows. Export symmetry alone is not numerical evidence.

The public-module and generic-precision functional mapping audits are complete.
No requirement for the final platform matrix, browser verification, complete
SHIRO bindings or combined three-project acceptance is waived by this audit.
