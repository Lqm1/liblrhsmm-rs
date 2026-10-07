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
