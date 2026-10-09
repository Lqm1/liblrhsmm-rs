# Workspace split verification

This checkpoint verifies the native/C/WASM package split. Earlier acceptance
reports describe the previous single-package layout and remain historical evidence.

## Scope

The repository has three flat workspace members: `liblrhsmm-rs`,
`liblrhsmm-rs-capi` and `liblrhsmm-rs-wasm`. All were initialized with
`cargo new --lib`. Computations and codecs remain in the native package;
both adapters depend directly on it through a workspace-local path.
The native package forbids unsafe code and has no binding dependencies.
No sibling repository is needed to build this project.

Native computations, original reference fixtures and license notices are preserved.
`data::validate_jump_group` is now public so adapters can reuse its checked validation.
Regenerated cbindgen 0.29.4 headers match the checked-in header byte for byte.
C symbols and layouts are unchanged. The C artifact base name is
`liblrhsmm_rs_capi`; JavaScript retains `liblrhsmm_rs.js` via `--out-name`.
Serialization defaults and explicit opt-out behavior are preserved in both adapters.

## Executed checks

Before this publication checkpoint, `cargo test --locked --workspace -- --include-ignored`
passed on x86_64-pc-windows-msvc, i686-pc-windows-msvc,
x86_64-pc-windows-gnu and x86_64-unknown-linux-gnu.
Each target passed 259 tests, with zero failures and zero ignored tests.
The native/C packages also passed 202 tests per target with serialization disabled.
The Linux 32-bit migration refresh and final cross-repository Git dependency gate
are recorded in [SHIRO's migration report](https://github.com/Lqm1/shiro-rs/blob/main/docs/workspace-split-verification.md).

All 33 independent C caller programs passed on those four targets and
on i686-unknown-linux-gnu. All 33 Python caller programs passed on each of the
three 64-bit targets. C callers used optimization with assertions enabled.
Python 32-bit execution was not tested.

Native workspace and wasm32 adapter Clippy checks passed with warnings denied.
Rustdoc passed with warnings denied, and formatting and whitespace checks passed.
The generated Node.js module passed all 18 verification families in two passes,
both with and without serialization. Three independent C DLLs and three
independent WASM modules also passed SHIRO's real-speech model/training composition.

The new web-target JavaScript glue is byte-identical to previously browser-verified
glue. A fresh browser execution during this migration was unavailable because
browser automation connections timed out. The new WASM computations were
executed through Node.js; this report does not claim a fresh browser run.

Scratch logs are held in the migration task workspace under `work/split-native-*`,
`work/split-test-*`, `work/split-callers-*` and `work/split-node-*`.
Checks ran on the split working tree before its publication commit. Old HEAD values
in runner logs are baseline identifiers, not evidence of the split in old commits.
