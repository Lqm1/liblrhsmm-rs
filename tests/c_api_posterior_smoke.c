#include "liblrhsmm_rs.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

static void sparse_storage(void) {
    float weights[4] = {0.0f, -0.0f, 0.0f, 2.0f};
    uint32_t smallest = 1;
    memcpy(&weights[2], &smallest, sizeof(smallest));
    float expected[4]; memcpy(expected, weights, sizeof(weights));
    _Alignas(8) LiblrhsmmRsPosteriorInput input[3] = {
        {0, UINTPTR_MAX, NULL, UINTPTR_MAX},
        {1, UINTPTR_MAX, NULL, 0}, {1, 3, weights, 4}
    };
    LiblrhsmmRsDurationOccupancies *duration = NULL, *duration_clone = NULL;
    LiblrhsmmRsMixtureOccupancies *mixture = NULL, *mixture_clone = NULL;
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout;
    _Alignas(8) LiblrhsmmRsPosteriorSlice slice;
    float copied[4], probability;
    assert(liblrhsmm_rs_duration_occupancies_create(1, 3, input, 3, &duration) == 0);
    assert(liblrhsmm_rs_mixture_occupancies_create(1, 3, input, 3, &mixture) == 0);
    assert(liblrhsmm_rs_duration_occupancies_clone(duration, &duration_clone) == 0);
    assert(liblrhsmm_rs_mixture_occupancies_clone(mixture, &mixture_clone) == 0);
    assert(liblrhsmm_rs_duration_occupancies_layout(duration, &layout) == 0);
    assert(layout.frames == 1 && layout.states == 3 && layout.values == 3);
    assert(liblrhsmm_rs_mixture_occupancies_layout(mixture, &layout) == 0);
    assert(layout.frames == 1 && layout.states == 3 && layout.values == 3);
    for (size_t state = 0; state < 3; ++state) {
        assert(liblrhsmm_rs_duration_occupancies_get_slice(duration, 0, state, &slice) == 0);
        assert(slice.present == (state != 0));
        assert(slice.values == (state == 2 ? 4 : 0));
        assert(slice.minimum == (state == 0 ? 0 : state == 1 ? UINTPTR_MAX : 3));
        assert(liblrhsmm_rs_mixture_occupancies_get_slice(mixture, 0, state, &slice) == 0);
        assert(slice.present == (state != 0) && slice.minimum == 0);
        assert(slice.values == (state == 2 ? 4 : 0));
    }
    weights[3] = NAN;
    assert(liblrhsmm_rs_duration_occupancies_replace(duration, 3, 1, input, 3) == 3);
    assert(liblrhsmm_rs_mixture_occupancies_replace(mixture, 3, 1, input, 3) == 3);
    assert(liblrhsmm_rs_duration_occupancies_copy_values(duration, 0, 2, 0, copied, 4) == 0);
    assert(memcmp(copied, expected, sizeof(copied)) == 0);
    assert(liblrhsmm_rs_mixture_occupancies_copy_values(mixture, 0, 2, 0, copied, 4) == 0);
    assert(memcmp(copied, expected, sizeof(copied)) == 0);
    for (size_t index = 0; index < 4; ++index) {
        assert(liblrhsmm_rs_duration_occupancies_probability(duration, 0, 2, index + 3, &probability) == 0);
        assert(memcmp(&probability, &expected[index], sizeof(probability)) == 0);
    }
    assert(liblrhsmm_rs_duration_occupancies_copy_values(duration, 0, 0, 0, NULL, 0) == 0);
    assert(liblrhsmm_rs_mixture_occupancies_copy_values(mixture, 0, 0, 0, NULL, 0) == 0);
    assert(liblrhsmm_rs_duration_occupancies_copy_values(duration, 0, 2, 5, NULL, 0) == 2);
    input[2].present = 2;
    assert(liblrhsmm_rs_duration_occupancies_replace(duration, 1, 3, input, 3) == 3);
    assert(liblrhsmm_rs_mixture_occupancies_replace(mixture, 1, 3, input, 3) == 3);
    input[2].present = 1; weights[3] = 2;
    assert(liblrhsmm_rs_duration_occupancies_replace(duration, 3, 1, input, 3) == 0);
    assert(liblrhsmm_rs_mixture_occupancies_replace(mixture, 3, 1, input, 3) == 0);
    assert(liblrhsmm_rs_duration_occupancies_release(&duration) == 0 && duration == NULL);
    assert(liblrhsmm_rs_mixture_occupancies_release(&mixture) == 0 && mixture == NULL);
    assert(liblrhsmm_rs_duration_occupancies_copy_values(duration_clone, 0, 2, 0, copied, 4) == 0);
    assert(memcmp(copied, expected, sizeof(copied)) == 0);
    assert(liblrhsmm_rs_mixture_occupancies_copy_values(mixture_clone, 0, 2, 0, copied, 4) == 0);
    assert(memcmp(copied, expected, sizeof(copied)) == 0);
    assert(liblrhsmm_rs_duration_occupancies_release(&duration_clone) == 0);
    assert(liblrhsmm_rs_mixture_occupancies_release(&mixture_clone) == 0);
    _Alignas(8) LiblrhsmmRsTransitionPosterior arrivals[3] = {{1, -0.0f, 0.0f}, {0, NAN, INFINITY}, {1, 2.0f, 3.0f}};
    memcpy(&arrivals[0].advance, &smallest, sizeof(smallest));
    LiblrhsmmRsTransitionOccupancies *transition = NULL, *transition_clone = NULL;
    _Alignas(8) LiblrhsmmRsTransitionPosterior arrival;
    assert(liblrhsmm_rs_transition_occupancies_create(1, 3, arrivals, 3, &transition) == 0);
    assert(liblrhsmm_rs_transition_occupancies_clone(transition, &transition_clone) == 0);
    assert(liblrhsmm_rs_transition_occupancies_layout(transition, &layout) == 0);
    assert(layout.frames == 1 && layout.states == 3 && layout.values == 3);
    arrivals[2].advance = -1;
    assert(liblrhsmm_rs_transition_occupancies_replace(transition, 3, 1, arrivals, 3) == 3);
    assert(liblrhsmm_rs_transition_occupancies_get(transition, 0, 2, &arrival) == 0);
    assert(arrival.present == 1 && arrival.stay == 2 && arrival.advance == 3);
    arrivals[2].advance = 3;
    assert(liblrhsmm_rs_transition_occupancies_replace(transition, 3, 1, arrivals, 3) == 0);
    assert(liblrhsmm_rs_transition_occupancies_release(&transition) == 0);
    assert(liblrhsmm_rs_transition_occupancies_get(transition_clone, 0, 0, &arrival) == 0);
    assert(arrival.present == 1 && signbit(arrival.stay));
    assert(memcmp(&arrival.advance, &smallest, sizeof(smallest)) == 0);
    assert(liblrhsmm_rs_transition_occupancies_get(transition_clone, 0, 1, &arrival) == 0);
    assert(arrival.present == 0 && arrival.stay == 0 && arrival.advance == 0);
    assert(liblrhsmm_rs_transition_occupancies_release(&transition_clone) == 0);
}

static void state_storage_f32(void) {
    float weights[4] = {0, -0.0, 0, 2}, expected[4], copied[4], value;
    uint32_t smallest = 1; memcpy(&weights[2], &smallest, sizeof(smallest));
    memcpy(expected, weights, sizeof(weights));
    LiblrhsmmRsStateOccupanciesF32 *owner = NULL, *clone = NULL;
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout;
    assert(liblrhsmm_rs_state_occupancies_create_f32(2, 2, weights, 4, &owner) == 0);
    assert(liblrhsmm_rs_state_occupancies_clone_f32(owner, &clone) == 0);
    weights[3] = NAN;
    assert(liblrhsmm_rs_state_occupancies_replace_f32(owner, 1, 4, weights, 4) == 3);
    assert(liblrhsmm_rs_state_occupancies_layout_f32(owner, &layout) == 0);
    assert(layout.frames == 2 && layout.states == 2 && layout.values == 4);
    assert(liblrhsmm_rs_state_occupancies_copy_values_f32(owner, 0, copied, 4) == 0);
    assert(memcmp(copied, expected, sizeof(copied)) == 0);
    assert(liblrhsmm_rs_state_occupancies_copy_frame_f32(owner, 0, copied, 2) == 0);
    assert(memcmp(copied, expected, 2 * sizeof(float)) == 0);
    assert(liblrhsmm_rs_state_occupancies_get_f32(owner, 1, 0, &value) == 0);
    assert(memcmp(&value, &smallest, sizeof(smallest)) == 0);
    weights[3] = 2;
    assert(liblrhsmm_rs_state_occupancies_replace_f32(owner, 1, 4, weights, 4) == 0);
    assert(liblrhsmm_rs_state_occupancies_copy_values_f32(owner, 4, NULL, 0) == 0);
    assert(liblrhsmm_rs_state_occupancies_copy_values_f32(owner, 5, NULL, 0) == 2);
    assert(liblrhsmm_rs_state_occupancies_release_f32(&owner) == 0 && owner == NULL);
    assert(liblrhsmm_rs_state_occupancies_copy_values_f32(clone, 0, copied, 4) == 0);
    assert(memcmp(copied, expected, sizeof(copied)) == 0);
    assert(liblrhsmm_rs_state_occupancies_release_f32(&clone) == 0);
    assert(liblrhsmm_rs_state_occupancies_release_f32(&clone) == 0);
}
static void exercise_f32(void) {
    LiblrhsmmRsStreamF32 *streams[2]={0}; const LiblrhsmmRsStreamF32 *stream_inputs[2];
    LiblrhsmmRsObservationStreamF32 *features[2]={0}; const LiblrhsmmRsObservationStreamF32 *feature_inputs[2];
    for(size_t stream=0;stream<2;++stream) {
        size_t components=stream==0?2:1, dimensions=stream==0?2:1, count=components*dimensions;
        LiblrhsmmRsGaussianF32 *emissions[3]={0}; const LiblrhsmmRsGaussianF32 *emission_inputs[3];
        for(size_t state=0;state<3;++state) {
            float weights[2]={0.5,0.5}, means[4], variances[4], floors[4]={0};
            if(stream) weights[0]=1;
            for(size_t index=0;index<count;++index) { means[index]=state*0.5+index*0.25; variances[index]=0.75+index*0.25; }
            _Alignas(8) LiblrhsmmRsGaussianParametersF32 parameters={components,dimensions,weights,means,variances,floors};
            assert(liblrhsmm_rs_gaussian_create_f32(&parameters,&emissions[state])==0); emission_inputs[state]=emissions[state];
        }
        assert(liblrhsmm_rs_stream_create_f32(stream==0?0.5:1.25,emission_inputs,3,&streams[stream])==0); stream_inputs[stream]=streams[stream];
        for(size_t state=0;state<3;++state) assert(liblrhsmm_rs_gaussian_release_f32(&emissions[state])==0);
        float samples[24];
        for(size_t frame=0;frame<12;++frame) for(size_t dimension=0;dimension<dimensions;++dimension) samples[frame*dimensions+dimension]=frame*0.125+dimension*0.5;
        assert(liblrhsmm_rs_observation_stream_create_f32(dimensions,samples,12*dimensions,&features[stream])==0); feature_inputs[stream]=features[stream];
    }
    LiblrhsmmRsDurationF32 *durations[3]={0}; const LiblrhsmmRsDurationF32 *duration_inputs[3];
    for(size_t state=0;state<3;++state) {
        _Alignas(8) LiblrhsmmRsDurationParametersF32 parameters;
        assert(liblrhsmm_rs_duration_default_f32(&parameters)==0); parameters.mean=4;parameters.variance=1;
        assert(liblrhsmm_rs_duration_create_f32(&parameters,&durations[state])==0); duration_inputs[state]=durations[state];
    }
    LiblrhsmmRsModelF32 *model=NULL; LiblrhsmmRsObservationF32 *observation=NULL;
    assert(liblrhsmm_rs_model_create_f32(stream_inputs,2,duration_inputs,3,&model)==0);
    assert(liblrhsmm_rs_observation_create_f32(12,feature_inputs,2,&observation)==0);
    for(size_t state=0;state<3;++state) assert(liblrhsmm_rs_duration_release_f32(&durations[state])==0);
    for(size_t stream=0;stream<2;++stream) { assert(liblrhsmm_rs_stream_release_f32(&streams[stream])==0);assert(liblrhsmm_rs_observation_stream_release_f32(&features[stream])==0); }
    int32_t boundaries[3]={4,8,12},durations_labels[3]={0,1,2}, labels[2][3]={{0,1,2},{2,1,0}};
    const int32_t *output_states[2]={labels[0],labels[1]};
    _Alignas(8) LiblrhsmmRsJumpF32 normal={1,1}; _Alignas(8) LiblrhsmmRsJumpSliceF32 jumps[3]={{&normal,1},{&normal,1},{&normal,1}};
    LiblrhsmmRsSegmentationF32 *segmentation=NULL;
    assert(liblrhsmm_rs_segmentation_create_f32(2,3,boundaries,durations_labels,output_states,jumps,&segmentation)==0);

    _Alignas(8) LiblrhsmmRsEmissionSettingsF32 emission_options;
    assert(liblrhsmm_rs_emission_settings_default_f32(&emission_options) == 0);
    emission_options.temperature = 0.75;
    LiblrhsmmRsLogEmissionsF32 *log = NULL;
    assert(liblrhsmm_rs_model_emission_log_probabilities_f32(model, observation, segmentation, &emission_options, &log) == 0);
    _Alignas(8) LiblrhsmmRsHsmmSettingsF32 hsmm;
    assert(liblrhsmm_rs_hsmm_settings_default_f32(&hsmm) == 0);
    hsmm.temperature = 0.75;
    LiblrhsmmRsHsmmScoresF32 *forward = NULL, *backward = NULL;
    LiblrhsmmRsStateScoresF32 *forward_end = NULL, *backward_end = NULL;
    assert(liblrhsmm_rs_model_forward_hsmm_f32(model, segmentation, log, &hsmm, &forward) == 0);
    assert(liblrhsmm_rs_model_backward_hsmm_f32(model, segmentation, log, &hsmm, &backward) == 0);
    assert(liblrhsmm_rs_hsmm_scores_get_end_f32(forward, &forward_end) == 0);
    assert(liblrhsmm_rs_hsmm_scores_get_end_f32(backward, &backward_end) == 0);
    float total;
    assert(liblrhsmm_rs_state_scores_total_through_state_f32(forward_end, backward_end, 0, &total) == 0);
    assert(isfinite(total));
    LiblrhsmmRsTransitionGraphF32 *graph = NULL;
    assert(liblrhsmm_rs_segmentation_build_transitions_f32(segmentation, &graph) == 0);
    LiblrhsmmRsDurationOccupancies *duration = NULL, *cached = NULL;
    assert(liblrhsmm_rs_model_duration_occupancies_f32(model, segmentation, log, forward, backward, total, &hsmm, &duration) == 0);
    assert(liblrhsmm_rs_model_duration_occupancies_with_graph_f32(model, graph, log, forward, backward, total, &hsmm, &cached) == 0);
    size_t positive_durations = 0;
    for(size_t frame = 0; frame < 12; ++frame) for(size_t state = 0; state < 3; ++state) {
        _Alignas(8) LiblrhsmmRsPosteriorSlice a, b;
        assert(liblrhsmm_rs_duration_occupancies_get_slice(duration, frame, state, &a) == 0);
        assert(liblrhsmm_rs_duration_occupancies_get_slice(cached, frame, state, &b) == 0);
        assert(a.present == b.present && a.minimum == b.minimum && a.values == b.values);
        float *x = malloc(a.values * sizeof(float)), *y = malloc(a.values * sizeof(float));
        assert(a.values == 0 || (x != NULL && y != NULL));
        assert(liblrhsmm_rs_duration_occupancies_copy_values(duration, frame, state, 0, x, a.values) == 0);
        assert(liblrhsmm_rs_duration_occupancies_copy_values(cached, frame, state, 0, y, b.values) == 0);
        if(a.values) assert(memcmp(x, y, a.values * sizeof(float)) == 0);
        for(size_t index = 0; index < a.values; ++index) { assert(isfinite(x[index]) && x[index] >= 0); positive_durations += x[index] > 0; }
        free(x); free(y);
    }
    assert(positive_durations > 0);
    LiblrhsmmRsStateOccupanciesF32 *hsmm_state = NULL, *geometric_state = NULL;
    assert(liblrhsmm_rs_model_hsmm_state_occupancies_f32(model, segmentation, log, duration, &hsmm, &hsmm_state) == 0);
    _Alignas(8) LiblrhsmmRsGeometricSettingsF32 geometric;
    assert(liblrhsmm_rs_geometric_settings_default_f32(&geometric) == 0);
    geometric.temperature = 0.75; geometric.pruning_slope = 10;
    LiblrhsmmRsStateScoresF32 *gf = NULL, *gb = NULL;
    assert(liblrhsmm_rs_model_forward_geometric_f32(model, segmentation, log, &geometric, &gf) == 0);
    assert(liblrhsmm_rs_model_backward_geometric_f32(model, segmentation, log, &geometric, &gb) == 0);
    assert(liblrhsmm_rs_state_scores_total_f32(gf, &total) == 0);
    assert(liblrhsmm_rs_state_scores_geometric_occupancies_f32(gf, gb, total, geometric.pruning_slope, &geometric_state) == 0);
    LiblrhsmmRsTransitionOccupancies *transition = NULL;
    assert(liblrhsmm_rs_model_geometric_transition_occupancies_f32(model, segmentation, log, gf, gb, total, &geometric, &transition) == 0);
    size_t positive_mixtures = 0;
    for(size_t stream = 0; stream < 2; ++stream) {
        LiblrhsmmRsMixtureOccupancies *mixtures[2] = {NULL, NULL};
        assert(liblrhsmm_rs_model_mixture_occupancies_hsmm_f32(model, observation, segmentation, hsmm_state, stream, 0.75, 0.5, &mixtures[0]) == 0);
        assert(liblrhsmm_rs_model_mixture_occupancies_geometric_f32(model, observation, segmentation, geometric_state, stream, 0.75, 0.34, &mixtures[1]) == 0);
        for(size_t mode = 0; mode < 2; ++mode) {
            for(size_t frame = 0; frame < 12; ++frame) for(size_t state = 0; state < 3; ++state) {
                _Alignas(8) LiblrhsmmRsPosteriorSlice slice;
                assert(liblrhsmm_rs_mixture_occupancies_get_slice(mixtures[mode], frame, state, &slice) == 0);
                float values[2]; assert(slice.values <= 2);
                assert(liblrhsmm_rs_mixture_occupancies_copy_values(mixtures[mode], frame, state, 0, values, slice.values) == 0);
                for(size_t index = 0; index < slice.values; ++index) { assert(isfinite(values[index]) && values[index] >= 0); positive_mixtures += values[index] > 0; }
            }
            assert(liblrhsmm_rs_mixture_occupancies_release(&mixtures[mode]) == 0);
        }
    }
    assert(positive_mixtures > 0);
    LiblrhsmmRsDurationOccupancies *sentinel = (LiblrhsmmRsDurationOccupancies *)(uintptr_t)0x1000;
    assert(liblrhsmm_rs_model_duration_occupancies_f32(model, segmentation, log, forward, backward, NAN, &hsmm, &sentinel) == 3);
    assert((uintptr_t)sentinel == 0x1000);
    assert(liblrhsmm_rs_model_release_f32(&model) == 0);
    assert(liblrhsmm_rs_observation_release_f32(&observation) == 0);
    assert(liblrhsmm_rs_segmentation_release_f32(&segmentation) == 0);
    float retained[36];
    assert(liblrhsmm_rs_state_occupancies_copy_values_f32(hsmm_state, 0, retained, 36) == 0);
    for(size_t index = 0; index < 36; ++index) assert(isfinite(retained[index]) && retained[index] >= 0);
    assert(liblrhsmm_rs_state_occupancies_release_f32(&hsmm_state) == 0);
    assert(liblrhsmm_rs_state_occupancies_release_f32(&geometric_state) == 0);
    assert(liblrhsmm_rs_duration_occupancies_release(&duration) == 0);
    assert(liblrhsmm_rs_duration_occupancies_release(&cached) == 0);
    assert(liblrhsmm_rs_transition_occupancies_release(&transition) == 0);
    assert(liblrhsmm_rs_state_scores_release_f32(&gf) == 0);
    assert(liblrhsmm_rs_state_scores_release_f32(&gb) == 0);
    assert(liblrhsmm_rs_state_scores_release_f32(&forward_end) == 0);
    assert(liblrhsmm_rs_state_scores_release_f32(&backward_end) == 0);
    assert(liblrhsmm_rs_hsmm_scores_release_f32(&forward) == 0);
    assert(liblrhsmm_rs_hsmm_scores_release_f32(&backward) == 0);
    assert(liblrhsmm_rs_transition_graph_release_f32(&graph) == 0);
    assert(liblrhsmm_rs_log_emissions_release_f32(&log) == 0);
}

static void state_storage_f64(void) {
    _Alignas(8) double weights[4] = {0, -0.0, 0, 2}, expected[4], copied[4], value;
    uint64_t smallest = 1; memcpy(&weights[2], &smallest, sizeof(smallest));
    memcpy(expected, weights, sizeof(weights));
    LiblrhsmmRsStateOccupanciesF64 *owner = NULL, *clone = NULL;
    _Alignas(8) LiblrhsmmRsInferenceTableLayout layout;
    assert(liblrhsmm_rs_state_occupancies_create_f64(2, 2, weights, 4, &owner) == 0);
    assert(liblrhsmm_rs_state_occupancies_clone_f64(owner, &clone) == 0);
    weights[3] = NAN;
    assert(liblrhsmm_rs_state_occupancies_replace_f64(owner, 1, 4, weights, 4) == 3);
    assert(liblrhsmm_rs_state_occupancies_layout_f64(owner, &layout) == 0);
    assert(layout.frames == 2 && layout.states == 2 && layout.values == 4);
    assert(liblrhsmm_rs_state_occupancies_copy_values_f64(owner, 0, copied, 4) == 0);
    assert(memcmp(copied, expected, sizeof(copied)) == 0);
    assert(liblrhsmm_rs_state_occupancies_copy_frame_f64(owner, 0, copied, 2) == 0);
    assert(memcmp(copied, expected, 2 * sizeof(double)) == 0);
    assert(liblrhsmm_rs_state_occupancies_get_f64(owner, 1, 0, &value) == 0);
    assert(memcmp(&value, &smallest, sizeof(smallest)) == 0);
    weights[3] = 2;
    assert(liblrhsmm_rs_state_occupancies_replace_f64(owner, 1, 4, weights, 4) == 0);
    assert(liblrhsmm_rs_state_occupancies_copy_values_f64(owner, 4, NULL, 0) == 0);
    assert(liblrhsmm_rs_state_occupancies_copy_values_f64(owner, 5, NULL, 0) == 2);
    assert(liblrhsmm_rs_state_occupancies_release_f64(&owner) == 0 && owner == NULL);
    assert(liblrhsmm_rs_state_occupancies_copy_values_f64(clone, 0, copied, 4) == 0);
    assert(memcmp(copied, expected, sizeof(copied)) == 0);
    assert(liblrhsmm_rs_state_occupancies_release_f64(&clone) == 0);
    assert(liblrhsmm_rs_state_occupancies_release_f64(&clone) == 0);
}
static void exercise_f64(void) {
    LiblrhsmmRsStreamF64 *streams[2]={0}; const LiblrhsmmRsStreamF64 *stream_inputs[2];
    LiblrhsmmRsObservationStreamF64 *features[2]={0}; const LiblrhsmmRsObservationStreamF64 *feature_inputs[2];
    for(size_t stream=0;stream<2;++stream) {
        size_t components=stream==0?2:1, dimensions=stream==0?2:1, count=components*dimensions;
        LiblrhsmmRsGaussianF64 *emissions[3]={0}; const LiblrhsmmRsGaussianF64 *emission_inputs[3];
        for(size_t state=0;state<3;++state) {
            _Alignas(8) double weights[2]={0.5,0.5}, means[4], variances[4], floors[4]={0};
            if(stream) weights[0]=1;
            for(size_t index=0;index<count;++index) { means[index]=state*0.5+index*0.25; variances[index]=0.75+index*0.25; }
            _Alignas(8) LiblrhsmmRsGaussianParametersF64 parameters={components,dimensions,weights,means,variances,floors};
            assert(liblrhsmm_rs_gaussian_create_f64(&parameters,&emissions[state])==0); emission_inputs[state]=emissions[state];
        }
        assert(liblrhsmm_rs_stream_create_f64(stream==0?0.5:1.25,emission_inputs,3,&streams[stream])==0); stream_inputs[stream]=streams[stream];
        for(size_t state=0;state<3;++state) assert(liblrhsmm_rs_gaussian_release_f64(&emissions[state])==0);
        _Alignas(8) double samples[24];
        for(size_t frame=0;frame<12;++frame) for(size_t dimension=0;dimension<dimensions;++dimension) samples[frame*dimensions+dimension]=frame*0.125+dimension*0.5;
        assert(liblrhsmm_rs_observation_stream_create_f64(dimensions,samples,12*dimensions,&features[stream])==0); feature_inputs[stream]=features[stream];
    }
    LiblrhsmmRsDurationF64 *durations[3]={0}; const LiblrhsmmRsDurationF64 *duration_inputs[3];
    for(size_t state=0;state<3;++state) {
        _Alignas(8) LiblrhsmmRsDurationParametersF64 parameters;
        assert(liblrhsmm_rs_duration_default_f64(&parameters)==0); parameters.mean=4;parameters.variance=1;
        assert(liblrhsmm_rs_duration_create_f64(&parameters,&durations[state])==0); duration_inputs[state]=durations[state];
    }
    LiblrhsmmRsModelF64 *model=NULL; LiblrhsmmRsObservationF64 *observation=NULL;
    assert(liblrhsmm_rs_model_create_f64(stream_inputs,2,duration_inputs,3,&model)==0);
    assert(liblrhsmm_rs_observation_create_f64(12,feature_inputs,2,&observation)==0);
    for(size_t state=0;state<3;++state) assert(liblrhsmm_rs_duration_release_f64(&durations[state])==0);
    for(size_t stream=0;stream<2;++stream) { assert(liblrhsmm_rs_stream_release_f64(&streams[stream])==0);assert(liblrhsmm_rs_observation_stream_release_f64(&features[stream])==0); }
    int32_t boundaries[3]={4,8,12},durations_labels[3]={0,1,2}, labels[2][3]={{0,1,2},{2,1,0}};
    const int32_t *output_states[2]={labels[0],labels[1]};
    _Alignas(8) LiblrhsmmRsJumpF64 normal={1,1}; _Alignas(8) LiblrhsmmRsJumpSliceF64 jumps[3]={{&normal,1},{&normal,1},{&normal,1}};
    LiblrhsmmRsSegmentationF64 *segmentation=NULL;
    assert(liblrhsmm_rs_segmentation_create_f64(2,3,boundaries,durations_labels,output_states,jumps,&segmentation)==0);

    _Alignas(8) LiblrhsmmRsEmissionSettingsF64 emission_options;
    assert(liblrhsmm_rs_emission_settings_default_f64(&emission_options) == 0);
    emission_options.temperature = 0.75;
    LiblrhsmmRsLogEmissionsF64 *log = NULL;
    assert(liblrhsmm_rs_model_emission_log_probabilities_f64(model, observation, segmentation, &emission_options, &log) == 0);
    _Alignas(8) LiblrhsmmRsHsmmSettingsF64 hsmm;
    assert(liblrhsmm_rs_hsmm_settings_default_f64(&hsmm) == 0);
    hsmm.temperature = 0.75;
    LiblrhsmmRsHsmmScoresF64 *forward = NULL, *backward = NULL;
    LiblrhsmmRsStateScoresF64 *forward_end = NULL, *backward_end = NULL;
    assert(liblrhsmm_rs_model_forward_hsmm_f64(model, segmentation, log, &hsmm, &forward) == 0);
    assert(liblrhsmm_rs_model_backward_hsmm_f64(model, segmentation, log, &hsmm, &backward) == 0);
    assert(liblrhsmm_rs_hsmm_scores_get_end_f64(forward, &forward_end) == 0);
    assert(liblrhsmm_rs_hsmm_scores_get_end_f64(backward, &backward_end) == 0);
    _Alignas(8) double total;
    assert(liblrhsmm_rs_state_scores_total_through_state_f64(forward_end, backward_end, 0, &total) == 0);
    assert(isfinite(total));
    LiblrhsmmRsTransitionGraphF64 *graph = NULL;
    assert(liblrhsmm_rs_segmentation_build_transitions_f64(segmentation, &graph) == 0);
    LiblrhsmmRsDurationOccupancies *duration = NULL, *cached = NULL;
    assert(liblrhsmm_rs_model_duration_occupancies_f64(model, segmentation, log, forward, backward, total, &hsmm, &duration) == 0);
    assert(liblrhsmm_rs_model_duration_occupancies_with_graph_f64(model, graph, log, forward, backward, total, &hsmm, &cached) == 0);
    size_t positive_durations = 0;
    for(size_t frame = 0; frame < 12; ++frame) for(size_t state = 0; state < 3; ++state) {
        _Alignas(8) LiblrhsmmRsPosteriorSlice a, b;
        assert(liblrhsmm_rs_duration_occupancies_get_slice(duration, frame, state, &a) == 0);
        assert(liblrhsmm_rs_duration_occupancies_get_slice(cached, frame, state, &b) == 0);
        assert(a.present == b.present && a.minimum == b.minimum && a.values == b.values);
        float *x = malloc(a.values * sizeof(float)), *y = malloc(a.values * sizeof(float));
        assert(a.values == 0 || (x != NULL && y != NULL));
        assert(liblrhsmm_rs_duration_occupancies_copy_values(duration, frame, state, 0, x, a.values) == 0);
        assert(liblrhsmm_rs_duration_occupancies_copy_values(cached, frame, state, 0, y, b.values) == 0);
        if(a.values) assert(memcmp(x, y, a.values * sizeof(float)) == 0);
        for(size_t index = 0; index < a.values; ++index) { assert(isfinite(x[index]) && x[index] >= 0); positive_durations += x[index] > 0; }
        free(x); free(y);
    }
    assert(positive_durations > 0);
    LiblrhsmmRsStateOccupanciesF64 *hsmm_state = NULL, *geometric_state = NULL;
    assert(liblrhsmm_rs_model_hsmm_state_occupancies_f64(model, segmentation, log, duration, &hsmm, &hsmm_state) == 0);
    _Alignas(8) LiblrhsmmRsGeometricSettingsF64 geometric;
    assert(liblrhsmm_rs_geometric_settings_default_f64(&geometric) == 0);
    geometric.temperature = 0.75; geometric.pruning_slope = 10;
    LiblrhsmmRsStateScoresF64 *gf = NULL, *gb = NULL;
    assert(liblrhsmm_rs_model_forward_geometric_f64(model, segmentation, log, &geometric, &gf) == 0);
    assert(liblrhsmm_rs_model_backward_geometric_f64(model, segmentation, log, &geometric, &gb) == 0);
    assert(liblrhsmm_rs_state_scores_total_f64(gf, &total) == 0);
    assert(liblrhsmm_rs_state_scores_geometric_occupancies_f64(gf, gb, total, geometric.pruning_slope, &geometric_state) == 0);
    LiblrhsmmRsTransitionOccupancies *transition = NULL;
    assert(liblrhsmm_rs_model_geometric_transition_occupancies_f64(model, segmentation, log, gf, gb, total, &geometric, &transition) == 0);
    size_t positive_mixtures = 0;
    for(size_t stream = 0; stream < 2; ++stream) {
        LiblrhsmmRsMixtureOccupancies *mixtures[2] = {NULL, NULL};
        assert(liblrhsmm_rs_model_mixture_occupancies_hsmm_f64(model, observation, segmentation, hsmm_state, stream, 0.75, 0.5, &mixtures[0]) == 0);
        assert(liblrhsmm_rs_model_mixture_occupancies_geometric_f64(model, observation, segmentation, geometric_state, stream, 0.75, 0.34, &mixtures[1]) == 0);
        for(size_t mode = 0; mode < 2; ++mode) {
            for(size_t frame = 0; frame < 12; ++frame) for(size_t state = 0; state < 3; ++state) {
                _Alignas(8) LiblrhsmmRsPosteriorSlice slice;
                assert(liblrhsmm_rs_mixture_occupancies_get_slice(mixtures[mode], frame, state, &slice) == 0);
                float values[2]; assert(slice.values <= 2);
                assert(liblrhsmm_rs_mixture_occupancies_copy_values(mixtures[mode], frame, state, 0, values, slice.values) == 0);
                for(size_t index = 0; index < slice.values; ++index) { assert(isfinite(values[index]) && values[index] >= 0); positive_mixtures += values[index] > 0; }
            }
            assert(liblrhsmm_rs_mixture_occupancies_release(&mixtures[mode]) == 0);
        }
    }
    assert(positive_mixtures > 0);
    LiblrhsmmRsDurationOccupancies *sentinel = (LiblrhsmmRsDurationOccupancies *)(uintptr_t)0x1000;
    assert(liblrhsmm_rs_model_duration_occupancies_f64(model, segmentation, log, forward, backward, NAN, &hsmm, &sentinel) == 3);
    assert((uintptr_t)sentinel == 0x1000);
    assert(liblrhsmm_rs_model_release_f64(&model) == 0);
    assert(liblrhsmm_rs_observation_release_f64(&observation) == 0);
    assert(liblrhsmm_rs_segmentation_release_f64(&segmentation) == 0);
    _Alignas(8) double retained[36];
    assert(liblrhsmm_rs_state_occupancies_copy_values_f64(hsmm_state, 0, retained, 36) == 0);
    for(size_t index = 0; index < 36; ++index) assert(isfinite(retained[index]) && retained[index] >= 0);
    assert(liblrhsmm_rs_state_occupancies_release_f64(&hsmm_state) == 0);
    assert(liblrhsmm_rs_state_occupancies_release_f64(&geometric_state) == 0);
    assert(liblrhsmm_rs_duration_occupancies_release(&duration) == 0);
    assert(liblrhsmm_rs_duration_occupancies_release(&cached) == 0);
    assert(liblrhsmm_rs_transition_occupancies_release(&transition) == 0);
    assert(liblrhsmm_rs_state_scores_release_f64(&gf) == 0);
    assert(liblrhsmm_rs_state_scores_release_f64(&gb) == 0);
    assert(liblrhsmm_rs_state_scores_release_f64(&forward_end) == 0);
    assert(liblrhsmm_rs_state_scores_release_f64(&backward_end) == 0);
    assert(liblrhsmm_rs_hsmm_scores_release_f64(&forward) == 0);
    assert(liblrhsmm_rs_hsmm_scores_release_f64(&backward) == 0);
    assert(liblrhsmm_rs_transition_graph_release_f64(&graph) == 0);
    assert(liblrhsmm_rs_log_emissions_release_f64(&log) == 0);
}

int main(void) { sparse_storage(); state_storage_f32(); state_storage_f64(); exercise_f32(); exercise_f64(); puts("All 51 posterior C exports passed"); return 0; }
