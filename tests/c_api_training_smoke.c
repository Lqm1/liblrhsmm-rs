#include "liblrhsmm_rs.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

static size_t snapshot_f32(LiblrhsmmRsModelStatisticsF32 *owner, float values[128]) {
    _Alignas(8) LiblrhsmmRsModelLayout model;
    assert(liblrhsmm_rs_model_statistics_layout_f32(owner, &model) == 0);
    assert(model.streams == 2 && model.durations == 3);
    size_t used = 0;
    for(size_t stream = 0; stream < 2; ++stream) {
        LiblrhsmmRsStreamStatisticsF32 *s = NULL;
        assert(liblrhsmm_rs_model_statistics_get_stream_f32(owner, stream, &s) == 0);
        size_t count; assert(liblrhsmm_rs_stream_statistics_count_f32(s, &count) == 0); assert(count == 3);
        for(size_t state = 0; state < count; ++state) {
            LiblrhsmmRsGaussianStatisticsF32 *g = NULL;
            _Alignas(8) LiblrhsmmRsGaussianStatisticsLayoutF32 shape;
            assert(liblrhsmm_rs_stream_statistics_get_emission_f32(s, state, &g) == 0);
            assert(liblrhsmm_rs_gaussian_statistics_layout_f32(g, &shape) == 0);
            values[used++] = shape.mixtures; values[used++] = shape.dimensions; values[used++] = shape.state_occupancy;
            for(size_t field = 0; field < 3; ++field) {
                size_t length = field == 2 ? shape.mixtures : shape.mixtures * shape.dimensions;
                assert(used + length <= 128);
                assert(liblrhsmm_rs_gaussian_statistics_copy_field_f32(g, field, 0, values + used, length) == 0);
                used += length;
            }
            assert(liblrhsmm_rs_gaussian_statistics_release_f32(&g) == 0);
        }
        assert(liblrhsmm_rs_stream_statistics_release_f32(&s) == 0);
    }
    for(size_t index = 0; index < 3; ++index) {
        LiblrhsmmRsDurationStatisticsF32 *d = NULL; _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF32 v;
        assert(liblrhsmm_rs_model_statistics_get_duration_f32(owner, index, &d) == 0);
        assert(liblrhsmm_rs_duration_statistics_get_f32(d, &v) == 0);
        values[used++] = v.value_sum; values[used++] = v.squared_sum; values[used++] = v.occupancy;
        assert(liblrhsmm_rs_duration_statistics_release_f32(&d) == 0);
    }
    return used;
}
static float infer_f32(LiblrhsmmRsModelF32 *model, LiblrhsmmRsObservationF32 *obs, LiblrhsmmRsSegmentationF32 *seg) {
    _Alignas(8) LiblrhsmmRsEmissionSettingsF32 settings; assert(liblrhsmm_rs_emission_settings_default_f32(&settings) == 0);
    LiblrhsmmRsLogEmissionsF32 *log = NULL;
    assert(liblrhsmm_rs_model_emission_log_probabilities_f32(model, obs, seg, &settings, &log) == 0);
    _Alignas(8) LiblrhsmmRsHsmmSettingsF32 hsmm; assert(liblrhsmm_rs_hsmm_settings_default_f32(&hsmm) == 0);
    LiblrhsmmRsHsmmPathF32 *path = NULL;
    assert(liblrhsmm_rs_model_viterbi_hsmm_f32(model, seg, log, &hsmm, &path) == 0);
    float value; assert(liblrhsmm_rs_hsmm_path_log_probability_f32(path, &value) == 0); assert(isfinite(value));
    assert(liblrhsmm_rs_hsmm_path_release_f32(&path) == 0);
    assert(liblrhsmm_rs_log_emissions_release_f32(&log) == 0);
    return value;
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


    _Alignas(8) LiblrhsmmRsStreamSpecification specs[2] = {{3, 1, 2}, {3, 1, 1}};
    LiblrhsmmRsModelF32 *initial = NULL;
    LiblrhsmmRsModelStatisticsF32 *initial_stats = NULL;
    assert(liblrhsmm_rs_model_new_f32(specs, 2, 3, &initial) == 0);
    assert(liblrhsmm_rs_model_statistics_from_model_f32(initial, &initial_stats) == 0);
    for(size_t repeat = 0; repeat < 2; ++repeat)
        assert(liblrhsmm_rs_model_statistics_collect_initialization_f32(initial_stats, initial, observation, segmentation) == 0);
    float initialized[128]; size_t initialized_count = snapshot_f32(initial_stats, initialized);
    for(size_t index = initialized_count - 9; index < initialized_count; index += 3)
        assert(initialized[index] == 8 && initialized[index + 1] == 32 && initialized[index + 2] == 2);
    _Alignas(8) LiblrhsmmRsUpdateOptionsF32 update; assert(liblrhsmm_rs_update_options_default_f32(&update) == 0);
    assert(liblrhsmm_rs_model_update_f32(initial, initial_stats, &update) == 0);
    assert(isfinite(infer_f32(initial, observation, segmentation)));
    assert(liblrhsmm_rs_model_statistics_release_f32(&initial_stats) == 0);
    assert(liblrhsmm_rs_model_release_f32(&initial) == 0);
    for(size_t mode = 0; mode < 2; ++mode) {
        LiblrhsmmRsModelStatisticsF32 *manual = NULL, *automatic = NULL;
        assert(liblrhsmm_rs_model_statistics_from_model_f32(model, &manual) == 0);
        assert(liblrhsmm_rs_model_statistics_from_model_f32(model, &automatic) == 0);
        _Alignas(8) LiblrhsmmRsHsmmSettingsF32 hsmm; assert(liblrhsmm_rs_hsmm_settings_default_f32(&hsmm) == 0);
        hsmm.temperature = 0.75; hsmm.state_radius = 1;
        _Alignas(8) LiblrhsmmRsGeometricSettingsF32 geo = {0.75, 0.5};
        _Alignas(8) LiblrhsmmRsEmissionSettingsF32 emission = {0.75, mode ? LIBLRHSMM_RS_EMISSION_PRUNING_LINEAR : LIBLRHSMM_RS_EMISSION_PRUNING_BOUNDARIES, 1, 0.5};
        LiblrhsmmRsLogEmissionsF32 *log = NULL;
        assert(liblrhsmm_rs_model_emission_log_probabilities_f32(model, observation, segmentation, &emission, &log) == 0);
        LiblrhsmmRsStateOccupanciesF32 *state = NULL;
        LiblrhsmmRsDurationOccupancies *durations = NULL;
        LiblrhsmmRsTransitionOccupancies *transitions = NULL;
        LiblrhsmmRsHsmmScoresF32 *hf = NULL, *hb = NULL;
        LiblrhsmmRsStateScoresF32 *gf = NULL, *gb = NULL;
        float total;
        if(mode == 0) {
            assert(liblrhsmm_rs_model_forward_hsmm_f32(model, segmentation, log, &hsmm, &hf) == 0);
            assert(liblrhsmm_rs_model_backward_hsmm_f32(model, segmentation, log, &hsmm, &hb) == 0);
            assert(liblrhsmm_rs_hsmm_scores_get_end_f32(hf, &gf) == 0);
            assert(liblrhsmm_rs_hsmm_scores_get_end_f32(hb, &gb) == 0);
            assert(liblrhsmm_rs_state_scores_total_through_state_f32(gf, gb, 0, &total) == 0);
            assert(liblrhsmm_rs_model_duration_occupancies_f32(model, segmentation, log, hf, hb, total, &hsmm, &durations) == 0);
            assert(liblrhsmm_rs_model_hsmm_state_occupancies_f32(model, segmentation, log, durations, &hsmm, &state) == 0);
        } else {
            assert(liblrhsmm_rs_model_forward_geometric_f32(model, segmentation, log, &geo, &gf) == 0);
            assert(liblrhsmm_rs_model_backward_geometric_f32(model, segmentation, log, &geo, &gb) == 0);
            assert(liblrhsmm_rs_state_scores_backward_total_f32(gb, log, &total) == 0);
            assert(liblrhsmm_rs_state_scores_geometric_occupancies_f32(gf, gb, total, geo.pruning_slope, &state) == 0);
            assert(liblrhsmm_rs_model_geometric_transition_occupancies_f32(model, segmentation, log, gf, gb, total, &geo, &transitions) == 0);
        }
        LiblrhsmmRsMixtureOccupancies *mixtures[2] = {NULL, NULL};
        const LiblrhsmmRsMixtureOccupancies *borrowed[2];
        for(size_t stream = 0; stream < 2; ++stream) {
            if(mode == 0) assert(liblrhsmm_rs_model_mixture_occupancies_hsmm_f32(model, observation, segmentation, state, stream, hsmm.temperature, hsmm.state_radius, &mixtures[stream]) == 0);
            else assert(liblrhsmm_rs_model_mixture_occupancies_geometric_f32(model, observation, segmentation, state, stream, geo.temperature, geo.pruning_slope, &mixtures[stream]) == 0);
            borrowed[stream] = mixtures[stream];
        }
        for(size_t repeat = 0; repeat < 2; ++repeat) {
            float actual = 99;
            if(mode == 0) {
                assert(liblrhsmm_rs_model_statistics_collect_hsmm_f32(manual, model, observation, segmentation, durations, state, borrowed, 2, hsmm.state_radius) == 0);
                assert(liblrhsmm_rs_model_statistics_estimate_hsmm_f32(automatic, model, observation, segmentation, &hsmm, &actual) == 0);
            } else {
                assert(liblrhsmm_rs_model_statistics_collect_geometric_f32(manual, model, observation, segmentation, transitions, state, borrowed, 2, geo.pruning_slope) == 0);
                assert(liblrhsmm_rs_model_statistics_estimate_geometric_f32(automatic, model, observation, segmentation, &geo, &actual) == 0);
            }
            assert(memcmp(&actual, &total, sizeof(total)) == 0);
            float a[128], b[128]; size_t count = snapshot_f32(manual, a);
            assert(count == snapshot_f32(automatic, b) && memcmp(a, b, count * sizeof(float)) == 0);
        }
        float before[128], after[128]; size_t count = snapshot_f32(automatic, before);
        assert(liblrhsmm_rs_model_statistics_estimate_hsmm_f32(automatic, model, observation, segmentation, &hsmm, NULL) == 1);
        assert(liblrhsmm_rs_model_statistics_estimate_geometric_f32(automatic, model, observation, segmentation, &geo, NULL) == 1);
        float sentinel = 99;
        hsmm.temperature = NAN; geo.temperature = NAN;
        assert(liblrhsmm_rs_model_statistics_estimate_hsmm_f32(automatic, model, observation, segmentation, &hsmm, &sentinel) == 3);
        assert(liblrhsmm_rs_model_statistics_estimate_geometric_f32(automatic, model, observation, segmentation, &geo, &sentinel) == 3);
        assert(sentinel == 99 && count == snapshot_f32(automatic, after) && memcmp(before, after, count * sizeof(float)) == 0);
        size_t manual_count = snapshot_f32(manual, before);
        const LiblrhsmmRsMixtureOccupancies *bad[2] = {borrowed[0], NULL};
        if(mode == 0) {
            assert(liblrhsmm_rs_model_statistics_collect_hsmm_f32(manual, model, observation, segmentation, durations, state, bad, 2, 1) == 1);
            assert(liblrhsmm_rs_model_statistics_collect_hsmm_f32(manual, model, observation, segmentation, durations, state, borrowed, 1, 1) == 3);
        } else {
            assert(liblrhsmm_rs_model_statistics_collect_geometric_f32(manual, model, observation, segmentation, transitions, state, bad, 2, 0.5) == 1);
            assert(liblrhsmm_rs_model_statistics_collect_geometric_f32(manual, model, observation, segmentation, transitions, state, borrowed, 1, 0.5) == 3);
        }
        assert(manual_count == snapshot_f32(manual, after) && memcmp(before, after, manual_count * sizeof(float)) == 0);
        LiblrhsmmRsModelF32 *trained = NULL; assert(liblrhsmm_rs_model_clone_f32(model, &trained) == 0);
        assert(liblrhsmm_rs_update_options_default_f32(&update) == 0); update.duration_mode = mode;
        assert(liblrhsmm_rs_model_update_f32(trained, automatic, &update) == 0);
        float likelihood = infer_f32(trained, observation, segmentation);
#ifdef LIBLRHSMM_RS_SERIALIZATION
        LiblrhsmmRsBytes *encoded = NULL; size_t length;
        assert(liblrhsmm_rs_model_write_bytes_f32(trained, LIBLRHSMM_RS_ENCODING_WITH_VARIANCE_FLOORS, &encoded) == 0);
        assert(liblrhsmm_rs_bytes_length(encoded, &length) == 0);
        uint8_t *data = malloc(length); assert(data != NULL);
        assert(liblrhsmm_rs_bytes_copy(encoded, 0, data, length) == 0);
        _Alignas(8) LiblrhsmmRsDecodeLimits limits; assert(liblrhsmm_rs_decode_limits_default(&limits) == 0);
        LiblrhsmmRsModelF32 *decoded = NULL;
        assert(liblrhsmm_rs_model_read_bytes_f32(data, length, &limits, &decoded) == 0);
        assert(liblrhsmm_rs_bytes_release(&encoded) == 0);
        assert(liblrhsmm_rs_model_release_f32(&trained) == 0); trained = decoded;
        float roundtrip = infer_f32(trained, observation, segmentation);
        assert(memcmp(&likelihood, &roundtrip, sizeof(likelihood)) == 0);
        LiblrhsmmRsBytes *reencoded = NULL; size_t second_length;
        assert(liblrhsmm_rs_model_write_bytes_f32(trained, LIBLRHSMM_RS_ENCODING_WITH_VARIANCE_FLOORS, &reencoded) == 0);
        assert(liblrhsmm_rs_bytes_length(reencoded, &second_length) == 0 && second_length == length);
        uint8_t *second_data = malloc(second_length); assert(second_data != NULL);
        assert(liblrhsmm_rs_bytes_copy(reencoded, 0, second_data, second_length) == 0);
        assert(memcmp(data, second_data, length) == 0);
        free(data); free(second_data); assert(liblrhsmm_rs_bytes_release(&reencoded) == 0);
#else
        assert(isfinite(likelihood));
#endif
        assert(liblrhsmm_rs_model_release_f32(&trained) == 0);
        for(size_t stream = 0; stream < 2; ++stream) assert(liblrhsmm_rs_mixture_occupancies_release(&mixtures[stream]) == 0);
        assert(liblrhsmm_rs_duration_occupancies_release(&durations) == 0);
        assert(liblrhsmm_rs_transition_occupancies_release(&transitions) == 0);
        assert(liblrhsmm_rs_state_occupancies_release_f32(&state) == 0);
        assert(liblrhsmm_rs_state_scores_release_f32(&gf) == 0);
        assert(liblrhsmm_rs_state_scores_release_f32(&gb) == 0);
        assert(liblrhsmm_rs_hsmm_scores_release_f32(&hf) == 0);
        assert(liblrhsmm_rs_hsmm_scores_release_f32(&hb) == 0);
        assert(liblrhsmm_rs_log_emissions_release_f32(&log) == 0);
        assert(liblrhsmm_rs_model_statistics_release_f32(&manual) == 0);
        assert(liblrhsmm_rs_model_statistics_release_f32(&automatic) == 0);
    }
    assert(liblrhsmm_rs_model_release_f32(&model) == 0);
    assert(liblrhsmm_rs_observation_release_f32(&observation) == 0);
    assert(liblrhsmm_rs_segmentation_release_f32(&segmentation) == 0);
}

static size_t snapshot_f64(LiblrhsmmRsModelStatisticsF64 *owner, double values[128]) {
    _Alignas(8) LiblrhsmmRsModelLayout model;
    assert(liblrhsmm_rs_model_statistics_layout_f64(owner, &model) == 0);
    assert(model.streams == 2 && model.durations == 3);
    size_t used = 0;
    for(size_t stream = 0; stream < 2; ++stream) {
        LiblrhsmmRsStreamStatisticsF64 *s = NULL;
        assert(liblrhsmm_rs_model_statistics_get_stream_f64(owner, stream, &s) == 0);
        size_t count; assert(liblrhsmm_rs_stream_statistics_count_f64(s, &count) == 0); assert(count == 3);
        for(size_t state = 0; state < count; ++state) {
            LiblrhsmmRsGaussianStatisticsF64 *g = NULL;
            _Alignas(8) LiblrhsmmRsGaussianStatisticsLayoutF64 shape;
            assert(liblrhsmm_rs_stream_statistics_get_emission_f64(s, state, &g) == 0);
            assert(liblrhsmm_rs_gaussian_statistics_layout_f64(g, &shape) == 0);
            values[used++] = shape.mixtures; values[used++] = shape.dimensions; values[used++] = shape.state_occupancy;
            for(size_t field = 0; field < 3; ++field) {
                size_t length = field == 2 ? shape.mixtures : shape.mixtures * shape.dimensions;
                assert(used + length <= 128);
                assert(liblrhsmm_rs_gaussian_statistics_copy_field_f64(g, field, 0, values + used, length) == 0);
                used += length;
            }
            assert(liblrhsmm_rs_gaussian_statistics_release_f64(&g) == 0);
        }
        assert(liblrhsmm_rs_stream_statistics_release_f64(&s) == 0);
    }
    for(size_t index = 0; index < 3; ++index) {
        LiblrhsmmRsDurationStatisticsF64 *d = NULL; _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF64 v;
        assert(liblrhsmm_rs_model_statistics_get_duration_f64(owner, index, &d) == 0);
        assert(liblrhsmm_rs_duration_statistics_get_f64(d, &v) == 0);
        values[used++] = v.value_sum; values[used++] = v.squared_sum; values[used++] = v.occupancy;
        assert(liblrhsmm_rs_duration_statistics_release_f64(&d) == 0);
    }
    return used;
}
static double infer_f64(LiblrhsmmRsModelF64 *model, LiblrhsmmRsObservationF64 *obs, LiblrhsmmRsSegmentationF64 *seg) {
    _Alignas(8) LiblrhsmmRsEmissionSettingsF64 settings; assert(liblrhsmm_rs_emission_settings_default_f64(&settings) == 0);
    LiblrhsmmRsLogEmissionsF64 *log = NULL;
    assert(liblrhsmm_rs_model_emission_log_probabilities_f64(model, obs, seg, &settings, &log) == 0);
    _Alignas(8) LiblrhsmmRsHsmmSettingsF64 hsmm; assert(liblrhsmm_rs_hsmm_settings_default_f64(&hsmm) == 0);
    LiblrhsmmRsHsmmPathF64 *path = NULL;
    assert(liblrhsmm_rs_model_viterbi_hsmm_f64(model, seg, log, &hsmm, &path) == 0);
    _Alignas(8) double value; assert(liblrhsmm_rs_hsmm_path_log_probability_f64(path, &value) == 0); assert(isfinite(value));
    assert(liblrhsmm_rs_hsmm_path_release_f64(&path) == 0);
    assert(liblrhsmm_rs_log_emissions_release_f64(&log) == 0);
    return value;
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


    _Alignas(8) LiblrhsmmRsStreamSpecification specs[2] = {{3, 1, 2}, {3, 1, 1}};
    LiblrhsmmRsModelF64 *initial = NULL;
    LiblrhsmmRsModelStatisticsF64 *initial_stats = NULL;
    assert(liblrhsmm_rs_model_new_f64(specs, 2, 3, &initial) == 0);
    assert(liblrhsmm_rs_model_statistics_from_model_f64(initial, &initial_stats) == 0);
    for(size_t repeat = 0; repeat < 2; ++repeat)
        assert(liblrhsmm_rs_model_statistics_collect_initialization_f64(initial_stats, initial, observation, segmentation) == 0);
    _Alignas(8) double initialized[128]; size_t initialized_count = snapshot_f64(initial_stats, initialized);
    for(size_t index = initialized_count - 9; index < initialized_count; index += 3)
        assert(initialized[index] == 8 && initialized[index + 1] == 32 && initialized[index + 2] == 2);
    _Alignas(8) LiblrhsmmRsUpdateOptionsF64 update; assert(liblrhsmm_rs_update_options_default_f64(&update) == 0);
    assert(liblrhsmm_rs_model_update_f64(initial, initial_stats, &update) == 0);
    assert(isfinite(infer_f64(initial, observation, segmentation)));
    assert(liblrhsmm_rs_model_statistics_release_f64(&initial_stats) == 0);
    assert(liblrhsmm_rs_model_release_f64(&initial) == 0);
    for(size_t mode = 0; mode < 2; ++mode) {
        LiblrhsmmRsModelStatisticsF64 *manual = NULL, *automatic = NULL;
        assert(liblrhsmm_rs_model_statistics_from_model_f64(model, &manual) == 0);
        assert(liblrhsmm_rs_model_statistics_from_model_f64(model, &automatic) == 0);
        _Alignas(8) LiblrhsmmRsHsmmSettingsF64 hsmm; assert(liblrhsmm_rs_hsmm_settings_default_f64(&hsmm) == 0);
        hsmm.temperature = 0.75; hsmm.state_radius = 1;
        _Alignas(8) LiblrhsmmRsGeometricSettingsF64 geo = {0.75, 0.5};
        _Alignas(8) LiblrhsmmRsEmissionSettingsF64 emission = {0.75, mode ? LIBLRHSMM_RS_EMISSION_PRUNING_LINEAR : LIBLRHSMM_RS_EMISSION_PRUNING_BOUNDARIES, 1, 0.5};
        LiblrhsmmRsLogEmissionsF64 *log = NULL;
        assert(liblrhsmm_rs_model_emission_log_probabilities_f64(model, observation, segmentation, &emission, &log) == 0);
        LiblrhsmmRsStateOccupanciesF64 *state = NULL;
        LiblrhsmmRsDurationOccupancies *durations = NULL;
        LiblrhsmmRsTransitionOccupancies *transitions = NULL;
        LiblrhsmmRsHsmmScoresF64 *hf = NULL, *hb = NULL;
        LiblrhsmmRsStateScoresF64 *gf = NULL, *gb = NULL;
        _Alignas(8) double total;
        if(mode == 0) {
            assert(liblrhsmm_rs_model_forward_hsmm_f64(model, segmentation, log, &hsmm, &hf) == 0);
            assert(liblrhsmm_rs_model_backward_hsmm_f64(model, segmentation, log, &hsmm, &hb) == 0);
            assert(liblrhsmm_rs_hsmm_scores_get_end_f64(hf, &gf) == 0);
            assert(liblrhsmm_rs_hsmm_scores_get_end_f64(hb, &gb) == 0);
            assert(liblrhsmm_rs_state_scores_total_through_state_f64(gf, gb, 0, &total) == 0);
            assert(liblrhsmm_rs_model_duration_occupancies_f64(model, segmentation, log, hf, hb, total, &hsmm, &durations) == 0);
            assert(liblrhsmm_rs_model_hsmm_state_occupancies_f64(model, segmentation, log, durations, &hsmm, &state) == 0);
        } else {
            assert(liblrhsmm_rs_model_forward_geometric_f64(model, segmentation, log, &geo, &gf) == 0);
            assert(liblrhsmm_rs_model_backward_geometric_f64(model, segmentation, log, &geo, &gb) == 0);
            assert(liblrhsmm_rs_state_scores_backward_total_f64(gb, log, &total) == 0);
            assert(liblrhsmm_rs_state_scores_geometric_occupancies_f64(gf, gb, total, geo.pruning_slope, &state) == 0);
            assert(liblrhsmm_rs_model_geometric_transition_occupancies_f64(model, segmentation, log, gf, gb, total, &geo, &transitions) == 0);
        }
        LiblrhsmmRsMixtureOccupancies *mixtures[2] = {NULL, NULL};
        const LiblrhsmmRsMixtureOccupancies *borrowed[2];
        for(size_t stream = 0; stream < 2; ++stream) {
            if(mode == 0) assert(liblrhsmm_rs_model_mixture_occupancies_hsmm_f64(model, observation, segmentation, state, stream, hsmm.temperature, hsmm.state_radius, &mixtures[stream]) == 0);
            else assert(liblrhsmm_rs_model_mixture_occupancies_geometric_f64(model, observation, segmentation, state, stream, geo.temperature, geo.pruning_slope, &mixtures[stream]) == 0);
            borrowed[stream] = mixtures[stream];
        }
        for(size_t repeat = 0; repeat < 2; ++repeat) {
            _Alignas(8) double actual = 99;
            if(mode == 0) {
                assert(liblrhsmm_rs_model_statistics_collect_hsmm_f64(manual, model, observation, segmentation, durations, state, borrowed, 2, hsmm.state_radius) == 0);
                assert(liblrhsmm_rs_model_statistics_estimate_hsmm_f64(automatic, model, observation, segmentation, &hsmm, &actual) == 0);
            } else {
                assert(liblrhsmm_rs_model_statistics_collect_geometric_f64(manual, model, observation, segmentation, transitions, state, borrowed, 2, geo.pruning_slope) == 0);
                assert(liblrhsmm_rs_model_statistics_estimate_geometric_f64(automatic, model, observation, segmentation, &geo, &actual) == 0);
            }
            assert(memcmp(&actual, &total, sizeof(total)) == 0);
            _Alignas(8) double a[128], b[128]; size_t count = snapshot_f64(manual, a);
            assert(count == snapshot_f64(automatic, b) && memcmp(a, b, count * sizeof(double)) == 0);
        }
        _Alignas(8) double before[128], after[128]; size_t count = snapshot_f64(automatic, before);
        assert(liblrhsmm_rs_model_statistics_estimate_hsmm_f64(automatic, model, observation, segmentation, &hsmm, NULL) == 1);
        assert(liblrhsmm_rs_model_statistics_estimate_geometric_f64(automatic, model, observation, segmentation, &geo, NULL) == 1);
        _Alignas(8) double sentinel = 99;
        hsmm.temperature = NAN; geo.temperature = NAN;
        assert(liblrhsmm_rs_model_statistics_estimate_hsmm_f64(automatic, model, observation, segmentation, &hsmm, &sentinel) == 3);
        assert(liblrhsmm_rs_model_statistics_estimate_geometric_f64(automatic, model, observation, segmentation, &geo, &sentinel) == 3);
        assert(sentinel == 99 && count == snapshot_f64(automatic, after) && memcmp(before, after, count * sizeof(double)) == 0);
        size_t manual_count = snapshot_f64(manual, before);
        const LiblrhsmmRsMixtureOccupancies *bad[2] = {borrowed[0], NULL};
        if(mode == 0) {
            assert(liblrhsmm_rs_model_statistics_collect_hsmm_f64(manual, model, observation, segmentation, durations, state, bad, 2, 1) == 1);
            assert(liblrhsmm_rs_model_statistics_collect_hsmm_f64(manual, model, observation, segmentation, durations, state, borrowed, 1, 1) == 3);
        } else {
            assert(liblrhsmm_rs_model_statistics_collect_geometric_f64(manual, model, observation, segmentation, transitions, state, bad, 2, 0.5) == 1);
            assert(liblrhsmm_rs_model_statistics_collect_geometric_f64(manual, model, observation, segmentation, transitions, state, borrowed, 1, 0.5) == 3);
        }
        assert(manual_count == snapshot_f64(manual, after) && memcmp(before, after, manual_count * sizeof(double)) == 0);
        LiblrhsmmRsModelF64 *trained = NULL; assert(liblrhsmm_rs_model_clone_f64(model, &trained) == 0);
        assert(liblrhsmm_rs_update_options_default_f64(&update) == 0); update.duration_mode = mode;
        assert(liblrhsmm_rs_model_update_f64(trained, automatic, &update) == 0);
        _Alignas(8) double likelihood = infer_f64(trained, observation, segmentation);
#ifdef LIBLRHSMM_RS_SERIALIZATION
        LiblrhsmmRsBytes *encoded = NULL; size_t length;
        assert(liblrhsmm_rs_model_write_bytes_f64(trained, LIBLRHSMM_RS_ENCODING_WITH_VARIANCE_FLOORS, &encoded) == 0);
        assert(liblrhsmm_rs_bytes_length(encoded, &length) == 0);
        uint8_t *data = malloc(length); assert(data != NULL);
        assert(liblrhsmm_rs_bytes_copy(encoded, 0, data, length) == 0);
        _Alignas(8) LiblrhsmmRsDecodeLimits limits; assert(liblrhsmm_rs_decode_limits_default(&limits) == 0);
        LiblrhsmmRsModelF64 *decoded = NULL;
        assert(liblrhsmm_rs_model_read_bytes_f64(data, length, &limits, &decoded) == 0);
        assert(liblrhsmm_rs_bytes_release(&encoded) == 0);
        assert(liblrhsmm_rs_model_release_f64(&trained) == 0); trained = decoded;
        _Alignas(8) double roundtrip = infer_f64(trained, observation, segmentation);
        assert(isfinite(roundtrip));
        LiblrhsmmRsBytes *reencoded = NULL; size_t second_length;
        assert(liblrhsmm_rs_model_write_bytes_f64(trained, LIBLRHSMM_RS_ENCODING_WITH_VARIANCE_FLOORS, &reencoded) == 0);
        assert(liblrhsmm_rs_bytes_length(reencoded, &second_length) == 0 && second_length == length);
        uint8_t *second_data = malloc(second_length); assert(second_data != NULL);
        assert(liblrhsmm_rs_bytes_copy(reencoded, 0, second_data, second_length) == 0);
        assert(memcmp(data, second_data, length) == 0);
        free(data); free(second_data); assert(liblrhsmm_rs_bytes_release(&reencoded) == 0);
#else
        assert(isfinite(likelihood));
#endif
        assert(liblrhsmm_rs_model_release_f64(&trained) == 0);
        for(size_t stream = 0; stream < 2; ++stream) assert(liblrhsmm_rs_mixture_occupancies_release(&mixtures[stream]) == 0);
        assert(liblrhsmm_rs_duration_occupancies_release(&durations) == 0);
        assert(liblrhsmm_rs_transition_occupancies_release(&transitions) == 0);
        assert(liblrhsmm_rs_state_occupancies_release_f64(&state) == 0);
        assert(liblrhsmm_rs_state_scores_release_f64(&gf) == 0);
        assert(liblrhsmm_rs_state_scores_release_f64(&gb) == 0);
        assert(liblrhsmm_rs_hsmm_scores_release_f64(&hf) == 0);
        assert(liblrhsmm_rs_hsmm_scores_release_f64(&hb) == 0);
        assert(liblrhsmm_rs_log_emissions_release_f64(&log) == 0);
        assert(liblrhsmm_rs_model_statistics_release_f64(&manual) == 0);
        assert(liblrhsmm_rs_model_statistics_release_f64(&automatic) == 0);
    }
    assert(liblrhsmm_rs_model_release_f64(&model) == 0);
    assert(liblrhsmm_rs_observation_release_f64(&observation) == 0);
    assert(liblrhsmm_rs_segmentation_release_f64(&segmentation) == 0);
}

int main(void) { exercise_f32(); exercise_f64(); puts("Training C: all10 exports, complete statistics and update/codec/inference pipelines passed"); return 0; }
