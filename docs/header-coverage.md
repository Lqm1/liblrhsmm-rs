# Public header structure and macro coverage

Baseline: liblrhsmm `1df92da4b77377f4725509e7240a9107ed4c063b`.
This supplements the function inventory; function coverage alone does not
establish complete port coverage. All public functionality remains in scope.
The tables distinguish parameter storage from derived, borrowed views. They
describe the current Rust interface, not C layout or ABI compatibility.

## Model and statistics fields

| C structure and fields | Rust representation |
| --- | --- |
| `lrh_duration.mean`, `var`, `floor`, `ceil`, `fixed_mean`, `vfloor` | `DurationParameters<T>::mean`, `variance`, `minimum`, `maximum`, `fixed_mean`, `variance_floor` |
| `lrh_duration._tmp_term`, `_tmp_prep` | `PreparedDurationParameters<T>::log_normalizer()`, `cached_log_probabilities()` |
| `lrh_duration_stat.valsum`, `sqrsum`, `ocpsum` | `DurationSufficientStatistics<T>::value_sum`, `squared_sum`, `occupancy` |
| `lrh_gmm.mean`, `var`, `weight`, `vfloor`, `ndim`, `nmix` | `GaussianMixtureParameters<T>::means`, `variances`, `weights`, `variance_floors`, `dimensions`, `weights.len()` |
| `lrh_gmm._tmp_term` | `PreparedGaussianParameters<T>::cached_log_normalizers()` |
| `lrh_gmm_stat.valsum`, `sqrsum`, `weightsum`, `ocpsum`, `ndim`, `nmix` | `GaussianSufficientStatistics<T>::weighted_sums`, `weighted_squared_sums`, `component_occupancies`, `state_occupancy`, `dimensions`, `component_occupancies.len()` |
| `lrh_stream.gmms`, `ngmm`, `weight` | `StreamParameters<T>::mixtures`, `mixtures.len()`, `weight` |
| `lrh_stream_stat.gmms`, `ngmm` | `StreamSufficientStatistics<T>::mixtures`, `mixtures.len()` |
| `lrh_model.streams`, `durations`, `nstream`, `nduration` | `ModelParameters<T>::streams`, `durations`, and their lengths |
| `lrh_model_stat.streams`, `durations`, `nstream`, `nduration` | `ModelSufficientStatistics<T>::streams`, `durations`, and their lengths |

Parameter and accumulator vectors remain public and editable. Preparation owns
derived caches while borrowing the parameters. Arbitrarily changing a C cache
or its owning pointer has no direct safe Rust field equivalent; preparation
recomputes the cache and the borrow prevents stale parameters. The Gaussian
cached value is the sum of negative log variance terms, before the density's
factor of one half. The duration normalizer precedes weighting and annealing.
With zero cache length, Rust still prepares its normalizer; C may leave its
sentinel untouched until evaluation. These are representation differences,
not a claim of identical arbitrary cache mutation behavior.

Evidence: `tests/construction.rs`, `tests/model_operations.rs`,
`tests/precision.rs`, `tests/preparation.rs`, `tests/precision_training.rs`,
and `tests/structure_fields.rs`. The last test checks 12 prepared values per
precision against unmodified C. Binary32 values must match bits; the binary64
corpus uses a normalized error bound of 2e-14, not a universal libm bound.

## Observation, segmentation and sample fields

| C structure and fields | Rust representation |
| --- | --- |
| `lrh_observ.data`, `ndim`, `nstream`, `nt` | `ObservationData<T>::streams`, each stream's `values` and `dimensions`, `streams.len()`, `frames` |
| `lrh_seg.time`, `durstate`, `outstate`, `nseg`, `nstream` | `SegmentationData<T>::boundaries`, `duration_states`, `output_states`, `boundaries.len()`, `output_states.len()` |
| `lrh_seg.djump_out`, `pjump_out` | `SegmentationData<T>::outgoing`, combining each delta/probability pair in `JumpParameters<T>` |
| `lrh_seg.djump_in`, `pjump_in` | `TransitionGraphParameters<'_, T>::incoming()` and independent `OwnedTransitionGraphParameters<T>` caches, built from outgoing topology; `TransitionGraphView` borrows either cache |
| `lrh_observset.samples`, `nsample` | `Vec<ObservationData<T>>`, its length; `SampleSetBuilder` represents unfilled construction slots |
| `lrh_segset.samples`, `nsample` | `Vec<SegmentationData<T>>`, its length; `SampleSetBuilder` represents unfilled construction slots |
| `lrh_dataset.observset`, `segset` | `AlignedDataset<T>::observations`, `segmentations` |
| `lrh_pslice.p`, `lower`, `upper` | Duration posterior's `probabilities`, `minimum`, and `minimum + probabilities.len() - 1`; mixture slices use component indices; geometric transitions name `stay` and `advance` |

Incoming transitions are a derived immutable view, so callers cannot leave
incoming and outgoing edges inconsistent. Source order and ordinary-edge
sentinels are covered by `tests/transitions.rs`. Posterior probabilities remain
binary32 in both source precisions. Optional slices represent absent entries;
duration lookup returns zero outside its inclusive bounds. Evidence also
includes `tests/data_operations.rs`, `tests/precision_datasets.rs`,
`tests/duration_occupancy.rs`, `tests/mixture_occupancy.rs`, and
`tests/geometric_inference.rs`.

## Memory pool fields

| C structure and fields | Rust representation |
| --- | --- |
| `lrh_mempool.first`, `last`; `lrh_mempool_page.next` | Ordered `MemoryPool::pages()` iterator; first/last entries and iteration replace linked owning pointers |
| `lrh_mempool.pagesize`, `totalsize`, `npage` | `page_size()`, `total_capacity()`, `page_count()` |
| `lrh_mempool_page.top`, `capacity` | Each `PageUsage::used_bytes`, `capacity` |
| `lrh_mempool_page.data` | Allocation handles plus `MemoryPool::get()` / `get_mut()` borrow initialized allocation regions |

The iterator has an exact remaining length. Page accounting uses requested
capacity rather than allocator-specific vector capacity. Page growth, including
oversized and zero-sized requests, follows the C last-page policy. Unsafe
access to unallocated bytes and owning pointer mutation are replaced by checked
allocation access. `tests/memory_pool.rs` covers handle lifetime, isolation and
growth. `tests/structure_fields.rs` compares all 13 page records for five
successive requests with the original linked list, including iterator lengths
and aggregate accounting.

## Header macros

| Source macro | Current counterpart / audit status |
| --- | --- |
| `lrh_gmmu`, `lrh_gmmv`, `lrh_gmmvf`, `lrh_gmmvs`, `lrh_gmmss` | Public flat component-major vectors; index `component * dimensions + dimension` |
| `lrh_obm` | Public time-major values; `frame(stream, time)` provides a checked dimension slice |
| `LRH_UNINITIALIZED` | Explicit prepared-object state rather than an editable numeric cache sentinel |
| `max`, `min` | Validated scalar comparisons; expression helpers do not require global Rust macros |
| `NEGINF` | `numerics::LOG_ZERO` and `LOG_ZERO_F64`; selected-precision inference sentinel |
| `EPS` | `numerics::OCCUPANCY_EPSILON`, retaining the unsuffixed binary64 `1e-7` in three maximization comparisons and HSMM state-occupancy row normalization |
| `M_PI` | Standard binary64 PI constant in Gaussian generation before source float-only cosine |
| `lrh_lse_table_size`, `lrh_exp_table_size`, table step macros | Public `LOG_ADD_TABLE_SIZE`, `EXPONENTIAL_TABLE_SIZE`, `LOG_ADD_TABLE_STEP`, `EXPONENTIAL_TABLE_STEP`; both precisions checked in `tests/numerics.rs` |
| `p`, `a`, `b`, `x`, `y`, `m` in `inference-helper.h` | Time-major emission, score, duration, state and mixture tables, indexed as `frame * states + state`; exposed through their `values()` and checked `get()` methods |
| `alloc`, `alloc_init`, `x_safe` | Owned binary32 posterior vectors replace pool allocations; `DurationPosterior::probability` returns zero outside its inclusive bounds. `alloc_init` has no source use site beyond its definition |
| `calculate_maxdur` | Private `hsmm::duration_limit`, retaining both integer assignments and the positive ceiling; 3,024 independent C boundary cases per precision |
| `for_tj_forward`, `for_tj_backward`, `end_for_tj` | Explicit segment/time/state loops with the source forward inclusive and backward exclusive right boundaries; shared `pruning::boundary_states` retains fractional lower truncation and inclusive upper state comparison |
| Local `ab`, `ae`, `bb`, `be` aliases | `HsmmScoreTables::begin` / `end`, with checked time-major score-table views |
| Local `sb`, `se`, `s` aliases and `VITERBI` selection | Owned backtracking vectors; shared HSMM/geometric search selects maximum/path tracking or log-sum evaluation for the corresponding public entry point |
| Header guards | C preprocessing mechanism; Rust module boundaries |

The math header also exposes mutable numerical table globals. `log_add_table`,
`exponential_table` and their `_f64` counterparts provide allocation-free,
exact-length, double-ended read access. All 28,000 default coefficients are
checked bit-for-bit against independently compiled C tables through these
public APIs, including first/last traversal and iterator exhaustion.
`LOG_ADD_COEFFICIENTS`, `EXPONENTIAL_COEFFICIENTS` and their `_F64` counterparts
provide checked `get` and `replace` operations. Changes affect subsequent
selected-precision helpers, emission calculations, inference and training,
matching the process-wide C globals. Binary32 and binary64 tables are separate.
Integer atomics preserve every scalar bit, including NaN payloads and signed
zero. Relaxed access orders each coefficient independently and publishes no
other memory; it does not promise a coherent table or inference snapshot during
concurrent modification. Configure tables before computation for reproducibility.

`tests/mutable_tables.rs` runs in a separate integration-test process, checks
default and modified coefficients against unmodified C sources, and restores
the originals on exit. Per precision, 258 numeric comparisons cover standalone
helpers, forward/backward score arrays, likelihood and sufficient statistics.
Training likelihoods must actually change; binary32 comparisons require exact
bits and binary64 comparisons retain the 2e-14 corpus-specific bound. Invalid
indices cannot mutate a table. The test also checks independent precision
tables, binary32 signaling-NaN payloads, and concurrent binary64 reads/replacements
to exercise whole-scalar access. These checks supplement the atomic API's memory
model; a stress test alone is not a proof of concurrency correctness.
The own-header macro use sites now have corresponding storage/loop/arithmetic
representations. This does not establish complete optimized C numerical
compatibility or the final native acceptance gate; those requirements and
bindings remain separate.

The cache/page fixture generator compiles unmodified `common.c`, `math-funcs.c`,
`model.c`, and `mempool.c` in both precisions with floating-point contraction
disabled. Windows Zig and Linux GCC regenerate the same 25-record fixtures
byte-for-byte. The generator and driver are tracked in `tests/`.

Primary iterator reference, consulted through Context7, Mintlify and the Rust
standard-library documentation:
https://doc.rust-lang.org/std/iter/struct.Map.html
https://doc.rust-lang.org/std/slice/struct.ChunksExact.html
https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU32.html
https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU64.html
https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html

The EPS use-site search covers the pinned project's own translation units:
three occurrences in `estimate.c` and one in `inference.c`. Rust retains these
three comparisons in `update.rs` and the row denominator in
`duration_occupancy.rs`. The constant remains binary64 even for binary32 model
storage, preserving the C unsuffixed expression before the final assignment.
The occupancy and precision-training C comparisons exercise these arithmetic
paths; no threshold or operation order changed.

## Duration bounds and traversal use sites

External posterior construction is supported by `DurationOccupancies::from_values`,
`TransitionOccupancies::from_values` and `MixtureOccupancies::from_values`.
These correspond to caller-owned `lrh_pslice` arrays passed to `lrh_stocp`,
`lrh_collect_ocp` and `lrh_collect_ocp_geometric`. Tables retain time-major order,
absent slices, empty slices and binary32 probability storage at both model
precisions. Constructors reject zero/mismatched/overflowing dimensions, negative
or nonfinite weights, and overflowing inclusive duration bounds. Nonnegative
weights may exceed one and need not be normalized. Model-specific component
counts are checked by expectation collection before statistics are changed.
`tests/external_posteriors.rs` passes externally constructed unnormalized weights
through both collectors with binary32 and binary64 models and checks independently
calculated duration and emission sufficient statistics. It also checks malformed
inputs, failure atomicity, absent/empty slices and signed-zero bit preservation.

The original `calculate_maxdur` first truncates
`standard_duration * (1 + extra_factor) + extra` to an integer, then compares
that integer with `mean + 2 * sqrt(variance)` and truncates the selected value
again. A positive duration ceiling is applied last. Rust's private
`duration_limit` preserves this order. Forward duration storage may then be
clipped to `frames + 1`, because candidates cannot start before frame zero;
backward and posterior calculations retain beyond-sample candidates.

`tests/reference/duration_bounds_oracle.c` includes the original macro and
compiles the source `common.c` unchanged. Each precision has 3,024 cases:
zero and positive reference durations, three extras, seven factors including
neighbors of one third, mean neighbors at an integer boundary, three positive
variances, and four ceilings. The private helper is compared with every exact
integer output in the unit test, without exposing implementation-only metadata
as a new public API. Windows Zig and Linux GCC both reproduce fixture bytes
under `-O2` and `-Ofast`, with contraction disabled in both profiles. This is
evidence for this macro's corpus, not for every optimized C algorithm.

Forward source uses of `for_tj_forward(0, 0, 1, 1)` map to inclusive right
segment boundaries capped at `frames - 1` in emission, HSMM forward, state
occupancy, mixture occupancy and expectation collection. The duration-posterior
call uses maximum state offset two; Rust explicitly excludes the final absorbing
state. Backward reverses segment order and visits the half-open interval from
the previous boundary to the current boundary. All paths share fractional state
bound handling. The source's final-state backward initiation retains only later
emissions, and Rust preserves that special case. The previously documented
backward destination-index defect remains corrected independently.

Evidence includes `tests/hsmm_inference.rs`, `tests/fractional_pruning.rs`,
`tests/precision_inference.rs`, `tests/duration_occupancy.rs`,
`tests/mixture_occupancy.rs`, `tests/estimation.rs`, and
`tests/precision_training.rs`. These cover forward/backward scores and paths,
fractional pruning in inference and learning, state jumps, posterior storage,
selected precision, and the disclosed source corrections. Local forward
header aliases index the same score/backtracking storage; C's repeated include
under `VITERBI` is represented by shared Rust searches with a path-tracking
choice. Neither the aliases nor `end_for_tj` add an independent runtime API.

## Duration C interfaces

The opt-in C boundary exposes all six duration parameter fields through complete
binary32/binary64 descriptors, with create/get/set/clone/release operations.
Independent immutable preparation descriptors retain cache length, weight and
temperature. Prepared snapshots expose all parameter fields, options, the cached
normalizer, cache length and checked interval copies. Source editing/release and
snapshot cloning cannot stale the stored parameters. Derived caches stay
immutable, following the native preparation contract; reprepare edited parameters.
The existing borrowed native APIs retain their lifetime restrictions.

Evidence: `tests/owned_preparation.rs`, `tests/c_api_duration.rs`,
`tests/c_api_prepared_duration.rs` and both ordinary/prepared C/Python callers.
Both original C normalizer/cache corpora and complete binary32 preparation cases
are checked on the required target matrix. Gaussian/model preparation, statistics,
other field/macro bindings and all WASM mappings remain required; this duration
checkpoint is not a whole-header or whole-project completion claim.

## Duration statistics and update C interfaces

Both precisions expose all three `lrh_duration_stat` accumulator fields through
editable complete descriptors and independent owned accumulators. Default/create,
get/set/clone, native clear and release preserve flat-start construction and
source-independent lifetime. The complete maximization descriptor retains both
variance floors and the normal/geometric mode; update delegates existing native
logic and publishes parameters/result flags only on success. Direct public-field
accumulation stays available through get/edit/set. Full model initialization and
EM accumulation, Gaussian/stream/model statistics and all other bindings remain
required.

Evidence: `tests/c_api_duration_statistics.rs` and its actual C/Python callers,
both original C duration update corpora, exact native comparisons, fixed-mean
residual variance, invalid values/options and output/model preservation across
all five native targets. WASM access is still pending.

## Duration codec and callback C interfaces

Both precisions expose strict standalone duration decoding, one-object borrowed
stream decoding and binary32 wire encoding through owned bytes or synchronous
writers. Decoder resource bounds and source-compatible object boundaries are
retained. Borrowed callbacks support short transfers and interruption; explicit
flush is separate from encoding. Byte owner interval copies avoid internal-pointer
lifetime exposure. All optional codec declarations follow the native serialization
feature; byte/I/O helpers remain available without it.

Evidence: `tests/c_api_duration_serial.rs`, `tests/duration_prefix.rs`, actual
C/Python codec callers, both original duration wire fixtures and optional-feature
callers. Source `serial.c` confirms readers do not require EOF. The seven other
native reader mappings were initially reopened for prefix parity. The follow-up
now verifies native prefix readers in tests/serial_prefix.rs on all five targets;
their C/WASM readers remain pending. Every outstanding C/field/macro/WASM mapping and final
combined acceptance remain required.

## Gaussian parameter C interfaces

`LiblrhsmmRsGaussianParametersF32/F64` borrow both shape values and all four
parameter arrays; create/replace copy independent storage. The layout descriptor
and selectors expose dimensions, mixture count, weights, means, variances and
variance floors for reads and edits without exporting owner pointers. Clones and
native combine/duration-conversion/splitting retain all field behavior, including
arbitrary parameter bits and flat-start storage. Both source count fields map to
validated native shape; preparation's `_tmp_term` remains pending.

Evidence: `tests/c_api_gaussian.rs`, assert-enabled C/Python Gaussian callers,
both original precision parameter corpora and all five native target suites.
This completes five source function mappings. Parameter-owner deletion is verified,
while the complete deletion mapping also needs prepared-owner release. Gaussian cache/statistics/codec
or whole-library binding coverage. WASM remains pending.

## Prepared Gaussian C interfaces

Both `LiblrhsmmRsPreparedGaussianF32/F64` independently own immutable parameters
and matching normalizers through the native owned preparation. Component count
and parameter fields remain available through independent editable parameter
copies. `_tmp_term` maps to checked cached-normalizer count/copies before the
factor of one half. Public component/whole-mixture kernels delegate the shared
native view and keep all precision/weight/temperature behavior. Destruction has
verified editable and prepared owner releases. Gaussian statistics fields and
codecs remain pending.

Evidence: `tests/owned_gaussian.rs`, `tests/c_api_prepared_gaussian.rs`, actual
C/Python prepared Gaussian callers and original precision/structure fixtures,
with all five native target suites. WASM and full combined acceptance remain
pending.

## Gaussian sufficient-statistics C fields

`LiblrhsmmRsGaussianStatisticsValuesF32/F64` expose full shape, state occupancy
and three borrowed vector inputs. The layouts return mixture count, dimensions
and state occupancy; interval copies/edits expose weighted sums, squared sums and
component occupancies. Full replacement supports shape changes without exporting
internal pointers. Constructors/from-model, clones, clear and release own their
storage independently. Native update validates the editable values and uses the
existing complete update settings and transactional model replacement.

Evidence: `tests/gaussian_statistics.rs`, four boundary tests, actual assert-enabled
C/Python callers and both original update corpora on all five native targets.
Statistics collection through whole-model initialization/EM, stream/model stats
and all WASM remain pending.

## Gaussian codec C interfaces

All native GMM parameter fields are retained through both current and historical
MessagePack layouts and both storage precisions. Explicit selectors expose native
encoding choices; decoders infer the original layout, use caller-selected limits
and preserve one-object stream boundaries through the prefix entry points. Common
callback/byte ownership adapters are reused; every optional function/selector is
covered by the serialization feature guard.

Evidence: `tests/historical_gaussian.rs`, `tests/c_api_gaussian_serial.rs`,
C/Python codec callers, unchanged original current fixture and exact extracted
original model byte span. Both source read/write rows are verified on all five
targets. WASM and complete model/stream/data codec interfaces remain pending.

## Stream parameter and builder C fields

`lrh_stream.weight`, `ngmm` and `gmms` map to precision-specific layout/weight
operations, checked emission indices and deep Gaussian child copies. All four
Gaussian fields and both shape values remain readable/editable through the existing
Gaussian-owner interface. Different emissions may have different shapes; borrowed
input arrays may repeat an owner without sharing stored mutable data. Complete
replacement can change stream shape and retains prior storage on failure.

Incremental builders represent the original null Gaussian slots with native optional
values. Presence, get/set/clear and build expose populated and unfilled states. Build
validates a snapshot and keeps the builder intact. Empty streams/builders and native
flat-start weights are retained. Owners, builders and returned children have separate
release functions and independent storage; no internal pointer is exported.

Evidence: both precision boundary tests in `tests/c_api_stream.rs`, optimized
assert-enabled C/Python callers and all five native target suites. The two constructor
rows are complete; full stream deletion/preparation/statistics/codecs, model/data/
inference/training/generation/pool interfaces and all WASM remain pending.

## Model parameter and builder C fields

`lrh_model.streams`, `nstream`, `durations` and `nduration` map to the complete
model layout and checked independent child get/set operations. Borrowed arrays
are deeply copied and can contain repeated owners. Model children preserve all
stream weights/counts, variable Gaussian shapes and all six duration fields.
Complete replacement can resize both arrays atomically. Flat-start specifications
expose all original per-stream emission/component/dimension counts.

Incremental model builders map original null stream/duration slots to native
optional values. Both slot families expose presence, copy, replacement and clear.
Build validates a snapshot and leaves the source intact. Every returned model,
builder and child copy owns independent storage and has a clearing release entry
point. Empty and one-family models keep native behavior.

Evidence: both precision tests in `tests/c_api_model.rs`, the original C construction
fixture, optimized assert-enabled C/Python callers and all five native target suites.
Both constructor rows are complete; owned whole-model preparation/deletion,
statistics/codecs and remaining data/inference/training/generation/pool interfaces,
all WASM and combined acceptance remain pending.

## Stream and model codec C interfaces

Complete stream/model children and fields retain the native current and historical
MessagePack layouts in both storage precisions. The twenty optional entry points
share explicit encoding selectors, caller-selected cumulative array limits and
borrowed byte/I/O adapters. One-object prefix readers preserve original C sequential
reader semantics; strict alternatives provide explicit trailing-data checks.

Evidence: `tests/c_api_stream_serial.rs`, `tests/c_api_model_serial.rs`, the unchanged
full current/historical models, current C stream and exact extracted historical
stream, `tests/historical_stream.rs`, optimized assert-enabled C/Python callers and
all five native target suites. All four source reader/writer rows are verified.
Whole-model preparation/deletion, stream/model statistics, data/inference/training/
generation/pool interfaces, all WASM and combined acceptance remain pending.

## Stream sufficient-statistics C fields

`lrh_stream_stat.gmms` and `ngmm` map to checked emission counts/child get/set and
complete create/replace operations. Every Gaussian accumulator field and shape
remains accessible through independent Gaussian-statistics owners. Mixed shapes,
repeated child inputs and arbitrary scalar bits are retained until native update
validation. Clones and from-stream accumulators retain no source borrow.

Clear resets all native accumulator values without changing shape; release frees
every nested vector and clears its ownership slot. Stream update delegates native
transactional maximization and exposes all existing typed update options.

Evidence: four tests in `tests/c_api_stream_statistics.rs`, the native empty-option
regression test, both original C update corpora and optimized assert-enabled C/Python
callers on all five native targets. The four statistics/update source rows are
complete. Model statistics/preparation/data/inference/training/generation/pool
interfaces, all WASM and combined acceptance remain pending.

## Model sufficient-statistics C fields

The complete stream and duration accumulator collections map to independent
child counts, checked get/set operations and complete create/replace functions.
All stream/Gaussian accumulator fields and all three duration accumulator fields
remain available through the child interfaces. Getters, clones and input copies
retain no source borrow. Empty and single-family collections are supported.

Clear delegates native zeroing while retaining nested shapes. Release drops
every owned child/vector and clears its ownership slot. Model update delegates
native transactional maximization with every typed update setting; a late
duration failure preserves the complete model.

Evidence: four tests in `tests/c_api_model_statistics.rs`, both original C update
corpora with 90 compared values through whole-model updates, C/Python callers
invoking all 24 new exports, all five native target suites and host callers with
serialization disabled. The four model-statistics/update source rows are
complete. Model/stream preparation, data/inference/training/generation/pool/
configuration interfaces, all WASM and combined acceptance remain pending.

## Prepared stream/model C fields and ownership

All model/stream Gaussian normalizers and duration cache fields are accessible
through copied prepared child owners. Full parameter copies and checked layouts
retain stream weights, collection counts and every nested shape and scalar field.
Both precisions expose all preparation settings and selected-duration availability.
Prepared children retain their caches after source or parent release.

Whole-model preparation supports all or selected duration states, duplicates and
empty selections. Unselected uninitialized parameters remain stored unchanged.
Normalizers/density/cache access delegates existing child interfaces without
recomputing derived state. Complete model/stream deletion mappings include editable
parameters, partially filled builders and independent prepared snapshots.

Evidence: `tests/c_api_prepared_stream.rs`, `tests/c_api_prepared_model.rs`, all
36 functions invoked by C/Python callers, all five native targets, host tests and
callers with serialization disabled, and unchanged original C corpora. Rust
stream boundary tests compare 166 original C cache/density values. Native math,
fixtures and existing child functions remain unchanged. The source precompute
and model/stream deletion rows are complete; data/inference/training/generation/
pool/configuration interfaces, all WASM and final combined acceptance remain pending.

## Observation stream C fields

Observation-stream dimensions and complete time-major scalar storage map to a
checked shape descriptor and copy/set operations. Frame count is derived from
the complete dimensions/value length. Independent clones and complete replacement
preserve feature bits without retaining caller buffers. Range/frame access and
zeroed construction are verified in both precisions and all five native targets.

Evidence: `tests/c_api_observation_stream.rs`, the unchanged original C observation
fixture, assert-enabled C/Python callers invoking all 20 exports with exact scalar
bit checks, and host tests/callers with serialization disabled. Complete observation
and segmentation source rows, codecs and dataset operations remain pending; no
original C source row is completed solely by this observation-stream interface.

## Complete observation C fields and indexing

All `lrh_observ` fields have verified C access in both precisions. Shared frame
and stream counts use `LiblrhsmmRsObservationLayout`; each stream's dimensions
and time-major storage use its layout and scalar/frame copy/set functions.
Native zeroed construction, complete copied construction/replacement, independent
stream getters/setters, deep cloning and release preserve full shapes and bits.
Different dimensions and repeated child inputs are supported while all streams
must match the shared frame count. The `lrh_obm` macro's time/stream/dimension
access is available through complete frame vectors or scalar ranges.

Evidence: `tests/c_api_observation.rs`, the unchanged original C observation
fixture, assert-enabled C/Python callers invoking all 26 functions, all five
native targets and host tests/callers without serialization. Copies survive
source/parent release, failed replacements preserve every original stream, and
arbitrary feature bits remain unchanged. No native arithmetic, wire schema or
fixture changes. Observation construction/deletion rows are complete; observation
codecs, segmentation/topology, sets/datasets and the remaining interfaces,
all WASM and final combined acceptance remain pending.

## Observation codec C verification

Complete observations retain every decoded frame/stream/dimension/feature field
through strict byte/callback readers, one-object callback prefix readers and
native byte/callback writers in both precisions. C read/write mappings use the
unchanged binary32 feature wire schema. Native strict budgeted reading adds no
new decoder or schema. Shared byte owners and synchronous I/O descriptors are
reused; callback contexts are never retained.

Evidence: `tests/observation_limits.rs`, `tests/c_api_observation_serial.rs`,
assert-enabled C/Python callers invoking all ten exports and reproducing the
unchanged original C observation bytes, all five native targets, native-only
limits tests and serialization-disabled host byte/I/O callers that verify the
new optional exports are absent. Exact cumulative budget, every truncation,
sequential boundaries, short/interrupted I/O and errors are covered. Rust boundary
tests additionally guard against EOF reads in prefix mode and verify late write
failure after seven bytes. Observation read/write rows are complete; sample sets,
segmentation/topology, datasets and remaining C/WASM interfaces stay pending.

## C segmentation parameter coverage

The optional C boundary exposes both precisions of every editable segmentation
parameter field, stream/segment counts, outgoing deltas/probabilities and ragged
group lengths. Full construction/replacement, independent row copies, interval
setters/copies, jump insertion, resegmentation, deep parameter clones and release
have executable Rust and actual C/Python coverage. The 34 exports preserve stored
scalar bits and failed-operation state; four Rust tests include unchanged C
segmentation/copy/resegmentation fixture comparisons. All five native targets
pass 66 suites /207 tests and all20 C caller families; all20 Python caller
families pass the three64bit targets. No-serialization coverage passes as well.

Current header totals are441 exports /31 descriptors /35 owners. Source inventory
is57 implemented /46 pending. Complete incoming-cache C ownership/copying remains
pending. In particular, lrh_seg_copy copies incoming caches in upstream data.c;
the parameter clone alone does not complete that source mapping. lrh_seg_buildjumps,
graph/cache integration into inference, segmentation wire codecs, sets/datasets,
other remaining C families, every WASM mapping and final combined acceptance
remain required. No standalone segmentation start/end fields exist upstream.

## Complete C transition graph cache ownership

Independent immutable graph owners now retain every segmentation parameter and
derived incoming jump group in both precisions. The18 new C exports build once,
copy complete cached graphs without recomputing incoming transitions, expose
counts and interval copies for incoming/outgoing groups, return independent
editable parameter copies and release unique graph owners. Source/child edits or
release cannot invalidate the snapshot or independent cached clones.

Two Rust tests and actual C/Python callers check complete fields, unchanged16-record
binary32 C transition output, source and parent release, failed-output retention,
raw selected-precision bits and empty graphs. The f64 C corpus comparison uses
exact widening of binary32 values, not independent C double results. All5 targets
pass67 suites /209 tests and all21 C caller families; all21 Python families pass
the3 64bit targets. New tests and actual callers also pass without serialization.

Header totals are459 exports /31 descriptors /37 owners. Inventory is59 complete
C mappings /44 pending. Complete incoming-cache copying now covers lrh_seg_copy;
lrh_seg_buildjumps is also complete. Existing inference still constructs local
graphs; TransitionGraphView integration and remaining segmentation/data codecs,
sets/datasets, other C families, all WASM and final combined acceptance remain
required. Cached incoming storage remains immutable to prevent inconsistent graphs.

## Native graph cache reuse in inference

Forward/backward/Viterbi and duration occupancy can now borrow TransitionGraphView
directly from either graph kind, retaining the existing numerical kernels and
legacy validation order. Both borrowed and owned prepared models reuse their
existing duration caches; owned models support all three HSMM operations with
ordinary segmentations or existing graph views. No parameter/cache copies or
graph construction occur on the cached input branch. Four tests compare every
score/path/posterior bit across Rust paths, check invalid inputs and independently
retain original float/double/fractional HSMM and corrected duration C corpora.

All5 targets pass68 suites /213 tests and all21 existing C caller families;
all21 Python families pass the3 64bit targets. Native-only and no-serialization
tests pass. Header459/31/37, C59/44 inventories and manifests remain unchanged.
Native cache reuse is verified; usable C inference/score/path/posterior interfaces
remain pending, as do remaining codecs/data families, every WASM mapping and final
combined acceptance. Shared immutable cache storage remains valid while views
are live; the existing three borrow gates remain verified.

## Typed inference tables and likelihood totals

The new inference_tables boundary adds52 exports,26 per precision, six opaque
owners and one layout descriptor. Complete copied imports and replacements cover
log emissions and scores; log-to-density conversion creates distinct density
owners. All tables expose layout, element/frame/interval copies, deep cloning and
unique release. No internal storage pointer escapes. Failed replacement preserves
the owner; copying and cloning preserve selected-precision bits. Native score
import validates only positive matching shape; log import retains native finite
validation. Density import and model-to-emission computation remain future work.

All four original lrh_total mappings are complete through the three native total
methods. Unchanged original C forward/backward score arrays and14 total records
per precision are checked through the Rust ABI tests. Actual C/Python callers
exercise all52 symbols, ownership, values and errors; their forward-total
expectations use the previously verified log-add interface. Their own caller
checks do not load the original total corpus. See [C API evidence](c-api.md#typed-inference-tables-and-likelihood-totals).

All five native targets pass70 suites /218 tests,22 C caller families and the
three borrow gates. All22 Python families pass the three64bit targets. Feature
variants, Clippy/Rustdoc with warnings denied and generated-header verification
pass. Header511 exports /32 descriptors /43 owners; C63 complete /40 pending;
every WASM entry remains pending. C inference paths, posteriors and the remaining
public bindings are still required. Evidence is Debug/shared, not Release/static
or Python32.

## Complete geometric and HSMM inference results

All geometric and HSMM forward/backward/Viterbi mappings now have C calls in both
precisions. HSMM supports raw/prepared models and segmentation/cached-graph paths,
reusing immutable Native caches. Six owners preserve full score tables, occurrences,
frame states, boundaries and likelihood. Four settings descriptors expose every
Native field/default; one layout descriptor gives complete path dimensions.
Child results and clones own independent storage; no internal pointer escapes.
Score-pair import preserves arbitrary scalar bits and rejects mismatched shape.

Four ABI tests compare all native result bits,4 scenarios and3 cache lengths per
precision. Actual C/Python callers use all70 new functions and compare768 float /
1536 double HSMM scores and all path occurrences from unchanged original ordinary/
fractional C corpora. Geometric native bit comparisons are in Rust ABI tests.
See [full C evidence](c-api.md#complete-geometric-and-hsmm-inference-results).

Five targets pass71 suites /222 tests and all23 C families; all23 Python families
pass three64bit targets. Feature variants, quality checks and generated-header
verification pass. Header581 exports /37 descriptors /49 owners. C69 complete /
34 pending; all WASM entries pending. Native algorithms, fixtures, formats and
dependencies remain unchanged. Evidence is Debug/shared, not Release/static or
Python32; the remaining public bindings and combined final acceptance are required.

## Weighted multistream emission computation

All three original sample_outputprob mappings now have complete C access through
raw/owned-prepared model log/density methods in both precisions. Ten exports,
two complete settings descriptors and three pruning constants use existing table
owners. All native modes, weighted mixed-layout streams and pruning boundaries
are retained. Prepared calls borrow owned normalizers directly; independent result
storage survives input release. No native algorithm, format or dependency changed.

Rust ABI tests compare raw/prepared Native bits across all modes and temperatures,
including selected-precision values beyond1.0. Actual C/Python callers use all10
functions and compare432 unchanged original C log values and36 float densities.
Double density evidence is native bit comparison, not a new independent C corpus.
See [complete evidence](c-api.md#weighted-multistream-emission-computation).

Five targets pass72 suites /224 tests,24 C families and the three borrow gates;
all24 Python families pass three64bit targets. Feature variants and quality/header
verification pass. Header591 exports /39 descriptors /49 owners; C72 complete /
31 pending; every WASM entry pending. Remaining public bindings and final combined
acceptance are required. Debug/shared evidence only, not Release/static/Python32.


## Complete posterior storage and computation

All state/duration/transition/mixture posterior storage and calculations are
accessible through copied independent owners in both model precisions. Sparse
weights retain binary32 storage and distinguish absent from calculated empty
entries. Six original C mappings complete; no native algorithms/formats changed.
See [complete evidence](c-api.md#complete-posterior-storage-and-computation).

Five targets pass73 suites /229 tests and25 C families; all25 Python families pass
three64bit targets. All51 new exports are called, feature variants and quality
checks pass, and original/corrected C corpus regressions remain intact. Header642
exports /42 descriptors /54 owners; C78 complete /25 pending. Every WASM entry and
the remaining bindings/final combined acceptance remain required. Debug/shared
evidence only, not Release/static/Python32.


## Complete initialization, collection and estimation

Complete initialization, full posterior collection and HSMM/geometric estimators
are accessible in both precisions through existing independent statistics owners.
Every native statistic and likelihood is retained, with staged failure behavior.
External mixture arrays are copied for the existing native contiguous-slice API.
See [complete evidence](c-api.md#complete-initialization-collection-and-estimation).

Five targets pass74 suites /233 tests and26 C families; all26 Python families pass
three64bit targets. All10 new exports and update/codec/inference pipelines are
called; feature variants and quality checks pass. Legacy binary32 persistence is
retained in both precisions; decoded inference is compared to the native decoder.
Header652 exports /42 descriptors /54 owners; C83 complete /20 pending. Every WASM
entry, remaining bindings and final combined acceptance remain required.
Debug/shared evidence only, not Release/static/Python32.


## Sampling and complete observation generation

Both precisions expose scalar sampling, component selection and complete
multistream generation through synchronous caller-owned random-word or scalar
callbacks. Failure retains the sequence and result; consumed draws are not
restored. Observation owners remain independent after input release.
See [generation contracts and evidence](c-api.md#sampling-and-complete-observation-generation).

Five targets pass75 suites /239 tests and27 actual C families; all27 Python
families pass three64bit targets. All10 new exports, all three original C corpora
and callback transactions are verified. Feature variants and quality checks pass.
Header662 exports /45 descriptors /54 owners, two callback aliases, ABI1.
C87 complete /16 pending; every WASM entry and final combined acceptance remain
required. Debug/shared evidence only, not Release/static/Python32.


## Segmentation serialization

Both precisions expose strict byte/stream decoding, one-object prefix reading
and byte/stream writing through existing owners and borrowed I/O callbacks.
Every label and jump is retained, including both original array-header mismatches
and binary32 wire probabilities. Errors preserve constructor slots; partial
writers may have emitted a prefix. See [codec contracts and evidence](c-api.md#segmentation-serialization).

Five targets pass76 suites /243 tests and28 actual C families; all28 Python
families pass three64bit targets. All10 new exports, six original/corrected C
fixtures, complete fields, exact wire and callback/error transactions are covered.
Feature variants and quality checks pass. Header672 exports /45 descriptors /
54 owners, ABI1. C89 complete /14 pending; every WASM entry, remaining bindings
and final combined acceptance remain required. Debug/shared evidence only,
not Release/static/Python32.


## Memory pools and allocation identities

Native page growth, complete accounting and shared region identities are exposed
through two independent owners. Checked copies/replacements retain byte contents
on errors. Allocation release keeps pool storage; pool release frees pages while
remaining region identities retain metadata. No internal storage pointer escapes.
See [pool contracts and evidence](c-api.md#memory-pools-and-allocation-identities).

Five targets pass77 suites /245 tests and29 actual C families; all29 Python
families pass three64bit targets. All11 new exports, original C growth/checksum
corpus and shared identity/failure behavior are verified. Feature variants and
quality checks pass. Header683 exports /47 descriptors /56 owners, ABI1.
C93 complete /10 pending; every WASM entry, remaining bindings and final combined
acceptance remain required. Debug/shared evidence only, not Release/static/Python32.


## Complete sample sets and missing builder slots

Seventy-two unconditional exports expose complete ordered observation and
segmentation sets and builders in both precisions. All sample fields, repeated
children, heterogeneous shapes and missing slots survive copies. Builders retain
missing slots until explicitly filled, cleared or resized, and validated builds
preserve the caller's builder. Set replacement stages complete input validation
and copies before publication. Children are independent of all source lifetimes.
Present empty samples retain their stream shapes and remain distinct from missing
slots. See [contracts and evidence](c-api.md#complete-sample-sets-and-incremental-builders).

Five targets pass78 suites /249 tests and30 actual C caller families. All30 Python
families pass three64bit targets. All72 exports, complete field/bit comparisons,
partial builder clones, present empty samples and retained output/error cases are
verified. Feature variants and quality checks pass. Header755 exports /
47 descriptors /64 owners, ABI1. C98 complete /5 pending; every WASM entry and
final combined acceptance remain required. Debug/shared evidence only, not
Release/static/Python32.

## Builder length audit correction

Four additional C ABI exports provide stream/model builder resize at both
precisions. The generated header now contains 791 function declarations;
comparison against the previous 787-export header finds exactly the intended
four additions and no removed declarations. No descriptor, owner or ABI version
changes are required. The public API field audit exposed this gap after the
upstream callable inventory had complete mappings.

Actual assertion-enabled optimized C and Python shared-library callers exercise
the new exports on Windows x86_64 with and without serialization. The earlier
five-target validation does not cover these new additions; their cross-platform
checks remain required in the final matrix. WASM now has mapped interfaces for
all 103 upstream entries, but the full public API audit and final combined
acceptance remain open. See `public-api-audit.md` for the verified scope.


## Complete paired dataset regrouping

Twelve unconditional exports and two owners expose complete paired regrouping in
both precisions. Input sets are borrowed directly by the native implementation;
one owner publishes both result sets after success. All features, labels, final
short groups and ordinary regrouped jumps are available through independent
editable result sets. Empty aligned samples and repeated boundaries are supported.
Errors retain initialized outputs; retrieved sets and children outlive all inputs
and dataset owners. See [contracts and evidence](c-api.md#complete-paired-dataset-regrouping).

All five targets pass 79 suites and 251 tests and all 31 actual C caller families.
All 31 Python families pass the three 64-bit targets. All 12 new exports, complete
result fields, 150 corrected-C double records, six unit sizes and error/lifetime
checks are verified. Feature variants and quality checks pass. The header contains
767 exports, 47 descriptors and 66 owners, with ABI version 1. C99 complete /
4 pending; every WASM entry and final combined acceptance remain required.
Debug/shared evidence only, not Release/static/Python32.


## Complete observation and segmentation set serialization

Twenty optional exports retain complete ordered samples and the original wire
schemas in both precisions. Strict decoding rejects trailing bytes; prefix
decoding leaves subsequent objects unread without an EOF request. Cumulative
limits cover the outer set and all nested arrays. Source f64 precision survives
writing, while serialized values retain the original binary32 representation.
Synchronous callbacks support short operations and interruption retry; failed
decoding retains outputs, and failed writes may emit a prefix. See
[contracts and evidence](c-api.md#complete-observation-and-segmentation-set-serialization).

All five targets pass 80 suites and 257 tests and all 32 C caller families.
All 32 Python families pass the three 64-bit targets. All 20 new exports,
complete original C set fields/wire, nested budget boundaries, every truncation,
empty sets/children, selected-precision round trips and callback transactions are
verified. Feature variants and quality checks pass. The generated header contains
787 exports, 47 descriptors and 66 owners, with ABI version 1. C caller symbols
cover all 787 exports, including both invoked numerical macro expansions.
All 103 original public liblrhsmm C mappings are complete; every WASM entry and
final combined acceptance remain required. Debug/shared evidence only, not
Release/static/Python32.
