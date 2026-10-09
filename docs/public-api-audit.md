# Public API audit

This audit supplements the upstream-symbol inventory. It does not infer complete
coverage from symbol counts. The remaining model/data/statistic field, prepared
view, codec, generic precision and public-module audits are open.

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
