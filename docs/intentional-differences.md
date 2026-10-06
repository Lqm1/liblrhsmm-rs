# Intentional differences from the C implementation

Geometric Viterbi reports an error if no complete first-to-last-state path survives pruning. The original traceback silently follows zero-initialized predecessors when a state is unreachable, producing a boundary array despite the absence of a valid path. The HMM variant continues to use only stay/next-state transitions, as upstream does; arbitrary jumps belong to the explicit-duration inference path.

Initialized density evaluation rejects nonfinite means or observations, nonpositive variances, and negative/nonfinite weights. An uninitialized model remains representable and serializable, but cannot be prepared for inference. The C implementation can return NaN or replace it with the arbitrary component score -999.

Prepared GMMs borrow their source distribution immutably. Parameters cannot change while a normalizer cache is in use, preventing the stale-cache condition permitted by the original mutable C objects.

A zero DAEM temperature returns a zero log density for a valid observation, including finite observations whose squared residual overflows. This avoids the undefined log score produced by multiplying negative infinity by zero.

The scalar exponential compatibility kernel returns positive infinity above its representable range. Upstream converts an out-of-range floating-point value to an unsigned integer, for which C does not define the result. Values within the defined range retain the original approximation and rounding.

Emission sampling rejects invalid dimensions, state indices, and unordered/out-of-range boundaries before evaluation. The original functions use unchecked indexing. Boundary pruning requires the final boundary to equal the observation frame count. Linear or unpruned sampling can operate without an existing alignment.
