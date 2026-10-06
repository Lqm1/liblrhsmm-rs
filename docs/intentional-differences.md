# Intentional differences from the C implementation

HSMM backward checks `0 <= destination < state_count`. The upstream check compares a state index against frame count and omits the lower bound. Rust preserves valid jumps when there are more states than frames and excludes negative/out-of-range jumps safely. A regression uses four states and two frames with a jump to state three, which the C check incorrectly excludes.

HSMM results return owned begin/end score tables. The upstream optional Viterbi score-output branch copies into the pointer-to-pointer argument itself instead of its pointed-to buffer. Rust avoids that invalid memory write. HSMM traceback also reports an unreachable final state rather than following uninitialized predecessor data.

Zero-weight HSMM transitions are excluded even at temperature zero, and minimum-duration exclusions remain active at temperature zero. This avoids `log(0) * 0` and prevents temperature scaling from turning the finite impossible-duration sentinel into a permitted score.

Geometric Viterbi reports an error if no complete first-to-last-state path survives pruning. The original traceback silently follows zero-initialized predecessors when a state is unreachable, producing a boundary array despite the absence of a valid path. The HMM variant continues to use only stay/next-state transitions, as upstream does; arbitrary jumps belong to the explicit-duration inference path.

Initialized density evaluation rejects nonfinite means or observations, nonpositive variances, and negative/nonfinite weights. An uninitialized model remains representable and serializable, but cannot be prepared for inference. The C implementation can return NaN or replace it with the arbitrary component score -999.

Prepared GMMs borrow their source distribution immutably. Parameters cannot change while a normalizer cache is in use, preventing the stale-cache condition permitted by the original mutable C objects.

A zero DAEM temperature returns a zero log density for a valid observation, including finite observations whose squared residual overflows. This avoids the undefined log score produced by multiplying negative infinity by zero.

The scalar exponential compatibility kernel returns positive infinity above its representable range. Upstream converts an out-of-range floating-point value to an unsigned integer, for which C does not define the result. Values within the defined range retain the original approximation and rounding.

Emission sampling rejects invalid dimensions, state indices, and unordered/out-of-range boundaries before evaluation. The original functions use unchecked indexing. Boundary pruning requires the final boundary to equal the observation frame count. Linear or unpruned sampling can operate without an existing alignment.
