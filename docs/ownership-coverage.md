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

Primary references:

- https://doc.rust-lang.org/reference/destructors.html
- https://doc.rust-lang.org/std/mem/fn.drop.html
- https://doc.rust-lang.org/std/vec/struct.Vec.html
