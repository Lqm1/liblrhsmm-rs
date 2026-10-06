# Intentional differences from the C implementation

Duration occupancy initializes the first state at frame zero with log probability zero. Upstream tests `i == 0` after a do/while loop which has already incremented `i`, making its initialization branch unreachable. The corrected reference differs by exactly one line, recorded in `tests/reference/duration-initialization.patch`. All other operations in that reference remain unchanged. Rust preserves the final absorbing state's exclusion from duration learning.

An empty duration search range produces an absent posterior slice. Upstream allocates a zero-length slice and then accesses its first element while trimming, which is an invalid memory access.

HSMM backward checks `0 <= destination < state_count`. The upstream check compares a state index against frame count and omits the lower bound. Rust preserves valid jumps when there are more states than frames and excludes negative/out-of-range jumps safely. A regression uses four states and two frames with a jump to state three, which the C check incorrectly excludes.

HSMM results return owned begin/end score tables. The upstream optional Viterbi score-output branch copies into the pointer-to-pointer argument itself instead of its pointed-to buffer. Rust avoids that invalid memory write. HSMM traceback also reports an unreachable final state rather than following uninitialized predecessor data.

Zero-weight HSMM transitions are excluded even at temperature zero, and minimum-duration exclusions remain active at temperature zero. This avoids `log(0) * 0` and prevents temperature scaling from turning the finite impossible-duration sentinel into a permitted score.

Geometric Viterbi reports an error if no complete first-to-last-state path survives pruning. The original traceback silently follows zero-initialized predecessors when a state is unreachable, producing a boundary array despite the absence of a valid path. The HMM variant continues to use only stay/next-state transitions, as upstream does; arbitrary jumps belong to the explicit-duration inference path.

Initialized density evaluation rejects nonfinite means or observations, nonpositive variances, and negative/nonfinite weights. An uninitialized model remains representable and serializable, but cannot be prepared for inference. The C implementation can return NaN or replace it with the arbitrary component score -999.

Prepared GMMs borrow their source distribution immutably. Parameters cannot change while a normalizer cache is in use, preventing the stale-cache condition permitted by the original mutable C objects.

A zero DAEM temperature returns a zero log density for a valid observation, including finite observations whose squared residual overflows. This avoids the undefined log score produced by multiplying negative infinity by zero.

The scalar exponential compatibility kernel returns positive infinity above its representable range. Upstream converts an out-of-range floating-point value to an unsigned integer, for which C does not define the result. Values within the defined range retain the original approximation and rounding.

Emission sampling rejects invalid dimensions, state indices, and unordered/out-of-range boundaries before evaluation. The original functions use unchecked indexing. Boundary pruning requires the final boundary to equal the observation frame count. Linear or unpruned sampling can operate without an existing alignment.

Fixed-mean normal duration updates compute E[(X - fixed_mean)^2]. Upstream substitutes the fixed mean into E[X^2] - mean^2, which is only valid when the mean equals the sample mean. For observed durations 2 and 4 with a fixed mean of 4, Rust returns variance 2; upstream clamps its erroneous negative variance to the configured floor. This correction has an independent squared-residual regression.

Geometric duration updates reject zero transition occupancy when state occupancy is positive, instead of storing infinity. Duration, Gaussian, stream, and model updates validate statistics and commit parameters only after successful calculation. Initialization also accumulates atomically and rejects nonfinite or overflowing statistics. Upstream can leave partially modified statistics when encountering a multi-component emission during initialization; Rust retains that operation's single-component restriction and reports the error before committing.

Initialization squares duration lengths in floating-point storage instead of signed C integer arithmetic. A 50,000-frame regression verifies a squared duration of 2,500,000,000, beyond the signed 32-bit range. Ordered boundaries may include zero-length segments, and repeated duration/emission states accumulate as upstream does.

Duration preparation belongs to each Rust inference call and its explicit temperature options. Upstream lrh_model_precompute only rebuilds its duration lookup when the normalizer sentinel is unset, even though the lookup also includes the global annealing temperature. Reusing a C model after changing temperature can therefore retain old probabilities. The estimation reference resets that sentinel before each independent case without changing the C algorithms. A regression compares Rust's temperature-1 statistics before and after a temperature-0.5 call on the same model.

Dataset regrouping uses each group's preceding right boundary as its first frame and its final state's right boundary as its end. Upstream starts at the first state's right boundary, skipping that state's frames and leaving a zero first boundary. It also allocates floor(segment_count / unit_size) samples while visiting ceil(segment_count / unit_size) groups and always copies unit_size states, causing out-of-range accesses for the final short group. Rust keeps all states and frames, includes a final short group, and validates alignment and stream counts before slicing. The corrected C comparison is disclosed in tests/reference/data-regroup.patch. Source state jumps are reset to ordinary next-state transitions as in upstream regrouping and resegmentation.

AddressSanitizer reproduces the unmodified regrouping defect with the five-state/two-state-unit oracle. It reports a heap-buffer-overflow write in data.c:234, immediately after a two-pointer observation-set allocation. The corrected data.c reference completes the same driver with exit code zero and no AddressSanitizer diagnostics. Only data.c and the driver are instrumented in this check; the pinned static archive retains its original Debug build.

Generation accepts caller-owned RNGs or an explicit uniform-draw callback. It uses rand's OpenClosed01 distribution for ordinary RNG calls, avoiding the original possible zero draw and log(0) in Box-Muller sampling. The explicit callback reports zero radial draws and nonfinite/out-of-range values as errors. Empty sequences and invalid model dimensions/states/parameters are rejected before consuming draws. Successful sampling overwrites right boundaries; any failure preserves the sequence. Consumed random draws are not rolled back. Duration constraints and jump probabilities remain unused by generation, matching upstream. No C-library-specific global rand sequence is promised.
