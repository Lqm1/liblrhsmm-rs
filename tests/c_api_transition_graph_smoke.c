/* Complete cached graph calls, including unchanged original C transition records. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static void check_f32(const LiblrhsmmRsTransitionGraphF32 *graph) {
    LiblrhsmmRsSegmentationLayout layout = {99, 99};
    assert(liblrhsmm_rs_transition_graph_layout_f32(graph, &layout) == 0);
    assert(layout.streams == 2 && layout.segments == 4);
    FILE *fixture = fopen("tests/fixtures/c-transitions.txt", "r");
    assert(fixture != NULL);
    int pass, segment, index, delta, records = 0;
    float probability;
    while (fscanf(fixture, "%d %d %d %d %f", &pass, &segment, &index, &delta, &probability) == 5) {
        LiblrhsmmRsJumpF32 actual;
        float expected = (float)probability;
        uintptr_t count = 99;
        assert(liblrhsmm_rs_transition_graph_incoming_count_f32(graph, (uintptr_t)segment, &count) == 0);
        assert(count == (segment == 0 || segment == 2 ? 2 : segment == 1 ? 3 : 1));
        assert(liblrhsmm_rs_transition_graph_copy_incoming_f32(graph, (uintptr_t)segment, (uintptr_t)index, &actual, 1) == 0);
        if (actual.delta != delta || memcmp(&actual.probability, &expected, sizeof expected) != 0)
            fprintf(stderr, "f32 cache mismatch: segment=%d index=%d delta=%d/%d probability=%.17g/%.17g\n", segment, index, actual.delta, delta, (double)actual.probability, (double)expected);
        assert(actual.delta == delta && memcmp(&actual.probability, &expected, sizeof expected) == 0);
        assert(liblrhsmm_rs_transition_graph_copy_incoming_f32(graph, (uintptr_t)segment, count, NULL, 0) == 0);
        records++;
    }
    assert(records == 16);
    fclose(fixture);
    LiblrhsmmRsJumpF32 actual[3];
    uintptr_t count = 99;
    assert(liblrhsmm_rs_transition_graph_outgoing_count_f32(graph, 0, &count) == 0 && count == 3);
    assert(liblrhsmm_rs_transition_graph_copy_outgoing_f32(graph, 0, 0, actual, 3) == 0);
    assert(actual[0].delta == 2 && actual[1].delta == -100 && actual[2].delta == 1);
    float expected[] = {(float)0.2f, (float)0.1f, (float)0.6f};
    for (unsigned i = 0; i < 3; i++) assert(memcmp(&actual[i].probability, &expected[i], sizeof expected[i]) == 0);
}
static void exercise_f32(void) {
    LiblrhsmmRsSegmentationF32 *source = NULL, *child = NULL;
    LiblrhsmmRsTransitionGraphF32 *graph = NULL, *clone = NULL;
    assert(liblrhsmm_rs_segmentation_new_f32(2, 4, &source) == 0);
    int32_t boundaries[] = {INT32_MIN, 3, 2, INT32_MAX}, durations[] = {INT32_MAX, -1, 7, INT32_MIN};
    int32_t states[] = {2, -3, 5, INT32_MIN}, other[] = {INT32_MAX, 8, 1, 9}, copied[4];
    assert(liblrhsmm_rs_segmentation_set_boundaries_f32(source, 0, boundaries, 4) == 0);
    assert(liblrhsmm_rs_segmentation_set_duration_states_f32(source, 0, durations, 4) == 0);
    assert(liblrhsmm_rs_segmentation_set_output_states_f32(source, 0, 0, states, 4) == 0);
    assert(liblrhsmm_rs_segmentation_set_output_states_f32(source, 1, 0, other, 4) == 0);
    int sources[] = {0, 1, 2, 3, 0, 3}, deltas[] = {2, 0, -2, -2, -100, 100};
    float probabilities[] = {0.2f, 0.3f, 0.4f, 0.5f, 0.1f, 0.1f};
    for (unsigned i = 0; i < 6; i++) assert(liblrhsmm_rs_segmentation_add_jump_f32(source, sources[i], deltas[i], (float)probabilities[i]) == 0);
    for (unsigned state = 0; state < 4; state++) {
        /* Store binary32 before widening: i686 GCC may evaluate float expressions in x87 extended precision. */
        volatile float weight = 0.6f + state * 0.1f;
        assert(liblrhsmm_rs_segmentation_add_jump_f32(source, state, 1, (float)weight) == 0);
    }
    assert(liblrhsmm_rs_segmentation_build_transitions_f32(source, &graph) == 0);
    check_f32(graph);
    assert(liblrhsmm_rs_transition_graph_clone_f32(graph, &clone) == 0);
    assert(liblrhsmm_rs_transition_graph_get_segmentation_f32(graph, &child) == 0);
    assert(liblrhsmm_rs_segmentation_add_jump_f32(source, 0, 0, 0.25) == 0);
    assert(liblrhsmm_rs_segmentation_release_f32(&source) == 0);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f32(child, 0, copied, 4) == 0 && memcmp(copied, boundaries, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f32(child, 0, copied, 4) == 0 && memcmp(copied, durations, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f32(child, 0, 0, copied, 4) == 0 && memcmp(copied, states, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f32(child, 1, 0, copied, 4) == 0 && memcmp(copied, other, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_add_jump_f32(child, 0, 0, 0.75) == 0);
    check_f32(graph);
    check_f32(clone);
    uintptr_t count = 77;
    assert(liblrhsmm_rs_transition_graph_incoming_count_f32(graph, 4, &count) == 2 && count == 77);
    assert(liblrhsmm_rs_transition_graph_outgoing_count_f32(graph, 4, &count) == 2 && count == 77);
    LiblrhsmmRsJumpF32 untouched = {77, -0.0};
    assert(liblrhsmm_rs_transition_graph_copy_incoming_f32(graph, 0, UINTPTR_MAX, &untouched, 1) == 2 && untouched.delta == 77 && signbit(untouched.probability));
    assert(liblrhsmm_rs_transition_graph_copy_outgoing_f32(graph, 0, 3, &untouched, 1) == 2 && untouched.delta == 77);
    assert(liblrhsmm_rs_transition_graph_copy_incoming_f32(graph, 0, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_transition_graph_copy_outgoing_f32(graph, 0, 3, NULL, 0) == 0);
    LiblrhsmmRsTransitionGraphF32 *sentinel = graph;
    LiblrhsmmRsJumpF32 invalid[] = {{1, NAN}};
    assert(liblrhsmm_rs_segmentation_set_outgoing_f32(child, 3, invalid, 1) == 0);
    assert(liblrhsmm_rs_segmentation_build_transitions_f32(child, &sentinel) == 3 && sentinel == graph);
    assert(liblrhsmm_rs_transition_graph_clone_f32(NULL, &sentinel) == 1 && sentinel == graph);
    assert(liblrhsmm_rs_transition_graph_get_segmentation_f32(graph, NULL) == 1);
    assert(liblrhsmm_rs_transition_graph_release_f32(&graph) == 0 && graph == NULL);
    check_f32(clone);
    assert(liblrhsmm_rs_segmentation_release_f32(&child) == 0);
    assert(liblrhsmm_rs_transition_graph_release_f32(&clone) == 0);
    assert(liblrhsmm_rs_transition_graph_release_f32(&clone) == 0);
    assert(liblrhsmm_rs_transition_graph_release_f32(NULL) == 1);
    for (unsigned streams = 0; streams <= 2; streams += 2) {
        assert(liblrhsmm_rs_segmentation_new_f32(streams, 0, &source) == 0);
        assert(liblrhsmm_rs_segmentation_build_transitions_f32(source, &graph) == 0);
        LiblrhsmmRsSegmentationLayout layout = {99, 99};
        assert(liblrhsmm_rs_transition_graph_layout_f32(graph, &layout) == 0 && layout.streams == streams && layout.segments == 0);
        assert(liblrhsmm_rs_transition_graph_incoming_count_f32(graph, 0, &count) == 2);
        assert(liblrhsmm_rs_segmentation_release_f32(&source) == 0);
        assert(liblrhsmm_rs_transition_graph_release_f32(&graph) == 0);
    }
}

static void check_f64(const LiblrhsmmRsTransitionGraphF64 *graph) {
    LiblrhsmmRsSegmentationLayout layout = {99, 99};
    assert(liblrhsmm_rs_transition_graph_layout_f64(graph, &layout) == 0);
    assert(layout.streams == 2 && layout.segments == 4);
    FILE *fixture = fopen("tests/fixtures/c-transitions.txt", "r");
    assert(fixture != NULL);
    int pass, segment, index, delta, records = 0;
    float probability;
    while (fscanf(fixture, "%d %d %d %d %f", &pass, &segment, &index, &delta, &probability) == 5) {
        LiblrhsmmRsJumpF64 actual;
        double expected = (double)probability;
        uintptr_t count = 99;
        assert(liblrhsmm_rs_transition_graph_incoming_count_f64(graph, (uintptr_t)segment, &count) == 0);
        assert(count == (segment == 0 || segment == 2 ? 2 : segment == 1 ? 3 : 1));
        assert(liblrhsmm_rs_transition_graph_copy_incoming_f64(graph, (uintptr_t)segment, (uintptr_t)index, &actual, 1) == 0);
        if (actual.delta != delta || memcmp(&actual.probability, &expected, sizeof expected) != 0)
            fprintf(stderr, "f64 cache mismatch: segment=%d index=%d delta=%d/%d probability=%.17g/%.17g\n", segment, index, actual.delta, delta, (double)actual.probability, (double)expected);
        assert(actual.delta == delta && memcmp(&actual.probability, &expected, sizeof expected) == 0);
        assert(liblrhsmm_rs_transition_graph_copy_incoming_f64(graph, (uintptr_t)segment, count, NULL, 0) == 0);
        records++;
    }
    assert(records == 16);
    fclose(fixture);
    LiblrhsmmRsJumpF64 actual[3];
    uintptr_t count = 99;
    assert(liblrhsmm_rs_transition_graph_outgoing_count_f64(graph, 0, &count) == 0 && count == 3);
    assert(liblrhsmm_rs_transition_graph_copy_outgoing_f64(graph, 0, 0, actual, 3) == 0);
    assert(actual[0].delta == 2 && actual[1].delta == -100 && actual[2].delta == 1);
    double expected[] = {(double)0.2f, (double)0.1f, (double)0.6f};
    for (unsigned i = 0; i < 3; i++) assert(memcmp(&actual[i].probability, &expected[i], sizeof expected[i]) == 0);
}
static void exercise_f64(void) {
    LiblrhsmmRsSegmentationF64 *source = NULL, *child = NULL;
    LiblrhsmmRsTransitionGraphF64 *graph = NULL, *clone = NULL;
    assert(liblrhsmm_rs_segmentation_new_f64(2, 4, &source) == 0);
    int32_t boundaries[] = {INT32_MIN, 3, 2, INT32_MAX}, durations[] = {INT32_MAX, -1, 7, INT32_MIN};
    int32_t states[] = {2, -3, 5, INT32_MIN}, other[] = {INT32_MAX, 8, 1, 9}, copied[4];
    assert(liblrhsmm_rs_segmentation_set_boundaries_f64(source, 0, boundaries, 4) == 0);
    assert(liblrhsmm_rs_segmentation_set_duration_states_f64(source, 0, durations, 4) == 0);
    assert(liblrhsmm_rs_segmentation_set_output_states_f64(source, 0, 0, states, 4) == 0);
    assert(liblrhsmm_rs_segmentation_set_output_states_f64(source, 1, 0, other, 4) == 0);
    int sources[] = {0, 1, 2, 3, 0, 3}, deltas[] = {2, 0, -2, -2, -100, 100};
    float probabilities[] = {0.2f, 0.3f, 0.4f, 0.5f, 0.1f, 0.1f};
    for (unsigned i = 0; i < 6; i++) assert(liblrhsmm_rs_segmentation_add_jump_f64(source, sources[i], deltas[i], (double)probabilities[i]) == 0);
    for (unsigned state = 0; state < 4; state++) {
        /* Store binary32 before widening: i686 GCC may evaluate float expressions in x87 extended precision. */
        volatile float weight = 0.6f + state * 0.1f;
        assert(liblrhsmm_rs_segmentation_add_jump_f64(source, state, 1, (double)weight) == 0);
    }
    assert(liblrhsmm_rs_segmentation_build_transitions_f64(source, &graph) == 0);
    check_f64(graph);
    assert(liblrhsmm_rs_transition_graph_clone_f64(graph, &clone) == 0);
    assert(liblrhsmm_rs_transition_graph_get_segmentation_f64(graph, &child) == 0);
    assert(liblrhsmm_rs_segmentation_add_jump_f64(source, 0, 0, 0.25) == 0);
    assert(liblrhsmm_rs_segmentation_release_f64(&source) == 0);
    assert(liblrhsmm_rs_segmentation_copy_boundaries_f64(child, 0, copied, 4) == 0 && memcmp(copied, boundaries, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_copy_duration_states_f64(child, 0, copied, 4) == 0 && memcmp(copied, durations, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f64(child, 0, 0, copied, 4) == 0 && memcmp(copied, states, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_copy_output_states_f64(child, 1, 0, copied, 4) == 0 && memcmp(copied, other, sizeof copied) == 0);
    assert(liblrhsmm_rs_segmentation_add_jump_f64(child, 0, 0, 0.75) == 0);
    check_f64(graph);
    check_f64(clone);
    uintptr_t count = 77;
    assert(liblrhsmm_rs_transition_graph_incoming_count_f64(graph, 4, &count) == 2 && count == 77);
    assert(liblrhsmm_rs_transition_graph_outgoing_count_f64(graph, 4, &count) == 2 && count == 77);
    LiblrhsmmRsJumpF64 untouched = {77, -0.0};
    assert(liblrhsmm_rs_transition_graph_copy_incoming_f64(graph, 0, UINTPTR_MAX, &untouched, 1) == 2 && untouched.delta == 77 && signbit(untouched.probability));
    assert(liblrhsmm_rs_transition_graph_copy_outgoing_f64(graph, 0, 3, &untouched, 1) == 2 && untouched.delta == 77);
    assert(liblrhsmm_rs_transition_graph_copy_incoming_f64(graph, 0, 0, NULL, 1) == 1);
    assert(liblrhsmm_rs_transition_graph_copy_outgoing_f64(graph, 0, 3, NULL, 0) == 0);
    LiblrhsmmRsTransitionGraphF64 *sentinel = graph;
    LiblrhsmmRsJumpF64 invalid[] = {{1, NAN}};
    assert(liblrhsmm_rs_segmentation_set_outgoing_f64(child, 3, invalid, 1) == 0);
    assert(liblrhsmm_rs_segmentation_build_transitions_f64(child, &sentinel) == 3 && sentinel == graph);
    assert(liblrhsmm_rs_transition_graph_clone_f64(NULL, &sentinel) == 1 && sentinel == graph);
    assert(liblrhsmm_rs_transition_graph_get_segmentation_f64(graph, NULL) == 1);
    assert(liblrhsmm_rs_transition_graph_release_f64(&graph) == 0 && graph == NULL);
    check_f64(clone);
    assert(liblrhsmm_rs_segmentation_release_f64(&child) == 0);
    assert(liblrhsmm_rs_transition_graph_release_f64(&clone) == 0);
    assert(liblrhsmm_rs_transition_graph_release_f64(&clone) == 0);
    assert(liblrhsmm_rs_transition_graph_release_f64(NULL) == 1);
    for (unsigned streams = 0; streams <= 2; streams += 2) {
        assert(liblrhsmm_rs_segmentation_new_f64(streams, 0, &source) == 0);
        assert(liblrhsmm_rs_segmentation_build_transitions_f64(source, &graph) == 0);
        LiblrhsmmRsSegmentationLayout layout = {99, 99};
        assert(liblrhsmm_rs_transition_graph_layout_f64(graph, &layout) == 0 && layout.streams == streams && layout.segments == 0);
        assert(liblrhsmm_rs_transition_graph_incoming_count_f64(graph, 0, &count) == 2);
        assert(liblrhsmm_rs_segmentation_release_f64(&source) == 0);
        assert(liblrhsmm_rs_transition_graph_release_f64(&graph) == 0);
    }
}
int main(void) { exercise_f32(); exercise_f64(); puts("Transition graph C: all18 exports and original C caches passed"); return 0; }
