# Upstream configuration binding audit

The nineteen entries in `binding-configuration-inventory.csv` cover nine
numerical constants/tables, nine calculation globals in upstream common.h,
and FP_TYPE. The earlier pending statuses were stale after their individual
interfaces had been implemented. This audit inspects the actual native/C/WASM
conversion paths and identifies computational tests, rather than promoting
statuses from function counts alone.

## Numerical configuration

| Original | C | WebAssembly |
| --- | --- | --- |
| lrh_lse_table_size / lrh_lse_table_step | NumericalConstants.log_add_count / log_add_step | numerical_constants()[0] / [1] |
| lrh_exp_table_size / lrh_exp_table_step | NumericalConstants.exponential_count / exponential_step | numerical_constants()[2] / [3] |
| EPS | NumericalConstants.occupancy_epsilon | numerical_constants()[4] |
| NEGINF | NumericalConstants.log_zero_f32 / log_zero_f64 | numerical_constants()[5] / [6] |
| M_PI | NumericalConstants.pi | Standard JavaScript Math.PI |
| lrh_lse_table / lrh_exp_table | Both-precision table get/copy/replace with selector 0/1 | NumericalTableF32/F64, complete values, live iterators and bit-preserving mutation |

Native Rust uses `std::f64::consts::PI` for the original double macro. The
upstream literal is `3.14159265358979323846`. C and Python callers compare
the returned PI to that literal. `wasm_configuration_checks.mjs` explicitly
checks that JavaScript's standard `Math.PI` matches it. No extra exported
WASM constant is necessary to represent this standard value; callers can
round it to binary32 with `Math.fround` when assigning a binary32 field.

`src/c_api/numerical.rs` and `src/wasm_api/numerics.rs` expose the remaining
constants and mutable tables. The independent original-C binary32/binary64
fixtures test every coefficient and 84,016 numerical helper records in the
WASM numerical family. Table edits are restored before subsequent families.
The explicit PI check does not replace those numerical comparisons.

## Calculation settings

Each row retains both F32 and F64 settings. Native Rust and the bindings
replace calculation globals with explicit per-call inputs; mutable coefficient
tables remain shared within one library/module. See `configuration-coverage.md`
for original defaults and native semantics.

| Original global | Complete public binding input | Computational verifier families |
| --- | --- | --- |
| lrh_duration_vfloor | UpdateOptions.duration_variance_floor | Duration, complete statistics/model updates, estimation |
| lrh_output_vfloor | UpdateOptions.emission_variance_floor | Gaussian, stream/model statistics updates, estimation |
| lrh_inference_duration_weight | PreparationOptions.duration_weight and HsmmOptions.duration_weight | Duration, prepared models, HSMM inference/posteriors/estimation |
| lrh_inference_stprune | HsmmOptions.state_radius, emission/mixture pruning parameter, HsmmExpectation.state_radius | Emissions, HSMM inference/posteriors/estimation |
| lrh_inference_stprune_full_slope | GeometricOptions.pruning_slope, emission/mixture pruning parameter, GeometricExpectation.pruning_slope | Emissions, geometric inference/posteriors/estimation |
| lrh_inference_duration_extra | HsmmOptions.duration_extra | Raw/prepared/graph HSMM inference and posterior/estimation calls |
| lrh_inference_duration_extra_factor | HsmmOptions.duration_extra_factor | Raw/prepared/graph HSMM inference and posterior/estimation calls |
| lrh_daem_temperature | Preparation/Hsmm/GeometricOptions.temperature and explicit density/emission temperature | Duration/Gaussian densities, prepared models, emissions, inference, posteriors and estimation |
| lrh_precompute_duration | PreparationOptions.duration_cache_length | Standalone/whole-model preparation, raw/prepared inference and caches |

C descriptors are `LiblrhsmmRsUpdateOptionsF32/F64`,
`LiblrhsmmRsDurationPreparationF32/F64`, `LiblrhsmmRsHsmmSettingsF32/F64`,
`LiblrhsmmRsGeometricSettingsF32/F64` and
`LiblrhsmmRsEmissionSettingsF32/F64`. Native validation is invoked after
conversion. Emission descriptors retain separate radius/slope fields selected
by pruning mode. WASM settings retain native defaults, editable fields, cloning,
precision-specific values and validation at calculation time.

Source paths are `src/c_api/duration_statistics.rs`, `prepared_duration.rs`,
`prepared_model.rs`, `emission.rs`, `inference.rs`, `posterior.rs` and
`training.rs`, plus the corresponding `src/wasm_api/duration.rs`,
`containers.rs`, `emissions.rs`, `hsmm.rs`, `geometric.rs`, `posteriors.rs`
and `estimation.rs`. Exact C exports are recorded in the configuration ledger.

## Precision and executable evidence

FP_TYPE maps to the complete F32/F64 owners, descriptors, typed arrays,
numerical helpers, preparation, inference, posteriors, statistics, initialization,
estimation, generation and codecs. Both numerical precisions remain available
in one crate instead of requiring a separate build. The legacy wire format
still stores its source binary32 fields; saving binary64 storage rounds to
that schema as documented in the codec tests.

The native/C tests, actual C/Python caller families and independent-C WASM
families supply separate evidence for calculation. The configuration verifier
checks 26 precision-specific editable fields, ten pruning/occupancy alternatives,
seven exported numerical constants and the standard PI mapping. It runs in
the shared-module aggregate, which runs all eighteen families twice in reverse
orders. Fresh default/no-serialization Node and browser executions pass that
aggregate. The browser aggregate includes the explicit standard PI check.
All five latest-source native and external C target gates pass; the complete
requirement audit remains in progress at this checkpoint. Ledger integrity or field counts do
not independently establish those runtime gates.
