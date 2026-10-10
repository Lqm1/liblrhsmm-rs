# Workspace layout

This repository contains five packages in a flat Cargo workspace:

| Package | Directory | Responsibility |
| --- | --- | --- |
| `liblrhsmm-rs` | `crates/liblrhsmm-rs` | Native Rust API and computations |
| `liblrhsmm-rs-capi` | `crates/liblrhsmm-rs-capi` | C ABI, ownership boundary and checked-in headers |
| `liblrhsmm-rs-wasm` | `crates/liblrhsmm-rs-wasm` | Browser and Node.js WebAssembly bindings |
| `liblrhsmm-rs-node` | `crates/liblrhsmm-rs-node` | Native Node.js addon with napi-rs |
| `liblrhsmm-rs-python` | `crates/liblrhsmm-rs-python` | Native Python extension with PyO3 and maturin |

The workspace defaults to the native package. `cargo test` and existing SHIRO
binary names continue to select native workflows. Use explicit package selection
for adapters. The native package forbids unsafe code and has no C/WASM binding
features or wasm-bindgen dependency. All adapters depend directly on the native
package; the WASM adapter does not call the C ABI.

```text
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo build --locked -p liblrhsmm-rs-capi
cargo build --locked -p liblrhsmm-rs-wasm --target wasm32-unknown-unknown
cbindgen --config crates/liblrhsmm-rs-capi/cbindgen.toml --crate liblrhsmm-rs-capi --output crates/liblrhsmm-rs-capi/include/liblrhsmm_rs.h
wasm-bindgen --target nodejs --out-name liblrhsmm_rs --out-dir target/wasm-node target/wasm32-unknown-unknown/debug/liblrhsmm_rs_wasm.wasm
wasm-bindgen --target web --out-name liblrhsmm_rs --out-dir target/wasm-web target/wasm32-unknown-unknown/debug/liblrhsmm_rs_wasm.wasm
```

Use wasm-bindgen CLI 0.2.129, matching the pinned binding dependency. The native
C artifact base name is now `liblrhsmm_rs_capi`, rather than `liblrhsmm_rs`. This changes DLL,
import-library and static-library filenames, not C symbols or ABI layouts. Pass
the new artifact path to existing C/Python callers. The generated JavaScript
module retains `liblrhsmm_rs.js` through `--out-name`; exported JS names are unchanged.

Rust integration tests live in their owning package. Independent C/Python/JS
callers, reference generators and shared fixtures remain in the root `tests/`
directory. This keeps one authoritative copy of each original reference dataset.
The root `docs/` and license/attribution files apply to all five packages.

Historical acceptance reports retain their original commit and execution scope.
Their feature-based commands do not define the new workspace interface.

## Dependency policy

Adapters use a local workspace path to their own native package. This ensures
that a source edit is tested by its adapters before committing. Cross-repository
dependencies use public GitHub URLs and immutable commit revisions; they do not
require sibling checkout directories. Cargo resolves the native package inside
the dependency repository workspace. No registry publication is required.

Native Node.js and Python setup, public API typing, ownership and local packaging
are documented in their package READMEs and [verification](native-bindings.md).
