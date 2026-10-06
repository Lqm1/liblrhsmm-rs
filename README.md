# liblrhsmm-rs

An idiomatic Rust reimplementation of the complete public liblrhsmm functionality, licensed under GPL-3.0-or-later. Upstream: https://github.com/Sleepwalking/liblrhsmm.

The port is in progress. Owned model/data containers and model, observation, segmentation, and dataset serialization are implemented. Model operations include single-component concatenation, duration conversion, highest-weight mixture splitting, and weighted duration log density. Prepared GMM likelihoods and weighted multistream log-density tables support the original boundary and diagonal pruning conventions. Geometric-duration HMM forward/backward and Viterbi are implemented with the original quantized probability helpers. Explicit-duration HSMM forward and Viterbi support skip, self, and backward transitions through an immutable transition graph. HSMM backward and geometric HMM state/transition occupancies are implemented. HSMM duration/state occupancies and HSMM/HMM mixture occupancies are implemented, including the documented initial-state correction. Expectation collection, parameter estimation, generation, data regrouping, and remaining model operations are not yet implemented. See `docs/api-inventory.csv` for the initial source inventory; it still requires review of macros, configuration values, and ownership equivalents.

```powershell
cargo test
cargo test --target i686-pc-windows-msvc
cargo clippy --all-targets -- -D warnings
```

The model reader supports the historical GMM format used by SHIRO's pretrained model and the current format with per-dimension variance floors. The default writer produces the current C-readable format. `ModelEncoding::WithoutVarianceFloors` explicitly writes historical models and rejects nonzero variance floors rather than losing data.

The complete native acceptance gate requires Windows and Linux x86 32-bit and 64-bit execution tests, including the Tier 1 Windows x86_64 GNU target, before C ABI and WebAssembly implementation. Those bindings are not implemented yet.
