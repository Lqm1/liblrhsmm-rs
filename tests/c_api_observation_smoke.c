/* Actual C calls for complete multi-stream observation ownership. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static void exercise_f32(void) {
    uint32_t bits[] = {0x7fc01234, 0x80000000, 0x7f800000, 0x3fa00000, 0xff800000, 1};
    float values[6], copied[6], row[] = {4, -0.0, 9};
    memcpy(values, bits, sizeof values);
    LiblrhsmmRsObservationStreamF32 *first = NULL, *second = NULL, *child = NULL;
    LiblrhsmmRsObservationF32 *owner = NULL, *clone = NULL, *zeroed = NULL;
    uintptr_t dimensions[] = {2, 3};
    assert(liblrhsmm_rs_observation_new_f32(2, dimensions, 2, &zeroed) == 0);
    assert(liblrhsmm_rs_observation_copy_values_f32(zeroed, 1, 0, copied, 6) == 0);
    for (unsigned i = 0; i < 6; i++) assert(copied[i] == 0);
    assert(liblrhsmm_rs_observation_release_f32(&zeroed) == 0);
    assert(liblrhsmm_rs_observation_stream_create_f32(2, values, 4, &first) == 0);
    assert(liblrhsmm_rs_observation_stream_create_f32(3, values, 6, &second) == 0);
    const LiblrhsmmRsObservationStreamF32 *inputs[] = {first, second};
    assert(liblrhsmm_rs_observation_create_f32(2, inputs, 2, &owner) == 0);
    assert(liblrhsmm_rs_observation_clone_f32(owner, &clone) == 0);
    _Alignas(8) LiblrhsmmRsObservationLayout layout = {99, 99};
    _Alignas(8) LiblrhsmmRsObservationStreamLayout stream_layout = {99, 99, 99};
    assert(liblrhsmm_rs_observation_layout_f32(owner, &layout) == 0 && layout.frames == 2 && layout.streams == 2);
    assert(liblrhsmm_rs_observation_get_stream_layout_f32(owner, 1, &stream_layout) == 0 && stream_layout.frames == 2 && stream_layout.dimensions == 3 && stream_layout.values == 6);
    assert(liblrhsmm_rs_observation_get_stream_f32(owner, 1, &child) == 0);
    assert(liblrhsmm_rs_observation_copy_values_f32(owner, 1, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_copy_frame_f32(owner, 1, 1, copied, 3) == 0);
    assert(memcmp(copied, bits + 3, 3 * sizeof(float)) == 0);
    assert(liblrhsmm_rs_observation_set_frame_f32(owner, 1, 1, row, 3) == 0);
    assert(liblrhsmm_rs_observation_set_values_f32(owner, 1, 0, row, 3) == 0);
    assert(liblrhsmm_rs_observation_set_stream_f32(owner, 0, second) == 0);
    assert(liblrhsmm_rs_observation_get_stream_layout_f32(owner, 0, &stream_layout) == 0 && stream_layout.dimensions == 3);
    assert(liblrhsmm_rs_observation_stream_replace_f32(first, 1, row, 3) == 0);
    assert(liblrhsmm_rs_observation_set_stream_f32(owner, 1, first) == 3);
    assert(liblrhsmm_rs_observation_replace_f32(owner, 2, inputs, 2) == 3);
    assert(liblrhsmm_rs_observation_copy_frame_f32(owner, 1, 0, copied, 3) == 0);
    assert(memcmp(copied, row, sizeof row) == 0);
    assert(liblrhsmm_rs_observation_set_values_f32(owner, 1, 6, row, 1) == 2);
    assert(liblrhsmm_rs_observation_copy_frame_f32(owner, 1, 2, copied, 3) == 2);
    assert(liblrhsmm_rs_observation_copy_frame_f32(owner, 1, 0, copied, 2) == 2);
    assert(liblrhsmm_rs_observation_copy_values_f32(owner, 1, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_observation_copy_values_f32(owner, 1, 6, NULL, 0) == 0);
    const LiblrhsmmRsObservationStreamF32 *repeated[] = {second, second};
    assert(liblrhsmm_rs_observation_replace_f32(owner, 2, repeated, 2) == 0);
    assert(liblrhsmm_rs_observation_release_f32(&owner) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f32(&first) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f32(&second) == 0);
    assert(liblrhsmm_rs_observation_copy_values_f32(clone, 1, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_release_f32(&clone) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_values_f32(child, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f32(&child) == 0);
    assert(liblrhsmm_rs_observation_new_f32(0, dimensions, 2, &owner) == 0);
    assert(liblrhsmm_rs_observation_copy_values_f32(owner, 0, 0, NULL, 0) == 0);
    assert(liblrhsmm_rs_observation_copy_frame_f32(owner, 0, 0, copied, 2) == 2);
    assert(liblrhsmm_rs_observation_release_f32(&owner) == 0 && owner == NULL);
    assert(liblrhsmm_rs_observation_release_f32(&owner) == 0);
}

static void exercise_f64(void) {
    uint64_t bits[] = {UINT64_C(0x7ff8000000001234), UINT64_C(0x8000000000000000), UINT64_C(0x7ff0000000000000), UINT64_C(0x3ff4000000000001), UINT64_C(0xfff0000000000000), 1};
    _Alignas(8) double values[6], copied[6], row[] = {4, -0.0, 9};
    memcpy(values, bits, sizeof values);
    LiblrhsmmRsObservationStreamF64 *first = NULL, *second = NULL, *child = NULL;
    LiblrhsmmRsObservationF64 *owner = NULL, *clone = NULL, *zeroed = NULL;
    uintptr_t dimensions[] = {2, 3};
    assert(liblrhsmm_rs_observation_new_f64(2, dimensions, 2, &zeroed) == 0);
    assert(liblrhsmm_rs_observation_copy_values_f64(zeroed, 1, 0, copied, 6) == 0);
    for (unsigned i = 0; i < 6; i++) assert(copied[i] == 0);
    assert(liblrhsmm_rs_observation_release_f64(&zeroed) == 0);
    assert(liblrhsmm_rs_observation_stream_create_f64(2, values, 4, &first) == 0);
    assert(liblrhsmm_rs_observation_stream_create_f64(3, values, 6, &second) == 0);
    const LiblrhsmmRsObservationStreamF64 *inputs[] = {first, second};
    assert(liblrhsmm_rs_observation_create_f64(2, inputs, 2, &owner) == 0);
    assert(liblrhsmm_rs_observation_clone_f64(owner, &clone) == 0);
    _Alignas(8) LiblrhsmmRsObservationLayout layout = {99, 99};
    _Alignas(8) LiblrhsmmRsObservationStreamLayout stream_layout = {99, 99, 99};
    assert(liblrhsmm_rs_observation_layout_f64(owner, &layout) == 0 && layout.frames == 2 && layout.streams == 2);
    assert(liblrhsmm_rs_observation_get_stream_layout_f64(owner, 1, &stream_layout) == 0 && stream_layout.frames == 2 && stream_layout.dimensions == 3 && stream_layout.values == 6);
    assert(liblrhsmm_rs_observation_get_stream_f64(owner, 1, &child) == 0);
    assert(liblrhsmm_rs_observation_copy_values_f64(owner, 1, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_copy_frame_f64(owner, 1, 1, copied, 3) == 0);
    assert(memcmp(copied, bits + 3, 3 * sizeof(double)) == 0);
    assert(liblrhsmm_rs_observation_set_frame_f64(owner, 1, 1, row, 3) == 0);
    assert(liblrhsmm_rs_observation_set_values_f64(owner, 1, 0, row, 3) == 0);
    assert(liblrhsmm_rs_observation_set_stream_f64(owner, 0, second) == 0);
    assert(liblrhsmm_rs_observation_get_stream_layout_f64(owner, 0, &stream_layout) == 0 && stream_layout.dimensions == 3);
    assert(liblrhsmm_rs_observation_stream_replace_f64(first, 1, row, 3) == 0);
    assert(liblrhsmm_rs_observation_set_stream_f64(owner, 1, first) == 3);
    assert(liblrhsmm_rs_observation_replace_f64(owner, 2, inputs, 2) == 3);
    assert(liblrhsmm_rs_observation_copy_frame_f64(owner, 1, 0, copied, 3) == 0);
    assert(memcmp(copied, row, sizeof row) == 0);
    assert(liblrhsmm_rs_observation_set_values_f64(owner, 1, 6, row, 1) == 2);
    assert(liblrhsmm_rs_observation_copy_frame_f64(owner, 1, 2, copied, 3) == 2);
    assert(liblrhsmm_rs_observation_copy_frame_f64(owner, 1, 0, copied, 2) == 2);
    assert(liblrhsmm_rs_observation_copy_values_f64(owner, 1, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_observation_copy_values_f64(owner, 1, 6, NULL, 0) == 0);
    const LiblrhsmmRsObservationStreamF64 *repeated[] = {second, second};
    assert(liblrhsmm_rs_observation_replace_f64(owner, 2, repeated, 2) == 0);
    assert(liblrhsmm_rs_observation_release_f64(&owner) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f64(&first) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f64(&second) == 0);
    assert(liblrhsmm_rs_observation_copy_values_f64(clone, 1, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_release_f64(&clone) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_values_f64(child, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f64(&child) == 0);
    assert(liblrhsmm_rs_observation_new_f64(0, dimensions, 2, &owner) == 0);
    assert(liblrhsmm_rs_observation_copy_values_f64(owner, 0, 0, NULL, 0) == 0);
    assert(liblrhsmm_rs_observation_copy_frame_f64(owner, 0, 0, copied, 2) == 2);
    assert(liblrhsmm_rs_observation_release_f64(&owner) == 0 && owner == NULL);
    assert(liblrhsmm_rs_observation_release_f64(&owner) == 0);
}

int main(void) { exercise_f32(); exercise_f64(); puts("Complete observation C ownership, all fields/bits, frames and errors verified"); return 0; }
