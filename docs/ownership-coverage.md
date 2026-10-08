# Owned resource destruction

The port implements upstream deletion operations through Rust ownership.
There are no raw owning pointers, manual frees, reference cycles, ManuallyDrop
fields, or custom destructors in these containers. Leaving scope or calling
std::mem::drop destroys owned fields recursively, including vector contents.
An absent optional builder slot owns no sample and needs no special deletion.

| Upstream operation | Owned Rust resource and nested storage |
| --- | --- |
| lrh_delete_duration | Duration parameters; PreparedDuration lookup vector |
| lrh_delete_gmm | GaussianMixture parameter vectors; PreparedGaussian normalizers |
| lrh_delete_stream | Stream mixture vector and every mixture's parameter vectors |
| lrh_delete_model | Model streams/durations; PreparedModel's emission and duration caches |
| lrh_delete_observ | Observation stream vectors and every stream's feature values |
| lrh_delete_observset | Vec of observations; optional sample slots in ObservationSetBuilder |
| lrh_delete_seg | Segmentation state/boundary vectors, per-stream mappings, and outgoing jumps |
| lrh_delete_segset | Vec of segmentations; optional sample slots in SegmentationSetBuilder |
| lrh_delete_mempool | MemoryPool page buffers and page list |

Prepared objects borrow source parameters and own their cache vectors. The
borrow checker prevents deleting a source while a prepared object remains in
use. The compile-fail example in preparation.rs verifies the related prohibition
on mutating source parameters. No deletion function is required for automatic
destruction; std::mem::drop provides explicit early destruction where useful.

PoolAllocation handles retain a shared identity token, not pool pages. Dropping
MemoryPool releases its pages even if handles still exist. A remaining handle
cannot expose freed storage and is rejected by any other pool. This case is
covered by tests/memory_pool.rs.

Prepared cache constants and lookup values are available through borrowed
read-only getters. MemoryPool::pages exposes per-page used bytes and requested
capacity through an exact-length iterator, without exposing owning pointers.
The field-by-field mappings and independent C metadata comparisons are recorded
in header-coverage.md.

The original C API permits NULL for its deletion functions. Rust represents an
optional resource with Option, whose None variant is safe to drop. Built sample
sets own each sample separately; deep clones allocate independent contents, as
verified against C copy fixtures in tests/construction.rs.

The precision-parameterized model/data containers own the same vectors and
optional construction slots at either binary32 or binary64 precision. Existing
binary32 type names are aliases, not additional storage wrappers. Prepared
Gaussian objects borrow the selected-precision parameters and own their
normalizer vectors. The binary64 tests verify that a deep clone can be changed
without modifying the source, while preserving values beyond binary32
precision in source parameters, observations and state-jump probabilities.

Selected-precision sufficient statistics own their accumulators. Expectation
and initialization validate and collect into a staged clone before replacement,
so an invalid later stream does not partially change earlier accumulators.
Model maximization stages all parameters before committing an update. Binary64
regressions cover a nonfinite observation after successful accumulation and an
invalid later duration after emission statistics were collected. Repeated
learning creates fresh accumulators for each step and leaves no borrowed cache
alive across parameter updates. Posterior slices remain owned binary32 vectors,
including when their state occupancies and input parameters are binary64.

Primary references:

- https://doc.rust-lang.org/reference/destructors.html
- https://doc.rust-lang.org/std/mem/fn.drop.html
- https://doc.rust-lang.org/std/vec/struct.Vec.html

## Owned stream and model preparation

`StreamParameters::prepare_owned` consumes its complete parameters into an
immutable snapshot with matching Gaussian normalizers. `ModelParameters::prepare_owned`
and `prepare_owned_selected_durations` retain the complete model once, alongside
emission normalizers and the selected duration caches. Duplicate selected states
are prepared once, and unused uninitialized durations can remain unprepared.
Invalid preparation settings are rejected even for an empty selection.

Prepared emission and duration views borrow the snapshot's immutable parameters
and cache slices. Creating or evaluating a view does not allocate or copy caches.
An explicit `to_owned` copies both parameters and existing cache state without
recomputing it. Snapshot clones own independent parameters/caches. `into_parameters`
for streams and `into_model` for models recover editable parameter vectors by
moving them, retaining their allocations.

Owned model emission-table sampling and borrowed preparation use the same
validation, pruning and accumulation implementation. A density callback supplies
the selected preparation's Gaussian evaluator. Gaussian probability formulas,
normalizer calculations and duration preparation remain unchanged.

Five tests in `tests/owned_model_preparation.rs` cover both precisions, mixed
Gaussian layouts, every parameter family, caches and component/mixture densities,
source changes/release, independent clones and copied children, moved parameter
recovery, selected/duplicate/empty duration states and invalid inputs. They compare
144 distinct original C emission-table values across three cache lengths and
every historical model emission cache plus its 205 prepared duration states.
Tests check borrowed and owned table/density equality under all pruning modes
and multiple temperatures. With serialization disabled, the four non-historical
tests also pass without enabling the C API.

All five native targets pass 58 suites /186 tests, zero failed/ignored, retaining
the original native test inventory and both compile-fail borrow checks. All
15 existing C callers pass on every target, and all 15 Python callers pass on
the three 64bit targets. Enabled/disabled serialization Clippy and Rustdoc with
warnings denied, formatting and cbindgen verification pass. C bindings remain
at 315 exports and 47 implemented /56 pending source mappings. These native
ownership helpers do not complete the pending stream/model preparation C/WASM
interfaces or final combined acceptance.

Stable standard-library allocation and checked-access references:

- https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.try_reserve_exact
- https://doc.rust-lang.org/stable/std/primitive.slice.html#method.get
