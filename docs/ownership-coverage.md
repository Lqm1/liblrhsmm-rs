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

## Owned transition graphs and borrowed cache views

`SegmentationData::build_transitions_owned` consumes complete segmentation
parameters and derives incoming jumps through the existing native builder once.
The segmentation and cache vectors move into an immutable owned snapshot.
Parameter recovery with `into_segmentation` moves their allocations back into
editable storage. The original borrowed graph type and all its previous methods
retain their existing implementation and generic bounds.

Borrowed graphs expose `as_view`, `to_owned` and `into_owned`. A view contains
only immutable references to complete segmentation parameters and incoming
cache groups. `to_owned` copies both parameters and already-derived incoming
jumps; `into_owned` copies borrowed parameters and moves the existing cache.
Owned graph clones are independent deep copies. Owned graphs and their views
expose complete outgoing and incoming groups plus all segmentation fields.
Views can also copy into independent owners without recomputing incoming jumps.

View copying uses manual `Copy`/`Clone` implementations so existing generic
users receive no additional scalar bounds. Parameter and cache storage stay
immutable while borrowed. A compile-fail example verifies that a live view
prevents consuming its owner into mutable parameters. No self-references or
unsafe code are introduced. Existing inference routines still build their own
graphs; integration of these cache views into inference and C/WASM interfaces
remains pending.

Four tests in `tests/owned_transitions.rs` check both precisions, every field,
source changes/drop, independent clones and copied views, original allocation
identity when consuming/recovering parameters or moving a borrowed cache,
repeated cache views, empty segmentations and native validation errors.
The unchanged 16-record binary32 C transition corpus is compared directly in
binary32 and by exact widening in binary64. This is not an independently compiled
binary64 C transition corpus. Additional binary64 cases verify precision beyond
binary32, signed-zero probabilities and extreme out-of-bounds jump deltas.

All five native targets pass 65 suites /203 tests, zero failed/ignored, including
the two previous E0502 compile-fail tests and new E0505 ownership test. All 19
existing C caller families pass every target and all 19 Python families pass
the three 64-bit targets. New tests also pass without C API, without serialization,
and with C API but no serialization. Enabled/disabled Clippy and Rustdoc with
warnings denied, formatting, whitespace and unchanged cbindgen verification pass.
Source comparison retains the exact original graph construction and getters.

The C header remains 407 exports /25 descriptors /33 owners; source inventory
remains 54 implemented /49 pending. Native ownership helpers complete no pending
C/WASM segmentation/topology entry by themselves. Segmentation, graph, inference,
remaining data/training/generation/pool/configuration bindings, all WASM and
final combined acceptance remain required. Current regression evidence establishes
Debug shared libraries, not Release/static-link/Python32 acceptance.

Primary references checked through Context7 and Mintlify:

- https://doc.rust-lang.org/stable/std/marker/trait.Copy.html
- https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
- https://doc.rust-lang.org/stable/std/primitive.slice.html
- https://doc.rust-lang.org/stable/std/vec/struct.Vec.html

## C segmentation parameter coverage

The optional C boundary exposes both precisions of every editable segmentation
parameter field, stream/segment counts, outgoing deltas/probabilities and ragged
group lengths. Full construction/replacement, independent row copies, interval
setters/copies, jump insertion, resegmentation, deep parameter clones and release
have executable Rust and actual C/Python coverage. The 34 exports preserve stored
scalar bits and failed-operation state; four Rust tests include unchanged C
segmentation/copy/resegmentation fixture comparisons. All five native targets
pass 66 suites /207 tests and all20 C caller families; all20 Python caller
families pass the three64bit targets. No-serialization coverage passes as well.

Current header totals are441 exports /31 descriptors /35 owners. Source inventory
is57 implemented /46 pending. Complete incoming-cache C ownership/copying remains
pending. In particular, lrh_seg_copy copies incoming caches in upstream data.c;
the parameter clone alone does not complete that source mapping. lrh_seg_buildjumps,
graph/cache integration into inference, segmentation wire codecs, sets/datasets,
other remaining C families, every WASM mapping and final combined acceptance
remain required. No standalone segmentation start/end fields exist upstream.

## Complete C transition graph cache ownership

Independent immutable graph owners now retain every segmentation parameter and
derived incoming jump group in both precisions. The18 new C exports build once,
copy complete cached graphs without recomputing incoming transitions, expose
counts and interval copies for incoming/outgoing groups, return independent
editable parameter copies and release unique graph owners. Source/child edits or
release cannot invalidate the snapshot or independent cached clones.

Two Rust tests and actual C/Python callers check complete fields, unchanged16-record
binary32 C transition output, source and parent release, failed-output retention,
raw selected-precision bits and empty graphs. The f64 C corpus comparison uses
exact widening of binary32 values, not independent C double results. All5 targets
pass67 suites /209 tests and all21 C caller families; all21 Python families pass
the3 64bit targets. New tests and actual callers also pass without serialization.

Header totals are459 exports /31 descriptors /37 owners. Inventory is59 complete
C mappings /44 pending. Complete incoming-cache copying now covers lrh_seg_copy;
lrh_seg_buildjumps is also complete. Existing inference still constructs local
graphs; TransitionGraphView integration and remaining segmentation/data codecs,
sets/datasets, other C families, all WASM and final combined acceptance remain
required. Cached incoming storage remains immutable to prevent inconsistent graphs.

## Native graph cache reuse in inference

Forward/backward/Viterbi and duration occupancy can now borrow TransitionGraphView
directly from either graph kind, retaining the existing numerical kernels and
legacy validation order. Both borrowed and owned prepared models reuse their
existing duration caches; owned models support all three HSMM operations with
ordinary segmentations or existing graph views. No parameter/cache copies or
graph construction occur on the cached input branch. Four tests compare every
score/path/posterior bit across Rust paths, check invalid inputs and independently
retain original float/double/fractional HSMM and corrected duration C corpora.

All5 targets pass68 suites /213 tests and all21 existing C caller families;
all21 Python families pass the3 64bit targets. Native-only and no-serialization
tests pass. Header459/31/37, C59/44 inventories and manifests remain unchanged.
Native cache reuse is verified; usable C inference/score/path/posterior interfaces
remain pending, as do remaining codecs/data families, every WASM mapping and final
combined acceptance. Shared immutable cache storage remains valid while views
are live; the existing three borrow gates remain verified.

## Typed inference tables and likelihood totals

The new inference_tables boundary adds52 exports,26 per precision, six opaque
owners and one layout descriptor. Complete copied imports and replacements cover
log emissions and scores; log-to-density conversion creates distinct density
owners. All tables expose layout, element/frame/interval copies, deep cloning and
unique release. No internal storage pointer escapes. Failed replacement preserves
the owner; copying and cloning preserve selected-precision bits. Native score
import validates only positive matching shape; log import retains native finite
validation. Density import and model-to-emission computation remain future work.

All four original lrh_total mappings are complete through the three native total
methods. Unchanged original C forward/backward score arrays and14 total records
per precision are checked through the Rust ABI tests. Actual C/Python callers
exercise all52 symbols, ownership, values and errors; their forward-total
expectations use the previously verified log-add interface. Their own caller
checks do not load the original total corpus. See [C API evidence](c-api.md#typed-inference-tables-and-likelihood-totals).

All five native targets pass70 suites /218 tests,22 C caller families and the
three borrow gates. All22 Python families pass the three64bit targets. Feature
variants, Clippy/Rustdoc with warnings denied and generated-header verification
pass. Header511 exports /32 descriptors /43 owners; C63 complete /40 pending;
every WASM entry remains pending. C inference paths, posteriors and the remaining
public bindings are still required. Evidence is Debug/shared, not Release/static
or Python32.

## Complete geometric and HSMM inference results

All geometric and HSMM forward/backward/Viterbi mappings now have C calls in both
precisions. HSMM supports raw/prepared models and segmentation/cached-graph paths,
reusing immutable Native caches. Six owners preserve full score tables, occurrences,
frame states, boundaries and likelihood. Four settings descriptors expose every
Native field/default; one layout descriptor gives complete path dimensions.
Child results and clones own independent storage; no internal pointer escapes.
Score-pair import preserves arbitrary scalar bits and rejects mismatched shape.

Four ABI tests compare all native result bits,4 scenarios and3 cache lengths per
precision. Actual C/Python callers use all70 new functions and compare768 float /
1536 double HSMM scores and all path occurrences from unchanged original ordinary/
fractional C corpora. Geometric native bit comparisons are in Rust ABI tests.
See [full C evidence](c-api.md#complete-geometric-and-hsmm-inference-results).

Five targets pass71 suites /222 tests and all23 C families; all23 Python families
pass three64bit targets. Feature variants, quality checks and generated-header
verification pass. Header581 exports /37 descriptors /49 owners. C69 complete /
34 pending; all WASM entries pending. Native algorithms, fixtures, formats and
dependencies remain unchanged. Evidence is Debug/shared, not Release/static or
Python32; the remaining public bindings and combined final acceptance are required.

## Weighted multistream emission computation

All three original sample_outputprob mappings now have complete C access through
raw/owned-prepared model log/density methods in both precisions. Ten exports,
two complete settings descriptors and three pruning constants use existing table
owners. All native modes, weighted mixed-layout streams and pruning boundaries
are retained. Prepared calls borrow owned normalizers directly; independent result
storage survives input release. No native algorithm, format or dependency changed.

Rust ABI tests compare raw/prepared Native bits across all modes and temperatures,
including selected-precision values beyond1.0. Actual C/Python callers use all10
functions and compare432 unchanged original C log values and36 float densities.
Double density evidence is native bit comparison, not a new independent C corpus.
See [complete evidence](c-api.md#weighted-multistream-emission-computation).

Five targets pass72 suites /224 tests,24 C families and the three borrow gates;
all24 Python families pass three64bit targets. Feature variants and quality/header
verification pass. Header591 exports /39 descriptors /49 owners; C72 complete /
31 pending; every WASM entry pending. Remaining public bindings and final combined
acceptance are required. Debug/shared evidence only, not Release/static/Python32.


## Complete posterior storage and computation

All state/duration/transition/mixture posterior storage and calculations are
accessible through copied independent owners in both model precisions. Sparse
weights retain binary32 storage and distinguish absent from calculated empty
entries. Six original C mappings complete; no native algorithms/formats changed.
See [complete evidence](c-api.md#complete-posterior-storage-and-computation).

Five targets pass73 suites /229 tests and25 C families; all25 Python families pass
three64bit targets. All51 new exports are called, feature variants and quality
checks pass, and original/corrected C corpus regressions remain intact. Header642
exports /42 descriptors /54 owners; C78 complete /25 pending. Every WASM entry and
the remaining bindings/final combined acceptance remain required. Debug/shared
evidence only, not Release/static/Python32.


## Complete initialization, collection and estimation

Complete initialization, full posterior collection and HSMM/geometric estimators
are accessible in both precisions through existing independent statistics owners.
Every native statistic and likelihood is retained, with staged failure behavior.
External mixture arrays are copied for the existing native contiguous-slice API.
See [complete evidence](c-api.md#complete-initialization-collection-and-estimation).

Five targets pass74 suites /233 tests and26 C families; all26 Python families pass
three64bit targets. All10 new exports and update/codec/inference pipelines are
called; feature variants and quality checks pass. Legacy binary32 persistence is
retained in both precisions; decoded inference is compared to the native decoder.
Header652 exports /42 descriptors /54 owners; C83 complete /20 pending. Every WASM
entry, remaining bindings and final combined acceptance remain required.
Debug/shared evidence only, not Release/static/Python32.


## Sampling and complete observation generation

Both precisions expose scalar sampling, component selection and complete
multistream generation through synchronous caller-owned random-word or scalar
callbacks. Failure retains the sequence and result; consumed draws are not
restored. Observation owners remain independent after input release.
See [generation contracts and evidence](c-api.md#sampling-and-complete-observation-generation).

Five targets pass75 suites /239 tests and27 actual C families; all27 Python
families pass three64bit targets. All10 new exports, all three original C corpora
and callback transactions are verified. Feature variants and quality checks pass.
Header662 exports /45 descriptors /54 owners, two callback aliases, ABI1.
C87 complete /16 pending; every WASM entry and final combined acceptance remain
required. Debug/shared evidence only, not Release/static/Python32.


## Segmentation serialization

Both precisions expose strict byte/stream decoding, one-object prefix reading
and byte/stream writing through existing owners and borrowed I/O callbacks.
Every label and jump is retained, including both original array-header mismatches
and binary32 wire probabilities. Errors preserve constructor slots; partial
writers may have emitted a prefix. See [codec contracts and evidence](c-api.md#segmentation-serialization).

Five targets pass76 suites /243 tests and28 actual C families; all28 Python
families pass three64bit targets. All10 new exports, six original/corrected C
fixtures, complete fields, exact wire and callback/error transactions are covered.
Feature variants and quality checks pass. Header672 exports /45 descriptors /
54 owners, ABI1. C89 complete /14 pending; every WASM entry, remaining bindings
and final combined acceptance remain required. Debug/shared evidence only,
not Release/static/Python32.


## Memory pools and allocation identities

Native page growth, complete accounting and shared region identities are exposed
through two independent owners. Checked copies/replacements retain byte contents
on errors. Allocation release keeps pool storage; pool release frees pages while
remaining region identities retain metadata. No internal storage pointer escapes.
See [pool contracts and evidence](c-api.md#memory-pools-and-allocation-identities).

Five targets pass77 suites /245 tests and29 actual C families; all29 Python
families pass three64bit targets. All11 new exports, original C growth/checksum
corpus and shared identity/failure behavior are verified. Feature variants and
quality checks pass. Header683 exports /47 descriptors /56 owners, ABI1.
C93 complete /10 pending; every WASM entry, remaining bindings and final combined
acceptance remain required. Debug/shared evidence only, not Release/static/Python32.


## Complete sample sets and missing builder slots

Seventy-two unconditional exports expose complete ordered observation and
segmentation sets and builders in both precisions. All sample fields, repeated
children, heterogeneous shapes and missing slots survive copies. Builders retain
missing slots until explicitly filled, cleared or resized, and validated builds
preserve the caller's builder. Set replacement stages complete input validation
and copies before publication. Children are independent of all source lifetimes.
Present empty samples retain their stream shapes and remain distinct from missing
slots. See [contracts and evidence](c-api.md#complete-sample-sets-and-incremental-builders).

Five targets pass78 suites /249 tests and30 actual C caller families. All30 Python
families pass three64bit targets. All72 exports, complete field/bit comparisons,
partial builder clones, present empty samples and retained output/error cases are
verified. Feature variants and quality checks pass. Header755 exports /
47 descriptors /64 owners, ABI1. C98 complete /5 pending; every WASM entry and
final combined acceptance remain required. Debug/shared evidence only, not
Release/static/Python32.


## Complete paired dataset regrouping

Twelve unconditional exports and two owners expose complete paired regrouping in
both precisions. Input sets are borrowed directly by the native implementation;
one owner publishes both result sets after success. All features, labels, final
short groups and ordinary regrouped jumps are available through independent
editable result sets. Empty aligned samples and repeated boundaries are supported.
Errors retain initialized outputs; retrieved sets and children outlive all inputs
and dataset owners. See [contracts and evidence](c-api.md#complete-paired-dataset-regrouping).

All five targets pass 79 suites and 251 tests and all 31 actual C caller families.
All 31 Python families pass the three 64-bit targets. All 12 new exports, complete
result fields, 150 corrected-C double records, six unit sizes and error/lifetime
checks are verified. Feature variants and quality checks pass. The header contains
767 exports, 47 descriptors and 66 owners, with ABI version 1. C99 complete /
4 pending; every WASM entry and final combined acceptance remain required.
Debug/shared evidence only, not Release/static/Python32.
