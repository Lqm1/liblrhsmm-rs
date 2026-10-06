# Attribution and reference provenance

This project reimplements liblrhsmm, copyright 2016 Kanru Hua, under GPL-3.0-or-later. Audited upstream commit: `1df92da4b77377f4725509e7240a9107ed4c063b`.

The model fixture originates from SHIRO at commit `203ef7b71bf382c8b5ce3f86b8116f63265e2711`, whose source is copyright 2017-2018 Kanru Hua and licensed under GPL-3.0-or-later. It is retained for interoperability testing. The upstream repository does not provide a distinct model-specific license statement; confirm fixture redistribution provenance before external release.

Reference C serialization uses upstream cmp. Its MIT notice is retained in `LICENSES/cmp-MIT.txt`. Product serialization uses the Rust `rmp` crate; no C library is linked into this Rust implementation.
