/* Actual C caller for model statistics ownership and transactional maximization. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <float.h>
#include <math.h>
#include <stdio.h>

static void exercise_f32(void) {
    LiblrhsmmRsModelF32 *model = NULL;
    LiblrhsmmRsModelStatisticsF32 *statistics = NULL, *copy = NULL, *assembled = NULL;
    LiblrhsmmRsStreamStatisticsF32 *stream = NULL;
    LiblrhsmmRsDurationStatisticsF32 *duration = NULL;
    _Alignas(8) const LiblrhsmmRsStreamSpecification specification = {1, 1, 1};
    assert(liblrhsmm_rs_model_new_f32(&specification, 1, 2, &model) == 0);
    assert(liblrhsmm_rs_model_statistics_from_model_f32(model, &statistics) == 0);
    _Alignas(8) LiblrhsmmRsModelLayout layout = {99, 99};
    assert(liblrhsmm_rs_model_statistics_layout_f32(statistics, &layout) == 0);
    assert(layout.streams == 1 && layout.durations == 2);
    assert(liblrhsmm_rs_model_statistics_get_stream_f32(statistics, 0, &stream) == 0);
    assert(liblrhsmm_rs_model_statistics_set_stream_f32(statistics, 0, stream) == 0);
    assert(liblrhsmm_rs_model_statistics_get_duration_f32(statistics, 0, &duration) == 0);
    _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF32 good = {6, 20, 2};
    _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF32 bad = {FLT_MAX, 20, 2};
    assert(liblrhsmm_rs_duration_statistics_set_f32(duration, &good) == 0);
    assert(liblrhsmm_rs_model_statistics_set_duration_f32(statistics, 0, duration) == 0);
    assert(liblrhsmm_rs_duration_statistics_set_f32(duration, &bad) == 0);
    assert(liblrhsmm_rs_model_statistics_set_duration_f32(statistics, 1, duration) == 0);
    _Alignas(8) LiblrhsmmRsUpdateOptionsF32 options;
    assert(liblrhsmm_rs_update_options_default_f32(&options) == 0);
    LiblrhsmmRsDurationF32 *parameters = NULL;
    _Alignas(8) LiblrhsmmRsDurationParametersF32 before, after;
    assert(liblrhsmm_rs_model_get_duration_f32(model, 0, &parameters) == 0);
    assert(liblrhsmm_rs_duration_get_f32(parameters, &before) == 0);
    assert(liblrhsmm_rs_duration_release_f32(&parameters) == 0);
    assert(liblrhsmm_rs_model_update_f32(model, statistics, &options) == 3);
    assert(liblrhsmm_rs_model_get_duration_f32(model, 0, &parameters) == 0);
    assert(liblrhsmm_rs_duration_get_f32(parameters, &after) == 0);
    assert(after.mean == before.mean && after.variance == before.variance);
    assert(after.minimum == before.minimum && after.maximum == before.maximum);
    assert(after.fixed_mean == before.fixed_mean && after.variance_floor == before.variance_floor);
    assert(liblrhsmm_rs_duration_release_f32(&parameters) == 0);
    assert(liblrhsmm_rs_duration_statistics_set_f32(duration, &good) == 0);
    assert(liblrhsmm_rs_model_statistics_set_duration_f32(statistics, 1, duration) == 0);
    assert(liblrhsmm_rs_model_statistics_clone_f32(statistics, &copy) == 0);
    assert(liblrhsmm_rs_model_statistics_clear_f32(statistics) == 0);
    assert(liblrhsmm_rs_model_update_f32(model, copy, &options) == 0);
    assert(liblrhsmm_rs_model_get_duration_f32(model, 0, &parameters) == 0);
    assert(liblrhsmm_rs_duration_get_f32(parameters, &after) == 0 && after.mean == 3 && after.variance == 1);
    assert(liblrhsmm_rs_duration_release_f32(&parameters) == 0);
    const LiblrhsmmRsStreamStatisticsF32 *streams[] = {stream, stream};
    const LiblrhsmmRsDurationStatisticsF32 *durations[] = {duration, duration};
    assert(liblrhsmm_rs_model_statistics_create_f32(streams, 2, durations, 2, &assembled) == 0);
    assert(liblrhsmm_rs_stream_statistics_release_f32(&stream) == 0);
    assert(liblrhsmm_rs_duration_statistics_release_f32(&duration) == 0);
    assert(liblrhsmm_rs_model_statistics_get_duration_f32(assembled, 1, &duration) == 0);
    _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF32 observed;
    assert(liblrhsmm_rs_duration_statistics_get_f32(duration, &observed) == 0);
    assert(observed.value_sum == 6 && observed.squared_sum == 20 && observed.occupancy == 2);
    assert(liblrhsmm_rs_duration_statistics_release_f32(&duration) == 0);
    const LiblrhsmmRsDurationStatisticsF32 *invalid[] = {NULL};
    assert(liblrhsmm_rs_model_statistics_replace_f32(assembled, NULL, 0, invalid, 1) == 1);
    assert(liblrhsmm_rs_model_statistics_layout_f32(assembled, &layout) == 0 && layout.streams == 2 && layout.durations == 2);
    assert(liblrhsmm_rs_model_statistics_get_stream_f32(assembled, 2, &stream) == 2 && stream == NULL);
    assert(liblrhsmm_rs_model_statistics_get_duration_f32(assembled, 2, &duration) == 2 && duration == NULL);
    assert(liblrhsmm_rs_model_statistics_replace_f32(assembled, NULL, 0, NULL, 0) == 0);
    assert(liblrhsmm_rs_model_statistics_layout_f32(assembled, &layout) == 0 && layout.streams == 0 && layout.durations == 0);
    assert(liblrhsmm_rs_model_release_f32(&model) == 0);
    assert(liblrhsmm_rs_model_statistics_release_f32(&statistics) == 0 && statistics == NULL);
    assert(liblrhsmm_rs_model_statistics_release_f32(&copy) == 0 && copy == NULL);
    assert(liblrhsmm_rs_model_statistics_release_f32(&assembled) == 0 && assembled == NULL);
    assert(liblrhsmm_rs_model_statistics_release_f32(&assembled) == 0);
    assert(liblrhsmm_rs_model_statistics_release_f32(NULL) == 1);
}

/* Actual C caller for model statistics ownership and transactional maximization. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <float.h>
#include <math.h>
#include <stdio.h>

static void exercise_f64(void) {
    LiblrhsmmRsModelF64 *model = NULL;
    LiblrhsmmRsModelStatisticsF64 *statistics = NULL, *copy = NULL, *assembled = NULL;
    LiblrhsmmRsStreamStatisticsF64 *stream = NULL;
    LiblrhsmmRsDurationStatisticsF64 *duration = NULL;
    _Alignas(8) const LiblrhsmmRsStreamSpecification specification = {1, 1, 1};
    assert(liblrhsmm_rs_model_new_f64(&specification, 1, 2, &model) == 0);
    assert(liblrhsmm_rs_model_statistics_from_model_f64(model, &statistics) == 0);
    _Alignas(8) LiblrhsmmRsModelLayout layout = {99, 99};
    assert(liblrhsmm_rs_model_statistics_layout_f64(statistics, &layout) == 0);
    assert(layout.streams == 1 && layout.durations == 2);
    assert(liblrhsmm_rs_model_statistics_get_stream_f64(statistics, 0, &stream) == 0);
    assert(liblrhsmm_rs_model_statistics_set_stream_f64(statistics, 0, stream) == 0);
    assert(liblrhsmm_rs_model_statistics_get_duration_f64(statistics, 0, &duration) == 0);
    _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF64 good = {6, 20, 2};
    _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF64 bad = {DBL_MAX, 20, 2};
    assert(liblrhsmm_rs_duration_statistics_set_f64(duration, &good) == 0);
    assert(liblrhsmm_rs_model_statistics_set_duration_f64(statistics, 0, duration) == 0);
    assert(liblrhsmm_rs_duration_statistics_set_f64(duration, &bad) == 0);
    assert(liblrhsmm_rs_model_statistics_set_duration_f64(statistics, 1, duration) == 0);
    _Alignas(8) LiblrhsmmRsUpdateOptionsF64 options;
    assert(liblrhsmm_rs_update_options_default_f64(&options) == 0);
    LiblrhsmmRsDurationF64 *parameters = NULL;
    _Alignas(8) LiblrhsmmRsDurationParametersF64 before, after;
    assert(liblrhsmm_rs_model_get_duration_f64(model, 0, &parameters) == 0);
    assert(liblrhsmm_rs_duration_get_f64(parameters, &before) == 0);
    assert(liblrhsmm_rs_duration_release_f64(&parameters) == 0);
    assert(liblrhsmm_rs_model_update_f64(model, statistics, &options) == 3);
    assert(liblrhsmm_rs_model_get_duration_f64(model, 0, &parameters) == 0);
    assert(liblrhsmm_rs_duration_get_f64(parameters, &after) == 0);
    assert(after.mean == before.mean && after.variance == before.variance);
    assert(after.minimum == before.minimum && after.maximum == before.maximum);
    assert(after.fixed_mean == before.fixed_mean && after.variance_floor == before.variance_floor);
    assert(liblrhsmm_rs_duration_release_f64(&parameters) == 0);
    assert(liblrhsmm_rs_duration_statistics_set_f64(duration, &good) == 0);
    assert(liblrhsmm_rs_model_statistics_set_duration_f64(statistics, 1, duration) == 0);
    assert(liblrhsmm_rs_model_statistics_clone_f64(statistics, &copy) == 0);
    assert(liblrhsmm_rs_model_statistics_clear_f64(statistics) == 0);
    assert(liblrhsmm_rs_model_update_f64(model, copy, &options) == 0);
    assert(liblrhsmm_rs_model_get_duration_f64(model, 0, &parameters) == 0);
    assert(liblrhsmm_rs_duration_get_f64(parameters, &after) == 0 && after.mean == 3 && after.variance == 1);
    assert(liblrhsmm_rs_duration_release_f64(&parameters) == 0);
    const LiblrhsmmRsStreamStatisticsF64 *streams[] = {stream, stream};
    const LiblrhsmmRsDurationStatisticsF64 *durations[] = {duration, duration};
    assert(liblrhsmm_rs_model_statistics_create_f64(streams, 2, durations, 2, &assembled) == 0);
    assert(liblrhsmm_rs_stream_statistics_release_f64(&stream) == 0);
    assert(liblrhsmm_rs_duration_statistics_release_f64(&duration) == 0);
    assert(liblrhsmm_rs_model_statistics_get_duration_f64(assembled, 1, &duration) == 0);
    _Alignas(8) LiblrhsmmRsDurationStatisticsValuesF64 observed;
    assert(liblrhsmm_rs_duration_statistics_get_f64(duration, &observed) == 0);
    assert(observed.value_sum == 6 && observed.squared_sum == 20 && observed.occupancy == 2);
    assert(liblrhsmm_rs_duration_statistics_release_f64(&duration) == 0);
    const LiblrhsmmRsDurationStatisticsF64 *invalid[] = {NULL};
    assert(liblrhsmm_rs_model_statistics_replace_f64(assembled, NULL, 0, invalid, 1) == 1);
    assert(liblrhsmm_rs_model_statistics_layout_f64(assembled, &layout) == 0 && layout.streams == 2 && layout.durations == 2);
    assert(liblrhsmm_rs_model_statistics_get_stream_f64(assembled, 2, &stream) == 2 && stream == NULL);
    assert(liblrhsmm_rs_model_statistics_get_duration_f64(assembled, 2, &duration) == 2 && duration == NULL);
    assert(liblrhsmm_rs_model_statistics_replace_f64(assembled, NULL, 0, NULL, 0) == 0);
    assert(liblrhsmm_rs_model_statistics_layout_f64(assembled, &layout) == 0 && layout.streams == 0 && layout.durations == 0);
    assert(liblrhsmm_rs_model_release_f64(&model) == 0);
    assert(liblrhsmm_rs_model_statistics_release_f64(&statistics) == 0 && statistics == NULL);
    assert(liblrhsmm_rs_model_statistics_release_f64(&copy) == 0 && copy == NULL);
    assert(liblrhsmm_rs_model_statistics_release_f64(&assembled) == 0 && assembled == NULL);
    assert(liblrhsmm_rs_model_statistics_release_f64(&assembled) == 0);
    assert(liblrhsmm_rs_model_statistics_release_f64(NULL) == 1);
}

int main(void) { exercise_f32(); exercise_f64(); puts("model statistics: all24 exports, independent copies and late update failure passed"); return 0; }
