/* Actual C caller for complete observation stream storage and frame access. */
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
    float values[6], copied[6];
    memcpy(values, bits, sizeof values);
    LiblrhsmmRsObservationStreamF32 *owner = NULL, *clone = NULL, *zeroed = NULL;
    assert(liblrhsmm_rs_observation_stream_new_f32(2, 3, &zeroed) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_values_f32(zeroed, 0, copied, 6) == 0);
    for (unsigned i = 0; i < 6; i++) assert(copied[i] == 0);
    assert(liblrhsmm_rs_observation_stream_release_f32(&zeroed) == 0);
    assert(liblrhsmm_rs_observation_stream_create_f32(3, values, 6, &owner) == 0);
    assert(liblrhsmm_rs_observation_stream_clone_f32(owner, &clone) == 0);
    memset(values, 0, sizeof values);
    _Alignas(8) LiblrhsmmRsObservationStreamLayout layout = {99, 99, 99};
    assert(liblrhsmm_rs_observation_stream_layout_f32(owner, &layout) == 0);
    assert(layout.frames == 2 && layout.dimensions == 3 && layout.values == 6);
    assert(liblrhsmm_rs_observation_stream_copy_values_f32(owner, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_frame_f32(owner, 1, copied, 3) == 0);
    assert(memcmp(copied, bits + 3, sizeof(float) * 3) == 0);
    float row[] = {1, 2, 3};
    assert(liblrhsmm_rs_observation_stream_set_frame_f32(owner, 1, row, 3) == 0);
    assert(liblrhsmm_rs_observation_stream_set_values_f32(owner, 0, row, 3) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_frame_f32(owner, 0, copied, 3) == 0);
    assert(memcmp(copied, row, sizeof row) == 0);
    assert(liblrhsmm_rs_observation_stream_replace_f32(owner, 3, row, 2) == 3);
    assert(liblrhsmm_rs_observation_stream_set_values_f32(owner, 6, row, 1) == 2);
    assert(liblrhsmm_rs_observation_stream_copy_values_f32(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_observation_stream_copy_values_f32(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_observation_stream_set_values_f32(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_frame_f32(owner, 0, copied, 2) == 2);
    assert(liblrhsmm_rs_observation_stream_replace_f32(owner, 1, row, 3) == 0);
    assert(liblrhsmm_rs_observation_stream_layout_f32(owner, &layout) == 0 && layout.frames == 3 && layout.dimensions == 1);
    assert(liblrhsmm_rs_observation_stream_release_f32(&owner) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_values_f32(clone, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f32(&clone) == 0);
    assert(liblrhsmm_rs_observation_stream_create_f32(3, NULL, 0, &owner) == 0);
    assert(liblrhsmm_rs_observation_stream_layout_f32(owner, &layout) == 0 && layout.frames == 0 && layout.values == 0);
    assert(liblrhsmm_rs_observation_stream_release_f32(&owner) == 0 && owner == NULL);
    assert(liblrhsmm_rs_observation_stream_release_f32(&owner) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f32(NULL) == 1);
}

/* Actual C caller for complete observation stream storage and frame access. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static void exercise_f64(void) {
    uint64_t bits[] = {0x7ff8000000001234, 0x8000000000000000, 0x7ff0000000000000, 0x3ff4000000000001, 0xfff0000000000000, 1};
    _Alignas(8) double values[6], copied[6];
    memcpy(values, bits, sizeof values);
    LiblrhsmmRsObservationStreamF64 *owner = NULL, *clone = NULL, *zeroed = NULL;
    assert(liblrhsmm_rs_observation_stream_new_f64(2, 3, &zeroed) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_values_f64(zeroed, 0, copied, 6) == 0);
    for (unsigned i = 0; i < 6; i++) assert(copied[i] == 0);
    assert(liblrhsmm_rs_observation_stream_release_f64(&zeroed) == 0);
    assert(liblrhsmm_rs_observation_stream_create_f64(3, values, 6, &owner) == 0);
    assert(liblrhsmm_rs_observation_stream_clone_f64(owner, &clone) == 0);
    memset(values, 0, sizeof values);
    _Alignas(8) LiblrhsmmRsObservationStreamLayout layout = {99, 99, 99};
    assert(liblrhsmm_rs_observation_stream_layout_f64(owner, &layout) == 0);
    assert(layout.frames == 2 && layout.dimensions == 3 && layout.values == 6);
    assert(liblrhsmm_rs_observation_stream_copy_values_f64(owner, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_frame_f64(owner, 1, copied, 3) == 0);
    assert(memcmp(copied, bits + 3, sizeof(double) * 3) == 0);
    _Alignas(8) double row[] = {1, 2, 3};
    assert(liblrhsmm_rs_observation_stream_set_frame_f64(owner, 1, row, 3) == 0);
    assert(liblrhsmm_rs_observation_stream_set_values_f64(owner, 0, row, 3) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_frame_f64(owner, 0, copied, 3) == 0);
    assert(memcmp(copied, row, sizeof row) == 0);
    assert(liblrhsmm_rs_observation_stream_replace_f64(owner, 3, row, 2) == 3);
    assert(liblrhsmm_rs_observation_stream_set_values_f64(owner, 6, row, 1) == 2);
    assert(liblrhsmm_rs_observation_stream_copy_values_f64(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_observation_stream_copy_values_f64(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_observation_stream_set_values_f64(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_frame_f64(owner, 0, copied, 2) == 2);
    assert(liblrhsmm_rs_observation_stream_replace_f64(owner, 1, row, 3) == 0);
    assert(liblrhsmm_rs_observation_stream_layout_f64(owner, &layout) == 0 && layout.frames == 3 && layout.dimensions == 1);
    assert(liblrhsmm_rs_observation_stream_release_f64(&owner) == 0);
    assert(liblrhsmm_rs_observation_stream_copy_values_f64(clone, 0, copied, 6) == 0);
    assert(memcmp(copied, bits, sizeof copied) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f64(&clone) == 0);
    assert(liblrhsmm_rs_observation_stream_create_f64(3, NULL, 0, &owner) == 0);
    assert(liblrhsmm_rs_observation_stream_layout_f64(owner, &layout) == 0 && layout.frames == 0 && layout.values == 0);
    assert(liblrhsmm_rs_observation_stream_release_f64(&owner) == 0 && owner == NULL);
    assert(liblrhsmm_rs_observation_stream_release_f64(&owner) == 0);
    assert(liblrhsmm_rs_observation_stream_release_f64(NULL) == 1);
}

int main(void) { exercise_f32(); exercise_f64(); puts("observation stream: all20 exports and exact scalar bits passed"); return 0; }
