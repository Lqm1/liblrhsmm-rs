/* Actual C calls for every segmentation export in both precisions. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static void exercise_f32(void) {
    LiblrhsmmRsSegmentationF32 *owner = NULL, *clone = NULL, *result = NULL;
    LiblrhsmmRsSegmentationLayout layout = {99, 99};
    int32_t boundaries[] = {2, 4, 8}, durations[] = {10, 11, 12};
    int32_t states[] = {20, INT32_MIN, INT32_MAX}, copied[3];
    const int32_t *rows[] = {states, states};
    uint32_t patterns[] = {0x7fc01234, 0x80000000, 1};
    LiblrhsmmRsJumpF32 jumps[3] = {{-2, 0}, {0, 0}, {1, 0}}, output[4];
    for (unsigned i = 0; i < 3; i++) memcpy(&jumps[i].probability, &patterns[i], sizeof patterns[i]);
    LiblrhsmmRsJumpSliceF32 groups[] = {{jumps, 3}, {jumps + 2, 1}, {jumps + 1, 2}};
    assert(liblrhsmm_rs_segmentation_new_f32(2, 3, &owner) == 0);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(owner, 0, copied, 3) == 0);
    assert(copied[0] == 0 && copied[1] == 0 && copied[2] == 0);
    assert(liblrhsmm_rs_segmentation_release_f32(&owner) == 0);
    assert(liblrhsmm_rs_segmentation_create_f32(2, 3, boundaries, durations, rows, groups, &owner) == 0);
    assert(liblrhsmm_rs_segmentation_clone_f32(owner, &clone) == 0);
    assert(liblrhsmm_rs_segmentation_layout_f32(owner, &layout) == 0 && layout.streams == 2 && layout.segments == 3);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f32(owner, 0, copied, 3) == 0);
    assert(memcmp(copied, durations, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f32(owner, 1, 0, copied, 3) == 0);
    assert(memcmp(copied, states, sizeof copied) == 0);
    uintptr_t count = 99;
    assert(liblrhsmm_rs_segmentation_outgoing_count_f32(owner, 0, &count) == 0 && count == 3);
    assert(liblrhsmm_rs_segmentation_copy_outgoing_f32(owner, 0, 0, output, 3) == 0);
    for (unsigned i = 0; i < 3; i++) {
        assert(output[i].delta == jumps[i].delta);
        assert(memcmp(&output[i].probability, &patterns[i], sizeof patterns[i]) == 0);
    }
    int32_t replacement[] = {3, 5, 9};
    assert(liblrhsmm_rs_segmentation_set_boundaries_f32(owner, 0, replacement, 3) == 0);
    assert(liblrhsmm_rs_segmentation_set_duration_states_f32(owner, 0, replacement, 3) == 0);
    assert(liblrhsmm_rs_segmentation_set_output_states_f32(owner, 0, 0, replacement, 3) == 0);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f32(owner, 1, 0, copied, 3) == 0);
    assert(memcmp(copied, states, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_set_outgoing_f32(owner, 1, jumps, 3) == 0);
    assert(liblrhsmm_rs_segmentation_add_jump_f32(owner, 1, 0, 0.25) == 0);
    assert(liblrhsmm_rs_segmentation_copy_outgoing_f32(owner, 1, 0, output, 4) == 0);
    assert(output[2].delta == 0 && output[2].probability == 0.25 && output[3].delta == 1);
    assert(liblrhsmm_rs_segmentation_add_jump_f32(owner, 1, 1, 0.5) == 0);
    assert(liblrhsmm_rs_segmentation_copy_outgoing_f32(owner, 1, 3, output, 1) == 0 && output[0].probability == 0.5);
    assert(liblrhsmm_rs_segmentation_set_outgoing_f32(owner, 1, NULL, 0) == 3);
    assert(liblrhsmm_rs_segmentation_outgoing_count_f32(owner, 1, &count) == 0 && count == 4);
    copied[0] = 77;
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(owner, 3, copied, 1) == 2 && copied[0] == 77);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(owner, 3, NULL, 0) == 0);
    assert(liblrhsmm_rs_segmentation_set_output_states_f32(owner, 2, 0, replacement, 1) == 2);
    assert(liblrhsmm_rs_segmentation_outgoing_count_f32(owner, 3, &count) == 2 && count == 4);
    LiblrhsmmRsJumpSliceF32 malformed[] = {{jumps, 3}, {jumps, 3}, {jumps, 2}};
    assert(liblrhsmm_rs_segmentation_replace_f32(owner, 2, 3, boundaries, durations, rows, malformed) == 3);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(owner, 0, copied, 3) == 0);
    assert(memcmp(copied, replacement, sizeof copied) == 0);
    LiblrhsmmRsStateOccurrence occurrences[] = {{2, 2}, {0, 2}, {2, 7}};
    assert(liblrhsmm_rs_segmentation_resegment_f32(owner, occurrences, 3, &result) == 0);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f32(result, 0, copied, 3) == 0);
    assert(copied[0] == 9 && copied[1] == 3 && copied[2] == 9);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(result, 0, copied, 3) == 0);
    assert(copied[0] == 2 && copied[1] == 2 && copied[2] == 7);
    assert(liblrhsmm_rs_segmentation_outgoing_count_f32(result, 0, &count) == 0 && count == 1);
    assert(liblrhsmm_rs_segmentation_release_f32(&result) == 0);
    occurrences[0].state = 3;
    result = clone;
    assert(liblrhsmm_rs_segmentation_resegment_f32(owner, occurrences, 3, &result) == 3 && result == clone);
    result = NULL;
    assert(liblrhsmm_rs_segmentation_replace_f32(owner, 2, 3, boundaries, durations, rows, groups) == 0);
    states[0] = 99;
    boundaries[0] = 99;
    jumps[0].probability = 99;
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(owner, 0, copied, 3) == 0 && copied[0] == 2);
    assert(liblrhsmm_rs_segmentation_release_f32(&owner) == 0 && owner == NULL);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f32(clone, 0, 0, copied, 3) == 0 && copied[0] == 20);
    assert(liblrhsmm_rs_segmentation_copy_outgoing_f32(clone, 0, 0, output, 3) == 0);
    for (unsigned i = 0; i < 3; i++) assert(memcmp(&output[i].probability, &patterns[i], sizeof patterns[i]) == 0);
    assert(liblrhsmm_rs_segmentation_release_f32(&clone) == 0);
    const int32_t *empty_rows[] = {NULL, NULL};
    assert(liblrhsmm_rs_segmentation_create_f32(2, 0, NULL, NULL, empty_rows, NULL, &owner) == 0);
    assert(liblrhsmm_rs_segmentation_replace_f32(owner, 0, 0, NULL, NULL, NULL, NULL) == 0);
    assert(liblrhsmm_rs_segmentation_layout_f32(owner, &layout) == 0 && layout.streams == 0 && layout.segments == 0);
    assert(liblrhsmm_rs_segmentation_resegment_f32(owner, NULL, 0, &result) == 0);
    assert(liblrhsmm_rs_segmentation_release_f32(&result) == 0);
    assert(liblrhsmm_rs_segmentation_release_f32(&owner) == 0);
    assert(liblrhsmm_rs_segmentation_release_f32(&owner) == 0);
    assert(liblrhsmm_rs_segmentation_release_f32(NULL) == 1);
}

static void exercise_f64(void) {
    LiblrhsmmRsSegmentationF64 *owner = NULL, *clone = NULL, *result = NULL;
    LiblrhsmmRsSegmentationLayout layout = {99, 99};
    int32_t boundaries[] = {2, 4, 8}, durations[] = {10, 11, 12};
    int32_t states[] = {20, INT32_MIN, INT32_MAX}, copied[3];
    const int32_t *rows[] = {states, states};
    uint64_t patterns[] = {UINT64_C(0x7ff8000000001234), UINT64_C(0x8000000000000000), UINT64_C(0x3ff0000000000001)};
    LiblrhsmmRsJumpF64 jumps[3] = {{-2, 0}, {0, 0}, {1, 0}}, output[4];
    for (unsigned i = 0; i < 3; i++) memcpy(&jumps[i].probability, &patterns[i], sizeof patterns[i]);
    LiblrhsmmRsJumpSliceF64 groups[] = {{jumps, 3}, {jumps + 2, 1}, {jumps + 1, 2}};
    assert(liblrhsmm_rs_segmentation_new_f64(2, 3, &owner) == 0);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(owner, 0, copied, 3) == 0);
    assert(copied[0] == 0 && copied[1] == 0 && copied[2] == 0);
    assert(liblrhsmm_rs_segmentation_release_f64(&owner) == 0);
    assert(liblrhsmm_rs_segmentation_create_f64(2, 3, boundaries, durations, rows, groups, &owner) == 0);
    assert(liblrhsmm_rs_segmentation_clone_f64(owner, &clone) == 0);
    assert(liblrhsmm_rs_segmentation_layout_f64(owner, &layout) == 0 && layout.streams == 2 && layout.segments == 3);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f64(owner, 0, copied, 3) == 0);
    assert(memcmp(copied, durations, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f64(owner, 1, 0, copied, 3) == 0);
    assert(memcmp(copied, states, sizeof copied) == 0);
    uintptr_t count = 99;
    assert(liblrhsmm_rs_segmentation_outgoing_count_f64(owner, 0, &count) == 0 && count == 3);
    assert(liblrhsmm_rs_segmentation_copy_outgoing_f64(owner, 0, 0, output, 3) == 0);
    for (unsigned i = 0; i < 3; i++) {
        assert(output[i].delta == jumps[i].delta);
        assert(memcmp(&output[i].probability, &patterns[i], sizeof patterns[i]) == 0);
    }
    int32_t replacement[] = {3, 5, 9};
    assert(liblrhsmm_rs_segmentation_set_boundaries_f64(owner, 0, replacement, 3) == 0);
    assert(liblrhsmm_rs_segmentation_set_duration_states_f64(owner, 0, replacement, 3) == 0);
    assert(liblrhsmm_rs_segmentation_set_output_states_f64(owner, 0, 0, replacement, 3) == 0);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f64(owner, 1, 0, copied, 3) == 0);
    assert(memcmp(copied, states, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_set_outgoing_f64(owner, 1, jumps, 3) == 0);
    assert(liblrhsmm_rs_segmentation_add_jump_f64(owner, 1, 0, 0.25) == 0);
    assert(liblrhsmm_rs_segmentation_copy_outgoing_f64(owner, 1, 0, output, 4) == 0);
    assert(output[2].delta == 0 && output[2].probability == 0.25 && output[3].delta == 1);
    assert(liblrhsmm_rs_segmentation_add_jump_f64(owner, 1, 1, 0.5) == 0);
    assert(liblrhsmm_rs_segmentation_copy_outgoing_f64(owner, 1, 3, output, 1) == 0 && output[0].probability == 0.5);
    assert(liblrhsmm_rs_segmentation_set_outgoing_f64(owner, 1, NULL, 0) == 3);
    assert(liblrhsmm_rs_segmentation_outgoing_count_f64(owner, 1, &count) == 0 && count == 4);
    copied[0] = 77;
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(owner, 3, copied, 1) == 2 && copied[0] == 77);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(owner, 3, NULL, 0) == 0);
    assert(liblrhsmm_rs_segmentation_set_output_states_f64(owner, 2, 0, replacement, 1) == 2);
    assert(liblrhsmm_rs_segmentation_outgoing_count_f64(owner, 3, &count) == 2 && count == 4);
    LiblrhsmmRsJumpSliceF64 malformed[] = {{jumps, 3}, {jumps, 3}, {jumps, 2}};
    assert(liblrhsmm_rs_segmentation_replace_f64(owner, 2, 3, boundaries, durations, rows, malformed) == 3);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(owner, 0, copied, 3) == 0);
    assert(memcmp(copied, replacement, sizeof copied) == 0);
    LiblrhsmmRsStateOccurrence occurrences[] = {{2, 2}, {0, 2}, {2, 7}};
    assert(liblrhsmm_rs_segmentation_resegment_f64(owner, occurrences, 3, &result) == 0);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f64(result, 0, copied, 3) == 0);
    assert(copied[0] == 9 && copied[1] == 3 && copied[2] == 9);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(result, 0, copied, 3) == 0);
    assert(copied[0] == 2 && copied[1] == 2 && copied[2] == 7);
    assert(liblrhsmm_rs_segmentation_outgoing_count_f64(result, 0, &count) == 0 && count == 1);
    assert(liblrhsmm_rs_segmentation_release_f64(&result) == 0);
    occurrences[0].state = 3;
    result = clone;
    assert(liblrhsmm_rs_segmentation_resegment_f64(owner, occurrences, 3, &result) == 3 && result == clone);
    result = NULL;
    assert(liblrhsmm_rs_segmentation_replace_f64(owner, 2, 3, boundaries, durations, rows, groups) == 0);
    states[0] = 99;
    boundaries[0] = 99;
    jumps[0].probability = 99;
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(owner, 0, copied, 3) == 0 && copied[0] == 2);
    assert(liblrhsmm_rs_segmentation_release_f64(&owner) == 0 && owner == NULL);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f64(clone, 0, 0, copied, 3) == 0 && copied[0] == 20);
    assert(liblrhsmm_rs_segmentation_copy_outgoing_f64(clone, 0, 0, output, 3) == 0);
    for (unsigned i = 0; i < 3; i++) assert(memcmp(&output[i].probability, &patterns[i], sizeof patterns[i]) == 0);
    assert(liblrhsmm_rs_segmentation_release_f64(&clone) == 0);
    const int32_t *empty_rows[] = {NULL, NULL};
    assert(liblrhsmm_rs_segmentation_create_f64(2, 0, NULL, NULL, empty_rows, NULL, &owner) == 0);
    assert(liblrhsmm_rs_segmentation_replace_f64(owner, 0, 0, NULL, NULL, NULL, NULL) == 0);
    assert(liblrhsmm_rs_segmentation_layout_f64(owner, &layout) == 0 && layout.streams == 0 && layout.segments == 0);
    assert(liblrhsmm_rs_segmentation_resegment_f64(owner, NULL, 0, &result) == 0);
    assert(liblrhsmm_rs_segmentation_release_f64(&result) == 0);
    assert(liblrhsmm_rs_segmentation_release_f64(&owner) == 0);
    assert(liblrhsmm_rs_segmentation_release_f64(&owner) == 0);
    assert(liblrhsmm_rs_segmentation_release_f64(NULL) == 1);
}
int main(void) { exercise_f32(); exercise_f64(); puts("Segmentation C: all 34 exports passed"); return 0; }
