# Upstream configuration coverage

The public globals in the pinned upstream common.h are configuration inputs.
Rust passes them per call rather than sharing mutable process-global state.
Defaults below are taken from upstream common.c.

| Upstream global | Default | Rust input | Evidence |
| --- | --- | --- | --- |
| lrh_duration_vfloor | 0.05 | UpdateOptions::duration_variance_floor | tests/maximization.rs |
| lrh_output_vfloor | 0.01 | UpdateOptions::emission_variance_floor | tests/maximization.rs |
| lrh_inference_duration_weight | 1.0 | HsmmOptions::duration_weight; Duration density weight argument | tests/hsmm_inference.rs; tests/duration_probability.rs |
| lrh_inference_stprune | 5 | HsmmOptions::state_radius; EmissionPruning::AroundBoundaries::radius; HsmmExpectation::state_radius | tests/hsmm_inference.rs; tests/emission_table.rs; tests/estimation.rs; tests/fractional_pruning.rs |
| lrh_inference_stprune_full_slope | 0.3 | GeometricOptions::pruning_slope; EmissionPruning::Linear::slope; GeometricExpectation::pruning_slope | tests/geometric_inference.rs; tests/emission_table.rs; tests/estimation.rs |
| lrh_inference_duration_extra | 30 | HsmmOptions::duration_extra | tests/hsmm_inference.rs |
| lrh_inference_duration_extra_factor | 1.0 | HsmmOptions::duration_extra_factor | tests/hsmm_inference.rs |
| lrh_daem_temperature | 1.0 | HsmmOptions::temperature; GeometricOptions::temperature; density and emission temperature arguments | tests/estimation.rs; tests/duration_probability.rs; tests/emission_probability.rs |
| lrh_precompute_duration | 300 | PreparationOptions::duration_cache_length | tests/preparation.rs; tests/reference/preparation_oracle.c |

Model::prepare provides reusable whole-model emission normalizers and duration
lookup tables. Cache length zero disables duration table storage while retaining
the prepared normalizer. Values beyond the cache are evaluated with that same
normalizer. Prepared HSMM forward/backward/Viterbi calls require temperature and
duration weight to match preparation; emission normalizers can be reused at
different temperatures.

The upstream lrh_inference_stprune global and original Rust state_radius/
boundary pruning APIs default to binary32. Parameterized settings also retain
binary64 radii for double builds. Shared bounds preserve C's lower-bound
truncation and inclusive upper comparison, including asymmetric fractional
radii. Tests compare radii 0.25, 0.5, 1.75, and 2.5 across emissions, inference,
posteriors, and expectation collection. Negative or nonfinite radii are errors.
The remaining header macros still require audit; this table covers the globals
in common.h. Build precision coverage is recorded separately below.

## Build precision and serialization

The upstream Makefile defaults to `FP_TYPE=float` and supports overriding it
with `FP_TYPE=double`. This changes model parameters, observations, statistics,
probability tables, inference storage, and training arithmetic. Complete
binary64 engine support is a required part of the port and remains unfinished.
Model and observation storage, construction, parameter operations, Gaussian
densities, whole-model preparation, emission tables, transition graphs, and
HMM/HSMM inference, posterior/occupancy processing, statistics and training now
support both precisions as described below. Dataset operations also support both
precisions. Generation and codecs still require precision generalization. These phases do not
close the full binary64 engine requirement.

`numerics::log_add_f64`, `negative_exponential_f64`, and `exponential_f64` now
retain the double-build table coefficients and quantization. The binary64
tables are parsed from the original published decimal literals, not widened
from the binary32 tables or recomputed from analytic functions. The positive
exponential branch deliberately narrows to binary32 because upstream calls
the float-only fastapprox kernel in both precision builds. The source random
Gaussian helper similarly calls float-only `fastcos`; future binary64
generation must account for that behavior.

`tests/generate_numerics_reference.py` compiles unmodified `math-funcs.c` and
the numerical header in both precision modes. Each fixture records all 14,000
table entries and 42,008 helper results, including adjacent floating-point
neighbors of quantization boundaries. `tests/numerics.rs` checks the entire
table payload and every result bit. Independent nonfinite-limit tests cover
the safe Rust behavior where the source would use an invalid table index.

The legacy serializer always calls `cmp_write_float` and reads model and
observation values into a local C `float`, including in double builds. Its
wire representation remains binary32. Binary64 model serialization must
preserve that representation with explicit narrowing and widening rather
than writing MessagePack doubles. The source segmentation jump reader passes
an `FP_TYPE*` directly to `cmp_read_float`, which requires `float*`; this is a
confirmed double-build defect and must be corrected when that reader is
generalized. No Rust binary64 codec is implemented yet.

Upstream `LRHSMM_SERIALIZATION=false` excludes the serialization translation
units. Rust currently always includes serialization; making that dependency
optional and verifying a computation-only build remains an open build audit
item. Debug/release settings and other public definitions remain subject to
the complete acceptance audit.

## Parameter and density precision

`model::DurationParameters<T>`, `GaussianMixtureParameters<T>`,
`StreamParameters<T>`, `ModelParameters<T>`, and their incremental builders
share storage and construction logic for `T = f32` or `f64`. The original
`Duration`, `GaussianMixture`, `Stream`, `Model`, and builder names remain
binary32 aliases; existing callers retain their parameter precision and type
inference. `data::ObservationData<T>`, `ObservationStreamData<T>`,
`JumpParameters<T>`, and `SegmentationData<T>` similarly preserve full selected
precision. The sealed `scalar::Scalar` trait admits only the two original
storage types and makes rounding from C's double expressions explicit.

Duration densities, annealing, Gaussian combination and splitting, and
`probability::PreparedGaussianParameters<'a, T>` share implementations. Selected
precision subtraction, products, division, and accumulation are retained;
double expressions round only where C assigns back to `FP_TYPE`. The source
float-only fastapprox exponential/logarithm calls remain binary32 in double
builds. Prepared Gaussian objects borrow their source parameters, preserving
the existing cache ownership guarantee at either precision.

`tests/precision.rs` compares 72 C records and 196 output values per precision:
duration constraints and three temperatures, repeated five-component splits,
component densities, and complete mixture densities. The source driver uses
unsuffixed decimal inputs so the double case retains more than float input
precision. Additional checks retain `1 + f64::EPSILON` in owned parameters,
builder output, observations and jumps, preserve deep-clone independence, and
verify double flat-start weights. These parameter tests do not establish double
training coverage. Additional inference evidence is recorded below.

The oracle compiler explicitly disables floating-point contraction. Initial
Windows Zig regeneration with its default contraction differed from Linux
GCC by as many as 64 binary32 ulps in a nested fastapprox result. Recompiling
the same unmodified source with `-ffp-contract=off` eliminated the difference;
no fixture or Rust comparison tolerance was changed to accommodate it.
Regeneration checks require exact binary32 outputs and bound binary64 libm
differences separately. This comparison does not promise exact equivalence
to every upstream `-Ofast` compiler configuration.

## Prepared models, emission tables, transitions, and inference

`PreparationSettings<T>`, `PreparedDurationParameters<'a, T>`, and
`PreparedModelParameters<'a, T>` retain selected precision in options,
normalizers and caches. Parameters remain immutably borrowed, with compile-fail
ownership examples at both precisions. Cache lengths zero, three, and 300
support inference. Standalone duration cases also cover length eight and
beyond-cache evaluation. Selected preparation skips unused uninitialized
durations and prepares duplicate selections once.

`EmissionPruningOptions<T>`, `LogEmissionTable<T>`, and
`EmissionDensityTable<T>` retain selected-precision scores, accumulation,
radius and slope. `TransitionGraphParameters<'a, T>` retains jump weights and
ordering while borrowing its segmentation. Original names remain binary32
aliases. Tests distinguish a radius of `1 - f64::EPSILON` from one at an
inclusive integer bound, and distinguish cache temperatures
`1 + f64::EPSILON` from one when checking preparation compatibility.

`GeometricSettings<T>`, `StateScoreTable<T>`, `GeometricPath<T>`,
`HsmmSettings<T>`, `HsmmScoreTables<T>`, and `HsmmPath<T>` share forward,
backward and Viterbi implementations. Quantized log sums dispatch to the
matching scalar table. Geometric transition log terms preserve source double
expressions and selected-precision probability assignments. HSMM jump logs
and duration terms retain source assignment points and fractional pruning.
Prepared HSMM calls require matching temperature and duration weight.

`tests/generate_inference_precision_reference.py` compiles pinned library
sources in double mode with contraction disabled. Existing oracle
drivers are adapted only for FP_TYPE storage, unsuffixed inputs and round-trip
printing. The geometric driver omits posterior calculations. Six ordinary fixtures
cover 238 geometric records, 210 ordinary and 210 fractional HSMM records,
72 duration-cache records, and 72 ordinary and 144 fractional emission records.
Tests compare score values, geometric Viterbi score tables/totals/boundaries,
HSMM occurrence paths, and prepared/unprepared outputs at multiple cache
lengths. The ordinary 10,248 numeric comparisons include repeated checks across cache
configurations; they are not that many independent source values. Two additional
402-record fixtures compare every internal HSMM Viterbi begin/end score under
ordinary and fractional pruning, adding 768 comparisons. Only these score
cases apply the existing tracked `viterbi-score-reference.patch` to a temporary
source copy. That patch corrects the optional output allocation/dereference
and adds a read-only snapshot hook; the driver checks returned begin scores
against the snapshot. The other six cases use unmodified library sources.
All eight source fixtures regenerate on Windows Zig and Linux GCC. The
combined 11,016 numeric comparisons retain the 2e-14 corpus-specific normalized
error criterion; no universal conditioning bound is claimed.

The public C `lrh_pslice.p` member is fixed as `float*` in `inference.h`, even
when FP_TYPE is double. Related pool macros cast to float pointers while
allocating using `sizeof(FP_TYPE)`. This is evidence of mixed storage precision,
not by itself a reproduced correctness defect. Posterior and training APIs now
retain this mixed precision: duration, transition and mixture slices remain
binary32, while state occupancies, sufficient statistics and parameters use the
selected scalar precision. Collection widens slice probabilities before products,
matching the source FP_TYPE local assignments. Duration-slice trimming compares
the widened probability with the source unsuffixed threshold.

`StateOccupancyTable<T>`, `MixtureOccupancySettings<T>`, the duration/Gaussian/
stream/model sufficient-statistics types, posterior-expectation inputs and
`UpdateSettings<T>` support both precisions; existing names remain f32 aliases.
Initialization, expectation and maximization share these implementations.
`tests/precision_training.rs` compares 1,261 selected-precision values, 527
binary32 duration probabilities and 398 binary32 mixture probabilities, plus
transition probabilities. The selected-precision comparisons have zero measured
normalized error in the current corpus, with a 2e-14 acceptance threshold.
Float slices are checked bitwise. The source generator applies the existing
disclosed duration-initialization correction only to duration/estimation cases.
Other cases compile unmodified source. Repeated HMM/HSMM training and inference,
atomic failure handling and corrected fixed-mean residual variance are tested.
These computation tests do not establish binary64 serialization or generation
coverage; those implementations remain outstanding.

`AlignedDataset<T>`, selected-precision sample-set builders, segmentation
resegmentation and `regroup_with_precision` preserve the scalar storage type.
`Dataset` and `regroup_data` retain the original binary32 API, including type
inference for empty input slices. Regrouping copies observations without casts,
preserves partial and empty groups, and resets transitions as in the original
operation. A separate double C oracle applies only the existing disclosed
`data-regroup.patch` to a temporary source copy; 150 records compare sample
counts, boundary/state references and values beyond binary32 precision. The
generator verifies fixture bytes exactly. Independent tests reconstruct all
frames for six group sizes, verify sample independence and reject unfilled slots.

The C LRH_UNINITIALIZED macro is an internal cache sentinel. Rust prepared
objects represent completed preparation explicitly and borrow parameters to
prevent changes while their caches are used. The upstream min/max macros are
internal expression helpers; model computations validate finite parameters
before using Rust comparisons. This document does not establish complete macro
or structure coverage across the remaining headers.

Float-to-integer state-bound conversions follow Rust's truncating, saturating
cast rules after finite, nonnegative validation and storage-bound clamping.
Primary language reference:
https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
