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
| `calculate_maxdur`, `for_tj_forward`, `for_tj_backward`, `end_for_tj` | HSMM duration bounds and pruning loops; fractional radius regression coverage exists, complete use-site audit pending |
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
No overall macro/configuration
completion claim follows from this table. Source debug/release options, remaining
internal inference headers, final combined native acceptance and bindings remain
separate open requirements.

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
