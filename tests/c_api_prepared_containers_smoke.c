/* Actual C calls to every prepared stream/model function. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <stdio.h>

static void exercise_f32(void) {
    float weights[] = {1}, means[] = {0}, variances[] = {1}, floors[] = {0.1};
    LiblrhsmmRsGaussianParametersF32 input = {1, 1, weights, means, variances, floors};
    LiblrhsmmRsGaussianF32 *gaussian = NULL;
    LiblrhsmmRsStreamF32 *stream = NULL, *editable_stream = NULL;
    LiblrhsmmRsPreparedStreamF32 *prepared_stream = NULL, *stream_copy = NULL;
    LiblrhsmmRsPreparedGaussianF32 *child = NULL;
    assert(liblrhsmm_rs_gaussian_create_f32(&input, &gaussian) == 0);
    const LiblrhsmmRsGaussianF32 *emissions[] = {gaussian};
    assert(liblrhsmm_rs_stream_create_f32(0.75, emissions, 1, &stream) == 0);
    assert(liblrhsmm_rs_gaussian_release_f32(&gaussian) == 0);
    assert(liblrhsmm_rs_stream_prepare_f32(stream, &prepared_stream) == 0);
    LiblrhsmmRsStreamLayoutF32 stream_layout = {99, 99};
    assert(liblrhsmm_rs_prepared_stream_layout_f32(prepared_stream, &stream_layout) == 0);
    assert(stream_layout.weight == 0.75 && stream_layout.emissions == 1);
    assert(liblrhsmm_rs_prepared_stream_parameters_f32(prepared_stream, &editable_stream) == 0);
    assert(liblrhsmm_rs_stream_set_weight_f32(editable_stream, -99) == 0);
    assert(liblrhsmm_rs_stream_release_f32(&editable_stream) == 0);
    assert(liblrhsmm_rs_prepared_stream_clone_f32(prepared_stream, &stream_copy) == 0);
    assert(liblrhsmm_rs_prepared_stream_get_emission_f32(stream_copy, 1, &child) == 2 && child == NULL);
    assert(liblrhsmm_rs_prepared_stream_get_emission_f32(stream_copy, 0, &child) == 0);
    assert(liblrhsmm_rs_prepared_stream_release_f32(&prepared_stream) == 0);
    assert(liblrhsmm_rs_prepared_stream_release_f32(&stream_copy) == 0);
    uintptr_t count = 99;
    assert(liblrhsmm_rs_prepared_gaussian_cache_length_f32(child, &count) == 0 && count == 1);
    assert(liblrhsmm_rs_prepared_gaussian_release_f32(&child) == 0);
    LiblrhsmmRsDurationParametersF32 duration_input = {4, 1, 0, 300, -1, 0.05};
    LiblrhsmmRsDurationF32 *duration = NULL;
    assert(liblrhsmm_rs_duration_create_f32(&duration_input, &duration) == 0);
    const LiblrhsmmRsStreamF32 *streams[] = {stream};
    const LiblrhsmmRsDurationF32 *durations[] = {duration, duration};
    LiblrhsmmRsModelF32 *model = NULL, *editable_model = NULL;
    LiblrhsmmRsPreparedModelF32 *prepared_model = NULL, *model_copy = NULL;
    assert(liblrhsmm_rs_model_create_f32(streams, 1, durations, 2, &model) == 0);
    assert(liblrhsmm_rs_stream_release_f32(&stream) == 0);
    assert(liblrhsmm_rs_duration_release_f32(&duration) == 0);
    LiblrhsmmRsDurationPreparationF32 options = {3, 0.75, 0.5}, fetched = {99, 99, 99};
    assert(liblrhsmm_rs_model_prepare_f32(model, &options, &prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_model_clone_f32(prepared_model, &model_copy) == 0);
    LiblrhsmmRsModelLayout layout = {99, 99};
    assert(liblrhsmm_rs_prepared_model_layout_f32(model_copy, &layout) == 0 && layout.streams == 1 && layout.durations == 2);
    assert(liblrhsmm_rs_prepared_model_options_f32(model_copy, &fetched) == 0);
    assert(fetched.duration_cache_length == 3 && fetched.duration_weight == 0.75 && fetched.temperature == 0.5);
    assert(liblrhsmm_rs_prepared_model_parameters_f32(model_copy, &editable_model) == 0);
    assert(liblrhsmm_rs_model_release_f32(&editable_model) == 0);
    assert(liblrhsmm_rs_prepared_model_stream_layout_f32(model_copy, 0, &stream_layout) == 0 && stream_layout.weight == 0.75 && stream_layout.emissions == 1);
    assert(liblrhsmm_rs_prepared_model_stream_parameters_f32(model_copy, 0, &editable_stream) == 0);
    assert(liblrhsmm_rs_stream_release_f32(&editable_stream) == 0);
    uint32_t available = 99;
    assert(liblrhsmm_rs_prepared_model_has_duration_f32(model_copy, 1, &available) == 0 && available == 1);
    LiblrhsmmRsPreparedDurationF32 *prepared_duration = NULL;
    assert(liblrhsmm_rs_prepared_model_get_duration_f32(model_copy, 1, &prepared_duration) == 0);
    assert(liblrhsmm_rs_prepared_model_get_emission_f32(model_copy, 0, 0, &child) == 0);
    assert(liblrhsmm_rs_prepared_model_get_emission_f32(model_copy, 1, 0, &child) == 2);
    assert(liblrhsmm_rs_prepared_model_release_f32(&prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_model_release_f32(&model_copy) == 0);
    assert(liblrhsmm_rs_prepared_duration_cache_length_f32(prepared_duration, &count) == 0 && count == 3);
    assert(liblrhsmm_rs_prepared_duration_release_f32(&prepared_duration) == 0);
    assert(liblrhsmm_rs_prepared_gaussian_release_f32(&child) == 0);
    const uintptr_t selected[] = {0, 0};
    assert(liblrhsmm_rs_model_prepare_selected_durations_f32(model, &options, selected, 2, &prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_model_has_duration_f32(prepared_model, 1, &available) == 0 && available == 0);
    assert(liblrhsmm_rs_prepared_model_get_duration_f32(prepared_model, 1, &prepared_duration) == 3 && prepared_duration == NULL);
    assert(liblrhsmm_rs_prepared_model_has_duration_f32(prepared_model, 2, &available) == 2 && available == 0);
    assert(liblrhsmm_rs_prepared_model_release_f32(&prepared_model) == 0);
    assert(liblrhsmm_rs_model_prepare_selected_durations_f32(model, &options, NULL, 0, &prepared_model) == 0);
    assert(liblrhsmm_rs_model_release_f32(&model) == 0);
    assert(liblrhsmm_rs_prepared_model_layout_f32(prepared_model, &layout) == 0 && layout.durations == 2);
    assert(liblrhsmm_rs_prepared_model_release_f32(&prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_model_release_f32(&prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_stream_release_f32(&prepared_stream) == 0);
    assert(liblrhsmm_rs_prepared_model_release_f32(NULL) == 1);
    assert(liblrhsmm_rs_prepared_stream_release_f32(NULL) == 1);
}

/* Actual C calls to every prepared stream/model function. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <stdio.h>

static void exercise_f64(void) {
    double weights[] = {1}, means[] = {0}, variances[] = {1}, floors[] = {0.1};
    LiblrhsmmRsGaussianParametersF64 input = {1, 1, weights, means, variances, floors};
    LiblrhsmmRsGaussianF64 *gaussian = NULL;
    LiblrhsmmRsStreamF64 *stream = NULL, *editable_stream = NULL;
    LiblrhsmmRsPreparedStreamF64 *prepared_stream = NULL, *stream_copy = NULL;
    LiblrhsmmRsPreparedGaussianF64 *child = NULL;
    assert(liblrhsmm_rs_gaussian_create_f64(&input, &gaussian) == 0);
    const LiblrhsmmRsGaussianF64 *emissions[] = {gaussian};
    assert(liblrhsmm_rs_stream_create_f64(0.75, emissions, 1, &stream) == 0);
    assert(liblrhsmm_rs_gaussian_release_f64(&gaussian) == 0);
    assert(liblrhsmm_rs_stream_prepare_f64(stream, &prepared_stream) == 0);
    LiblrhsmmRsStreamLayoutF64 stream_layout = {99, 99};
    assert(liblrhsmm_rs_prepared_stream_layout_f64(prepared_stream, &stream_layout) == 0);
    assert(stream_layout.weight == 0.75 && stream_layout.emissions == 1);
    assert(liblrhsmm_rs_prepared_stream_parameters_f64(prepared_stream, &editable_stream) == 0);
    assert(liblrhsmm_rs_stream_set_weight_f64(editable_stream, -99) == 0);
    assert(liblrhsmm_rs_stream_release_f64(&editable_stream) == 0);
    assert(liblrhsmm_rs_prepared_stream_clone_f64(prepared_stream, &stream_copy) == 0);
    assert(liblrhsmm_rs_prepared_stream_get_emission_f64(stream_copy, 1, &child) == 2 && child == NULL);
    assert(liblrhsmm_rs_prepared_stream_get_emission_f64(stream_copy, 0, &child) == 0);
    assert(liblrhsmm_rs_prepared_stream_release_f64(&prepared_stream) == 0);
    assert(liblrhsmm_rs_prepared_stream_release_f64(&stream_copy) == 0);
    uintptr_t count = 99;
    assert(liblrhsmm_rs_prepared_gaussian_cache_length_f64(child, &count) == 0 && count == 1);
    assert(liblrhsmm_rs_prepared_gaussian_release_f64(&child) == 0);
    LiblrhsmmRsDurationParametersF64 duration_input = {4, 1, 0, 300, -1, 0.05};
    LiblrhsmmRsDurationF64 *duration = NULL;
    assert(liblrhsmm_rs_duration_create_f64(&duration_input, &duration) == 0);
    const LiblrhsmmRsStreamF64 *streams[] = {stream};
    const LiblrhsmmRsDurationF64 *durations[] = {duration, duration};
    LiblrhsmmRsModelF64 *model = NULL, *editable_model = NULL;
    LiblrhsmmRsPreparedModelF64 *prepared_model = NULL, *model_copy = NULL;
    assert(liblrhsmm_rs_model_create_f64(streams, 1, durations, 2, &model) == 0);
    assert(liblrhsmm_rs_stream_release_f64(&stream) == 0);
    assert(liblrhsmm_rs_duration_release_f64(&duration) == 0);
    LiblrhsmmRsDurationPreparationF64 options = {3, 0.75, 0.5}, fetched = {99, 99, 99};
    assert(liblrhsmm_rs_model_prepare_f64(model, &options, &prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_model_clone_f64(prepared_model, &model_copy) == 0);
    LiblrhsmmRsModelLayout layout = {99, 99};
    assert(liblrhsmm_rs_prepared_model_layout_f64(model_copy, &layout) == 0 && layout.streams == 1 && layout.durations == 2);
    assert(liblrhsmm_rs_prepared_model_options_f64(model_copy, &fetched) == 0);
    assert(fetched.duration_cache_length == 3 && fetched.duration_weight == 0.75 && fetched.temperature == 0.5);
    assert(liblrhsmm_rs_prepared_model_parameters_f64(model_copy, &editable_model) == 0);
    assert(liblrhsmm_rs_model_release_f64(&editable_model) == 0);
    assert(liblrhsmm_rs_prepared_model_stream_layout_f64(model_copy, 0, &stream_layout) == 0 && stream_layout.weight == 0.75 && stream_layout.emissions == 1);
    assert(liblrhsmm_rs_prepared_model_stream_parameters_f64(model_copy, 0, &editable_stream) == 0);
    assert(liblrhsmm_rs_stream_release_f64(&editable_stream) == 0);
    uint32_t available = 99;
    assert(liblrhsmm_rs_prepared_model_has_duration_f64(model_copy, 1, &available) == 0 && available == 1);
    LiblrhsmmRsPreparedDurationF64 *prepared_duration = NULL;
    assert(liblrhsmm_rs_prepared_model_get_duration_f64(model_copy, 1, &prepared_duration) == 0);
    assert(liblrhsmm_rs_prepared_model_get_emission_f64(model_copy, 0, 0, &child) == 0);
    assert(liblrhsmm_rs_prepared_model_get_emission_f64(model_copy, 1, 0, &child) == 2);
    assert(liblrhsmm_rs_prepared_model_release_f64(&prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_model_release_f64(&model_copy) == 0);
    assert(liblrhsmm_rs_prepared_duration_cache_length_f64(prepared_duration, &count) == 0 && count == 3);
    assert(liblrhsmm_rs_prepared_duration_release_f64(&prepared_duration) == 0);
    assert(liblrhsmm_rs_prepared_gaussian_release_f64(&child) == 0);
    const uintptr_t selected[] = {0, 0};
    assert(liblrhsmm_rs_model_prepare_selected_durations_f64(model, &options, selected, 2, &prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_model_has_duration_f64(prepared_model, 1, &available) == 0 && available == 0);
    assert(liblrhsmm_rs_prepared_model_get_duration_f64(prepared_model, 1, &prepared_duration) == 3 && prepared_duration == NULL);
    assert(liblrhsmm_rs_prepared_model_has_duration_f64(prepared_model, 2, &available) == 2 && available == 0);
    assert(liblrhsmm_rs_prepared_model_release_f64(&prepared_model) == 0);
    assert(liblrhsmm_rs_model_prepare_selected_durations_f64(model, &options, NULL, 0, &prepared_model) == 0);
    assert(liblrhsmm_rs_model_release_f64(&model) == 0);
    assert(liblrhsmm_rs_prepared_model_layout_f64(prepared_model, &layout) == 0 && layout.durations == 2);
    assert(liblrhsmm_rs_prepared_model_release_f64(&prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_model_release_f64(&prepared_model) == 0);
    assert(liblrhsmm_rs_prepared_stream_release_f64(&prepared_stream) == 0);
    assert(liblrhsmm_rs_prepared_model_release_f64(NULL) == 1);
    assert(liblrhsmm_rs_prepared_stream_release_f64(NULL) == 1);
}

int main(void) { exercise_f32(); exercise_f64(); puts("prepared containers: all36 exports and independent child lifetimes passed"); return 0; }
