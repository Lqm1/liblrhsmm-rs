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
| `lrh_seg.djump_in`, `pjump_in` | `TransitionGraphParameters<'_, T>::incoming()`, built from the outgoing topology |
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
