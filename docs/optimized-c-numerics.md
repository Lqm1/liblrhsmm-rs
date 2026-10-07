# Optimized C numerical comparison

The pinned source is liblrhsmm `1df92da4b77377f4725509e7240a9107ed4c063b`.
`tests/compare_numerics_optimization.py` compiles the existing numerical oracle
and unmodified `math-funcs.c` with `FP_TYPE=float` and `double`. It compares
`-O2`, `-Ofast`, explicit `-O3 -ffast-math`, and two diagnostic option combinations. All builds disable
floating-point contraction. The strict output must match the existing C
reference fixture byte-for-byte before any optimizer differences are reported.
The original fixtures and Rust numerical acceptance thresholds are unchanged.
The diagnostic driver substitutes finite `FLT_MAX` nextafter directions for
infinity, avoiding nonfinite expressions when finite-math-only is enabled.
All tested values are much smaller than FLT_MAX, so the neighboring values are
the same; strict payload equality additionally verifies this adaptation.
The library sources and original oracle file are unchanged.

The corpus contains 14,000 table entries and 42,008 helper results per precision,
including neighboring representable inputs at quantization boundaries. All
optimized variants preserve table bytes and recorded input bytes. Output
differences below are against the strict C baseline, which the Rust helper tests
already match bit-for-bit. These are measurements, not relaxed pass criteria.

| Compiler and precision | Log-add changes / 21,000 | Negative exponential changes / 21,000 | Exponential changes / 8 |
| --- | --- | --- | --- |
| Clang 21.1.0 via Windows Zig, float | 0 | 0 | 0 |
| Clang 21.1.0 via Windows Zig, double | 0 | 0 | 0 |
| GCC 13.3.0 on Linux, float | 0 | 0 | 1 |
| GCC 13.3.0 on Linux, double | 946 | 951 | 1 |

The Windows Zig 0.16.0 wrapper does not enable fast-math for `-Ofast` alone:
preprocessing leaves `__FAST_MATH__` undefined and `__FINITE_MATH_ONLY__` at zero.
Explicit `-O3 -ffast-math` defines the former and sets the latter to one.
Under that effective fast-math mode, Clang's double corpus has the same 946
log-add and 951 negative-exponential changes as GCC, while its eight exponential
samples and the entire float corpus remain unchanged. GCC's explicit fast-math
results match its `-Ofast` results. Thus the zero-difference Clang `-Ofast` rows
measure wrapper option handling, not absence of fast-math sensitivity.

GCC's double maximum absolute differences are 0.0004893700000000445 for log-add
and 0.0018370300000000173 for negative exponential. The first changed inputs
are `(0, -0.043)` and `-0.086`, respectively. They select neighboring table
coefficients. The positive-exponential difference at input 40 is
2199023255552.0 in absolute units, or 9.342476503671593e-6 relative to the
strict output. This branch intentionally uses the float-only fastapprox
implementation in both precision modes.

The isolated index probe reproduces integer indices 42 versus 43 for division
by 0.001/0.002. GCC emits `divsd` under `-O2` and `mulsd` under `-Ofast`, with
floating-point-to-integer truncation afterward. `-fno-reciprocal-math` restores
division and index 42 in this isolated probe, but does not restore all outputs
of the complete inlined corpus. Thus reciprocal replacement explains a concrete
boundary shift, while it is not by itself a complete explanation of every
inlined transformation. Disabling the broader unsafe-math optimization group
restores every recorded helper output. Inlining/input-production effects and
optimized model/inference/training behavior still require separate comparison.

## Inference and expectation corpus

`tests/compare_inference_optimization.py` now compares the existing geometric
HMM, explicit-duration HSMM and expectation-statistics drivers at both
precisions. It first verifies every strict output against the existing C
fixtures, including discrete fields. Float values are compared after binary32
conversion; double comparisons retain the existing 2e-14 normalized criterion.
It then compiles the same inputs with explicit `-O3 -ffast-math`, with contraction
disabled in both modes. Effective `__FAST_MATH__` and `__FINITE_MATH_ONLY__`
macros are recorded, so the Zig wrapper's inactive `-Ofast` behavior cannot
produce a false fast-math result. Original numerical fixtures are never changed.

Geometric and HSMM drivers use unchanged library sources. Only the expectation
driver uses the already disclosed duration-initialization correction in a
temporary source copy. Driver scalar declarations/literal suffixes and printing
are adapted exactly as in the existing precision reference generators.

Each precision compares 1,256 geometric numeric fields, 768 HSMM score fields
and 220 expectation/likelihood fields. The geometric numeric fields include
posterior slice presence indicators. Fourteen geometric boundary records and
18 HSMM occurrence-path records remain identical under fast math on both
compilers and precisions. This is evidence for these corpus decisions, not all
possible ties or pruning thresholds.

| Compiler and precision | Family | Changed numeric fields | Maximum absolute difference | Maximum normalized difference |
| --- | --- | ---: | ---: | ---: |
| Clang float | Geometric | 0 | 0 | 0 |
| Clang float | HSMM | 0 | 0 | 0 |
| Clang float | Expectation | 33 | 3.24249267578125e-5 | 2.3245811462402344e-6 |
| GCC float | Geometric | 0 | 0 | 0 |
| GCC float | HSMM | 16 | 1.52587890625e-5 | 2.384185791015625e-7 |
| GCC float | Expectation | 44 | 2.193450927734375e-4 | 8.9068385025487e-6 |
| Clang double | Geometric | 7 | 1.7763568394002505e-15 | 2.157943920346365e-16 |
| Clang double | HSMM | 42 | 3.906510000000196e-4 | 3.646827000001185e-4 |
| Clang double | Expectation | 24 | 2.86102294921875e-5 | 1.7137885582150869e-6 |
| GCC double | Geometric | 0 | 0 | 0 |
| GCC double | HSMM | 57 | 3.906510000000196e-4 | 3.646827000001185e-4 |
| GCC double | Expectation | 47 | 1.0150979688461348e-4 | 4.193203113328152e-6 |

Reports retain compiler/source/flag identity, strict and fast output SHA-256
hashes, effective macros and the first numerical difference:
`inference-optimization-zig.json` and `inference-optimization-gcc.json`.
The measured differences are compiler transformations of the same C algorithms.
Rust's strict arithmetic and acceptance tolerances remain unchanged. These
drivers cover forward/backward scores, Viterbi decisions, posteriors and one
expectation step. They do not replace repeated-training, real-audio SHIRO or
final combined native workflow verification.

GCC documents that fast-math options permit non-IEEE transformations and that
reciprocal math can replace division with reciprocal multiplication.
`-ffp-contract=off` controls contraction; it does not establish strict behavior
for the other enabled optimizations. Primary documentation was fetched through
Context7 and the Mintlify index was queried (it returned no relevant content):
https://gcc.gnu.org/onlinedocs/gcc/Optimize-Options.html

Raw reports record compiler identity, source revision, option sets, result
counts, first differences, maximum errors, payload SHA-256 digests, effective
preprocessor macros and isolated probe assembly operations:

- `numerics-optimization-gcc.json`
- `numerics-optimization-zig.json`

The reports are compiler-specific evidence. Rust retains the original division
expressions and its existing strict numerical semantics. No model format,
algorithm default, coefficient or tolerance has been changed to follow a
compiler-dependent boundary choice. This audit does not close the broader
optimized C numerical compatibility or final native acceptance requirements.
