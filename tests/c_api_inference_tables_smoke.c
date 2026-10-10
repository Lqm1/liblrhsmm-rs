/* Actual calls for complete typed inference table storage in both precisions. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static void check_log_emissions_f32(const LiblrhsmmRsLogEmissionsF32 *owner, const float *expected) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout = {99, 99, 99};
    float copied[6], value = -77;
    assert(liblrhsmm_rs_log_emissions_layout_f32(owner, &layout) == 0 && layout.frames == 2 && layout.states == 3 && layout.values == 6);
    assert(liblrhsmm_rs_log_emissions_copy_values_f32(owner, 0, copied, 6) == 0 && memcmp(copied, expected, sizeof copied) == 0);
    assert(liblrhsmm_rs_log_emissions_copy_frame_f32(owner, 1, copied, 3) == 0 && memcmp(copied, expected + 3, 3*sizeof(float)) == 0);
    for (uintptr_t frame = 0; frame < 2; frame++) for (uintptr_t state = 0; state < 3; state++) {
        assert(liblrhsmm_rs_log_emissions_get_f32(owner, frame, state, &value) == 0);
        assert(memcmp(&value, expected + frame*3+state, sizeof value) == 0);
    }
    value = -77;
    assert(liblrhsmm_rs_log_emissions_copy_values_f32(owner, 6, &value, 1) == 2 && value == -77);
    assert(liblrhsmm_rs_log_emissions_copy_values_f32(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_log_emissions_copy_values_f32(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_log_emissions_get_f32(owner, 2, 0, &value) == 2 && value == -77);
    assert(liblrhsmm_rs_log_emissions_copy_frame_f32(owner, 0, &value, 2) == 2 && value == -77);
}

static void check_state_scores_f32(const LiblrhsmmRsStateScoresF32 *owner, const float *expected) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout = {99, 99, 99};
    float copied[6], value = -77;
    assert(liblrhsmm_rs_state_scores_layout_f32(owner, &layout) == 0 && layout.frames == 2 && layout.states == 3 && layout.values == 6);
    assert(liblrhsmm_rs_state_scores_copy_values_f32(owner, 0, copied, 6) == 0 && memcmp(copied, expected, sizeof copied) == 0);
    assert(liblrhsmm_rs_state_scores_copy_frame_f32(owner, 1, copied, 3) == 0 && memcmp(copied, expected + 3, 3*sizeof(float)) == 0);
    for (uintptr_t frame = 0; frame < 2; frame++) for (uintptr_t state = 0; state < 3; state++) {
        assert(liblrhsmm_rs_state_scores_get_f32(owner, frame, state, &value) == 0);
        assert(memcmp(&value, expected + frame*3+state, sizeof value) == 0);
    }
    value = -77;
    assert(liblrhsmm_rs_state_scores_copy_values_f32(owner, 6, &value, 1) == 2 && value == -77);
    assert(liblrhsmm_rs_state_scores_copy_values_f32(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_state_scores_copy_values_f32(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_state_scores_get_f32(owner, 2, 0, &value) == 2 && value == -77);
    assert(liblrhsmm_rs_state_scores_copy_frame_f32(owner, 0, &value, 2) == 2 && value == -77);
}

static void check_emission_densities_f32(const LiblrhsmmRsEmissionDensitiesF32 *owner, const float *expected) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout = {99, 99, 99};
    float copied[6], value = -77;
    assert(liblrhsmm_rs_emission_densities_layout_f32(owner, &layout) == 0 && layout.frames == 2 && layout.states == 3 && layout.values == 6);
    assert(liblrhsmm_rs_emission_densities_copy_values_f32(owner, 0, copied, 6) == 0 && memcmp(copied, expected, sizeof copied) == 0);
    assert(liblrhsmm_rs_emission_densities_copy_frame_f32(owner, 1, copied, 3) == 0 && memcmp(copied, expected + 3, 3*sizeof(float)) == 0);
    for (uintptr_t frame = 0; frame < 2; frame++) for (uintptr_t state = 0; state < 3; state++) {
        assert(liblrhsmm_rs_emission_densities_get_f32(owner, frame, state, &value) == 0);
        assert(memcmp(&value, expected + frame*3+state, sizeof value) == 0);
    }
    value = -77;
    assert(liblrhsmm_rs_emission_densities_copy_values_f32(owner, 6, &value, 1) == 2 && value == -77);
    assert(liblrhsmm_rs_emission_densities_copy_values_f32(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_emission_densities_copy_values_f32(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_emission_densities_get_f32(owner, 2, 0, &value) == 2 && value == -77);
    assert(liblrhsmm_rs_emission_densities_copy_frame_f32(owner, 0, &value, 2) == 2 && value == -77);
}

static void exercise_f32(void) {
    uint32_t finite_bits[] = {0, 0x80000000, 1, 0x80000001, 0x3f800001, 0xbf000000}, raw_bits[] = {0x7fc01234, 0x80000000, 0x7f800000, 0xff800000, 1, 0x3f800001};
    float finite[6], raw[6], density_values[6];
    memcpy(finite, finite_bits, sizeof finite);
    memcpy(raw, raw_bits, sizeof raw);
    LiblrhsmmRsLogEmissionsF32 *log = NULL, *log_clone = NULL;
    LiblrhsmmRsStateScoresF32 *scores = NULL, *score_clone = NULL;
    LiblrhsmmRsEmissionDensitiesF32 *density = NULL, *density_clone = NULL;
    assert(liblrhsmm_rs_log_emissions_create_f32(2, 3, finite, 6, &log) == 0);
    assert(liblrhsmm_rs_state_scores_create_f32(2, 3, raw, 6, &scores) == 0);
    assert(liblrhsmm_rs_log_emissions_probabilities_f32(log, &density) == 0);
    assert(liblrhsmm_rs_emission_densities_copy_values_f32(density, 0, density_values, 6) == 0 && density_values[0] == 1 && density_values[1] == 1);
    check_log_emissions_f32(log, finite);
    check_state_scores_f32(scores, raw);
    check_emission_densities_f32(density, density_values);
    assert(liblrhsmm_rs_log_emissions_clone_f32(log, &log_clone) == 0);
    assert(liblrhsmm_rs_state_scores_clone_f32(scores, &score_clone) == 0);
    assert(liblrhsmm_rs_emission_densities_clone_f32(density, &density_clone) == 0);
    float replacement[] = {-0.5, -1, -1.5, -2, -2.5, -3};
    assert(liblrhsmm_rs_log_emissions_replace_f32(log, 2, 3, replacement, 6) == 0);
    assert(liblrhsmm_rs_state_scores_replace_f32(scores, 2, 3, replacement, 6) == 0);
    check_log_emissions_f32(log, replacement);
    check_state_scores_f32(scores, replacement);
    float invalid[] = {-0.5, -1, -1.5, -2, -2.5, NAN};
    assert(liblrhsmm_rs_log_emissions_replace_f32(log, 2, 3, invalid, 6) == 3);
    assert(liblrhsmm_rs_state_scores_replace_f32(scores, 2, 4, replacement, 6) == 3);
    check_log_emissions_f32(log, replacement);
    check_state_scores_f32(scores, replacement);
    assert(liblrhsmm_rs_state_scores_replace_f32(score_clone, 2, 3, replacement, 6) == 0);
    float total = 99, expected = -1e15;
    for (unsigned i = 3; i < 6; i++) assert(liblrhsmm_rs_log_add_f32(expected, replacement[i], &expected) == 0);
    assert(liblrhsmm_rs_state_scores_total_f32(scores, &total) == 0 && memcmp(&total, &expected, sizeof total) == 0);
    expected = -1e15;
    for (unsigned i = 0; i < 2; i++) assert(liblrhsmm_rs_log_add_f32(expected, replacement[i*3]+replacement[i*3], &expected) == 0);
    assert(liblrhsmm_rs_state_scores_total_through_state_f32(scores, score_clone, 0, &total) == 0 && memcmp(&total, &expected, sizeof total) == 0);
    assert(liblrhsmm_rs_state_scores_backward_total_f32(score_clone, log, &total) == 0 && total == -1);
    total = 77;
    assert(liblrhsmm_rs_state_scores_total_through_state_f32(scores, score_clone, 3, &total) == 3 && total == 77);
    assert(liblrhsmm_rs_log_emissions_release_f32(&log) == 0);
    assert(liblrhsmm_rs_state_scores_release_f32(&scores) == 0);
    assert(liblrhsmm_rs_emission_densities_release_f32(&density) == 0);
    check_log_emissions_f32(log_clone, finite);
    check_state_scores_f32(score_clone, replacement);
    check_emission_densities_f32(density_clone, density_values);
    assert(liblrhsmm_rs_log_emissions_release_f32(&log_clone) == 0);
    assert(liblrhsmm_rs_state_scores_release_f32(&score_clone) == 0);
    assert(liblrhsmm_rs_emission_densities_release_f32(&density_clone) == 0);
    assert(liblrhsmm_rs_log_emissions_release_f32(&log_clone) == 0);
    assert(liblrhsmm_rs_state_scores_release_f32(&score_clone) == 0);
    assert(liblrhsmm_rs_emission_densities_release_f32(&density_clone) == 0);
}

static void check_log_emissions_f64(const LiblrhsmmRsLogEmissionsF64 *owner, const double *expected) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout = {99, 99, 99};
    _Alignas(8) double copied[6], value = -77;
    assert(liblrhsmm_rs_log_emissions_layout_f64(owner, &layout) == 0 && layout.frames == 2 && layout.states == 3 && layout.values == 6);
    assert(liblrhsmm_rs_log_emissions_copy_values_f64(owner, 0, copied, 6) == 0 && memcmp(copied, expected, sizeof copied) == 0);
    assert(liblrhsmm_rs_log_emissions_copy_frame_f64(owner, 1, copied, 3) == 0 && memcmp(copied, expected + 3, 3*sizeof(double)) == 0);
    for (uintptr_t frame = 0; frame < 2; frame++) for (uintptr_t state = 0; state < 3; state++) {
        assert(liblrhsmm_rs_log_emissions_get_f64(owner, frame, state, &value) == 0);
        assert(memcmp(&value, expected + frame*3+state, sizeof value) == 0);
    }
    value = -77;
    assert(liblrhsmm_rs_log_emissions_copy_values_f64(owner, 6, &value, 1) == 2 && value == -77);
    assert(liblrhsmm_rs_log_emissions_copy_values_f64(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_log_emissions_copy_values_f64(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_log_emissions_get_f64(owner, 2, 0, &value) == 2 && value == -77);
    assert(liblrhsmm_rs_log_emissions_copy_frame_f64(owner, 0, &value, 2) == 2 && value == -77);
}

static void check_state_scores_f64(const LiblrhsmmRsStateScoresF64 *owner, const double *expected) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout = {99, 99, 99};
    _Alignas(8) double copied[6], value = -77;
    assert(liblrhsmm_rs_state_scores_layout_f64(owner, &layout) == 0 && layout.frames == 2 && layout.states == 3 && layout.values == 6);
    assert(liblrhsmm_rs_state_scores_copy_values_f64(owner, 0, copied, 6) == 0 && memcmp(copied, expected, sizeof copied) == 0);
    assert(liblrhsmm_rs_state_scores_copy_frame_f64(owner, 1, copied, 3) == 0 && memcmp(copied, expected + 3, 3*sizeof(double)) == 0);
    for (uintptr_t frame = 0; frame < 2; frame++) for (uintptr_t state = 0; state < 3; state++) {
        assert(liblrhsmm_rs_state_scores_get_f64(owner, frame, state, &value) == 0);
        assert(memcmp(&value, expected + frame*3+state, sizeof value) == 0);
    }
    value = -77;
    assert(liblrhsmm_rs_state_scores_copy_values_f64(owner, 6, &value, 1) == 2 && value == -77);
    assert(liblrhsmm_rs_state_scores_copy_values_f64(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_state_scores_copy_values_f64(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_state_scores_get_f64(owner, 2, 0, &value) == 2 && value == -77);
    assert(liblrhsmm_rs_state_scores_copy_frame_f64(owner, 0, &value, 2) == 2 && value == -77);
}

static void check_emission_densities_f64(const LiblrhsmmRsEmissionDensitiesF64 *owner, const double *expected) {
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout = {99, 99, 99};
    _Alignas(8) double copied[6], value = -77;
    assert(liblrhsmm_rs_emission_densities_layout_f64(owner, &layout) == 0 && layout.frames == 2 && layout.states == 3 && layout.values == 6);
    assert(liblrhsmm_rs_emission_densities_copy_values_f64(owner, 0, copied, 6) == 0 && memcmp(copied, expected, sizeof copied) == 0);
    assert(liblrhsmm_rs_emission_densities_copy_frame_f64(owner, 1, copied, 3) == 0 && memcmp(copied, expected + 3, 3*sizeof(double)) == 0);
    for (uintptr_t frame = 0; frame < 2; frame++) for (uintptr_t state = 0; state < 3; state++) {
        assert(liblrhsmm_rs_emission_densities_get_f64(owner, frame, state, &value) == 0);
        assert(memcmp(&value, expected + frame*3+state, sizeof value) == 0);
    }
    value = -77;
    assert(liblrhsmm_rs_emission_densities_copy_values_f64(owner, 6, &value, 1) == 2 && value == -77);
    assert(liblrhsmm_rs_emission_densities_copy_values_f64(owner, 6, NULL, 0) == 0);
    assert(liblrhsmm_rs_emission_densities_copy_values_f64(owner, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_emission_densities_get_f64(owner, 2, 0, &value) == 2 && value == -77);
    assert(liblrhsmm_rs_emission_densities_copy_frame_f64(owner, 0, &value, 2) == 2 && value == -77);
}

static void exercise_f64(void) {
    uint64_t finite_bits[] = {0, UINT64_C(0x8000000000000000), 1, UINT64_C(0x8000000000000001), UINT64_C(0x3ff0000000000001), UINT64_C(0xbfe0000000000000)}, raw_bits[] = {UINT64_C(0x7ff8000000001234), UINT64_C(0x8000000000000000), UINT64_C(0x7ff0000000000000), UINT64_C(0xfff0000000000000), 1, UINT64_C(0x3ff0000000000001)};
    _Alignas(8) double finite[6], raw[6], density_values[6];
    memcpy(finite, finite_bits, sizeof finite);
    memcpy(raw, raw_bits, sizeof raw);
    LiblrhsmmRsLogEmissionsF64 *log = NULL, *log_clone = NULL;
    LiblrhsmmRsStateScoresF64 *scores = NULL, *score_clone = NULL;
    LiblrhsmmRsEmissionDensitiesF64 *density = NULL, *density_clone = NULL;
    assert(liblrhsmm_rs_log_emissions_create_f64(2, 3, finite, 6, &log) == 0);
    assert(liblrhsmm_rs_state_scores_create_f64(2, 3, raw, 6, &scores) == 0);
    assert(liblrhsmm_rs_log_emissions_probabilities_f64(log, &density) == 0);
    assert(liblrhsmm_rs_emission_densities_copy_values_f64(density, 0, density_values, 6) == 0 && density_values[0] == 1 && density_values[1] == 1);
    check_log_emissions_f64(log, finite);
    check_state_scores_f64(scores, raw);
    check_emission_densities_f64(density, density_values);
    assert(liblrhsmm_rs_log_emissions_clone_f64(log, &log_clone) == 0);
    assert(liblrhsmm_rs_state_scores_clone_f64(scores, &score_clone) == 0);
    assert(liblrhsmm_rs_emission_densities_clone_f64(density, &density_clone) == 0);
    _Alignas(8) double replacement[] = {-0.5, -1, -1.5, -2, -2.5, -3};
    assert(liblrhsmm_rs_log_emissions_replace_f64(log, 2, 3, replacement, 6) == 0);
    assert(liblrhsmm_rs_state_scores_replace_f64(scores, 2, 3, replacement, 6) == 0);
    check_log_emissions_f64(log, replacement);
    check_state_scores_f64(scores, replacement);
    _Alignas(8) double invalid[] = {-0.5, -1, -1.5, -2, -2.5, NAN};
    assert(liblrhsmm_rs_log_emissions_replace_f64(log, 2, 3, invalid, 6) == 3);
    assert(liblrhsmm_rs_state_scores_replace_f64(scores, 2, 4, replacement, 6) == 3);
    check_log_emissions_f64(log, replacement);
    check_state_scores_f64(scores, replacement);
    assert(liblrhsmm_rs_state_scores_replace_f64(score_clone, 2, 3, replacement, 6) == 0);
    _Alignas(8) double total = 99, expected = -1e15;
    for (unsigned i = 3; i < 6; i++) assert(liblrhsmm_rs_log_add_f64(expected, replacement[i], &expected) == 0);
    assert(liblrhsmm_rs_state_scores_total_f64(scores, &total) == 0 && memcmp(&total, &expected, sizeof total) == 0);
    expected = -1e15;
    for (unsigned i = 0; i < 2; i++) assert(liblrhsmm_rs_log_add_f64(expected, replacement[i*3]+replacement[i*3], &expected) == 0);
    assert(liblrhsmm_rs_state_scores_total_through_state_f64(scores, score_clone, 0, &total) == 0 && memcmp(&total, &expected, sizeof total) == 0);
    assert(liblrhsmm_rs_state_scores_backward_total_f64(score_clone, log, &total) == 0 && total == -1);
    total = 77;
    assert(liblrhsmm_rs_state_scores_total_through_state_f64(scores, score_clone, 3, &total) == 3 && total == 77);
    assert(liblrhsmm_rs_log_emissions_release_f64(&log) == 0);
    assert(liblrhsmm_rs_state_scores_release_f64(&scores) == 0);
    assert(liblrhsmm_rs_emission_densities_release_f64(&density) == 0);
    check_log_emissions_f64(log_clone, finite);
    check_state_scores_f64(score_clone, replacement);
    check_emission_densities_f64(density_clone, density_values);
    assert(liblrhsmm_rs_log_emissions_release_f64(&log_clone) == 0);
    assert(liblrhsmm_rs_state_scores_release_f64(&score_clone) == 0);
    assert(liblrhsmm_rs_emission_densities_release_f64(&density_clone) == 0);
    assert(liblrhsmm_rs_log_emissions_release_f64(&log_clone) == 0);
    assert(liblrhsmm_rs_state_scores_release_f64(&score_clone) == 0);
    assert(liblrhsmm_rs_emission_densities_release_f64(&density_clone) == 0);
}
int main(void) { exercise_f32(); exercise_f64(); puts("Inference tables C: all52 exports and independent table storage passed"); return 0; }
