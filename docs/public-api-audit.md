# Public API audit

This audit supplements the upstream-symbol inventory. It does not infer complete
coverage from symbol counts. The remaining data field, prepared view, codec,
generic precision and public-module audits are open. Model/statistic field
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
