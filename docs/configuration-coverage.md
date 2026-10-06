# Upstream configuration coverage

The public globals in the pinned upstream common.h are configuration inputs.
Rust passes them per call rather than sharing mutable process-global state.
Defaults below are taken from upstream common.c.

| Upstream global | Default | Rust input | Evidence |
| --- | --- | --- | --- |
| lrh_duration_vfloor | 0.05 | UpdateOptions::duration_variance_floor | tests/maximization.rs |
| lrh_output_vfloor | 0.01 | UpdateOptions::emission_variance_floor | tests/maximization.rs |
| lrh_inference_duration_weight | 1.0 | HsmmOptions::duration_weight; Duration density weight argument | tests/hsmm_inference.rs; tests/duration_probability.rs |
| lrh_inference_stprune | 5 | HsmmOptions::state_radius; EmissionPruning::AroundBoundaries::radius; HsmmExpectation::state_radius | tests/hsmm_inference.rs; tests/emission_table.rs; tests/estimation.rs |
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

The upstream lrh_inference_stprune global is floating-point. The current Rust
state_radius and boundary-pruning radius accept integer radii. Fractional-radius
behavior in inference-helper.h still needs implementation and differential
verification; the mapping above does not establish full fractional coverage.

The C LRH_UNINITIALIZED macro is an internal cache sentinel. Rust prepared
objects represent completed preparation explicitly and borrow parameters to
prevent changes while their caches are used. The upstream min/max macros are
internal expression helpers; model computations validate finite parameters
before using Rust comparisons. This document does not establish complete macro
or structure coverage across the remaining headers.
