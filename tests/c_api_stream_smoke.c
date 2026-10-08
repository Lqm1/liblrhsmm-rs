/* Verify every stream and builder export through the generated public header. */
#ifdef NDEBUG
#undef NDEBUG
#endif
#include "liblrhsmm_rs.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

static void check_f32(const LiblrhsmmRsStreamF32 *stream, float weight, size_t count,
                    size_t first_mixtures, size_t first_dimensions, float mean) {
    LiblrhsmmRsStreamLayoutF32 layout;
    assert(liblrhsmm_rs_stream_layout_f32(stream, &layout) == 0);
    assert(memcmp(&layout.weight, &weight, sizeof(weight)) == 0);
    assert(layout.emissions == count);
    if (count) {
        LiblrhsmmRsGaussianF32 *child = NULL;
        assert(liblrhsmm_rs_stream_get_emission_f32(stream, 0, &child) == 0);
        LiblrhsmmRsGaussianLayout shape;
        assert(liblrhsmm_rs_gaussian_layout_f32(child, &shape) == 0);
        assert(shape.mixtures == first_mixtures && shape.dimensions == first_dimensions);
        float result = 0;
        assert(liblrhsmm_rs_gaussian_copy_field_f32(child, 1, 0, &result, 1) == 0);
        assert(memcmp(&result, &mean, sizeof(mean)) == 0);
        assert(liblrhsmm_rs_gaussian_release_f32(&child) == 0 && child == NULL);
    }
}
static void exercise_f32(void) {
    LiblrhsmmRsGaussianF32 *a = NULL, *b = NULL, *child = NULL;
    LiblrhsmmRsStreamF32 *stream = NULL, *copy = NULL, *created = NULL, *built = NULL, *rebuilt = NULL;
    LiblrhsmmRsStreamBuilderF32 *builder = NULL, *builder_copy = NULL;
    assert(liblrhsmm_rs_stream_new_f32(2, 1, 2, &stream) == 0);
    check_f32(stream, 1, 2, 1, 2, 0);
    assert(liblrhsmm_rs_gaussian_new_f32(1, 1, &a) == 0);
    assert(liblrhsmm_rs_gaussian_new_f32(2, 3, &b) == 0);
    float fields[4] = { 0.125, -0.0, INFINITY, -2.5 };
    for (uint32_t field = 0; field < 4; ++field) {
        assert(liblrhsmm_rs_gaussian_set_field_f32(a, field, 0, &fields[field], 1) == 0);
        assert(liblrhsmm_rs_gaussian_set_field_f32(b, field, 0, &fields[field], 1) == 0);
    }
    const LiblrhsmmRsGaussianF32 *inputs[] = {a, b, a};
    assert(liblrhsmm_rs_stream_replace_f32(stream, -0.0, inputs, 3) == 0);
    assert(liblrhsmm_rs_stream_create_f32(-0.0, inputs, 3, &created) == 0);
    assert(liblrhsmm_rs_stream_clone_f32(stream, &copy) == 0);
    check_f32(copy, -0.0, 3, 1, 1, -0.0);
    assert(liblrhsmm_rs_stream_set_weight_f32(stream, 0.5) == 0);
    assert(liblrhsmm_rs_stream_set_emission_f32(stream, 0, b) == 0);
    check_f32(stream, 0.5, 3, 2, 3, -0.0);
    assert(liblrhsmm_rs_stream_get_emission_f32(stream, 0, &child) == 0);
    for (uint32_t field = 0; field < 4; ++field) {
        float result;
        assert(liblrhsmm_rs_gaussian_copy_field_f32(child, field, 0, &result, 1) == 0);
        assert(memcmp(&result, &fields[field], sizeof(result)) == 0);
    }
    float changed = 99;
    assert(liblrhsmm_rs_gaussian_set_field_f32(child, 1, 0, &changed, 1) == 0);
    assert(liblrhsmm_rs_gaussian_release_f32(&child) == 0);
    check_f32(stream, 0.5, 3, 2, 3, -0.0);
    assert(liblrhsmm_rs_stream_set_emission_f32(stream, 3, a) == 2);
    assert(liblrhsmm_rs_stream_get_emission_f32(stream, 3, &child) == 2 && child == NULL);
    const LiblrhsmmRsGaussianF32 *invalid[] = {a, NULL};
    assert(liblrhsmm_rs_stream_replace_f32(stream, 9, invalid, 2) == 1);
    check_f32(stream, 0.5, 3, 2, 3, -0.0);
    LiblrhsmmRsStreamF32 *failure = stream;
    assert(liblrhsmm_rs_stream_create_f32(1, NULL, 2, &failure) == 1 && failure == stream);
    assert(liblrhsmm_rs_stream_new_f32(1, 0, 1, &failure) == 3 && failure == stream);
    assert(liblrhsmm_rs_stream_clone_f32(stream, NULL) == 1);
    assert(liblrhsmm_rs_stream_builder_new_f32(2, &builder) == 0);
    LiblrhsmmRsStreamLayoutF32 layout;
    assert(liblrhsmm_rs_stream_builder_layout_f32(builder, &layout) == 0);
    assert(layout.weight == 1 && layout.emissions == 2);
    uint32_t present = 99;
    assert(liblrhsmm_rs_stream_builder_has_emission_f32(builder, 0, &present) == 0 && present == 0);
    child = a;
    assert(liblrhsmm_rs_stream_builder_get_emission_f32(builder, 0, &child) == 3 && child == a);
    assert(liblrhsmm_rs_stream_builder_build_f32(builder, &failure) == 3 && failure == stream);
    assert(liblrhsmm_rs_stream_builder_set_weight_f32(builder, -0.0) == 0);
    assert(liblrhsmm_rs_stream_builder_set_emission_f32(builder, 0, a) == 0);
    assert(liblrhsmm_rs_stream_builder_set_emission_f32(builder, 1, b) == 0);
    assert(liblrhsmm_rs_stream_builder_has_emission_f32(builder, 0, &present) == 0 && present == 1);
    assert(liblrhsmm_rs_stream_builder_clone_f32(builder, &builder_copy) == 0);
    assert(liblrhsmm_rs_stream_builder_build_f32(builder, &built) == 0);
    child = NULL;
    assert(liblrhsmm_rs_stream_builder_get_emission_f32(builder, 0, &child) == 0);
    assert(liblrhsmm_rs_gaussian_set_field_f32(child, 1, 0, &changed, 1) == 0);
    assert(liblrhsmm_rs_gaussian_release_f32(&child) == 0);
    assert(liblrhsmm_rs_stream_builder_clear_emission_f32(builder, 0) == 0);
    assert(liblrhsmm_rs_stream_builder_build_f32(builder, &failure) == 3 && failure == stream);
    present = 99;
    assert(liblrhsmm_rs_stream_builder_has_emission_f32(builder, 2, &present) == 2 && present == 99);
    assert(liblrhsmm_rs_stream_builder_clear_emission_f32(builder, 2) == 2);
    assert(liblrhsmm_rs_stream_builder_set_emission_f32(builder, 2, a) == 2);
    assert(liblrhsmm_rs_stream_builder_get_emission_f32(builder, 2, &child) == 2 && child == NULL);
    assert(liblrhsmm_rs_gaussian_release_f32(&a) == 0);
    assert(liblrhsmm_rs_gaussian_release_f32(&b) == 0);
    assert(liblrhsmm_rs_stream_builder_release_f32(&builder) == 0 && builder == NULL);
    assert(liblrhsmm_rs_stream_builder_release_f32(&builder) == 0);
    assert(liblrhsmm_rs_stream_builder_build_f32(builder_copy, &rebuilt) == 0);
    check_f32(built, -0.0, 2, 1, 1, -0.0);
    check_f32(rebuilt, -0.0, 2, 1, 1, -0.0);
    assert(liblrhsmm_rs_stream_builder_release_f32(&builder_copy) == 0);
    assert(liblrhsmm_rs_stream_release_f32(&stream) == 0 && stream == NULL);
    check_f32(copy, -0.0, 3, 1, 1, -0.0);
    check_f32(created, -0.0, 3, 1, 1, -0.0);
    assert(liblrhsmm_rs_stream_release_f32(&copy) == 0);
    assert(liblrhsmm_rs_stream_release_f32(&created) == 0);
    assert(liblrhsmm_rs_stream_release_f32(&built) == 0);
    assert(liblrhsmm_rs_stream_release_f32(&rebuilt) == 0);
    assert(liblrhsmm_rs_stream_create_f32(-0.0, NULL, 0, &stream) == 0);
    check_f32(stream, -0.0, 0, 0, 0, 0);
    assert(liblrhsmm_rs_stream_replace_f32(stream, 1, NULL, 0) == 0);
    check_f32(stream, 1, 0, 0, 0, 0);
    assert(liblrhsmm_rs_stream_release_f32(&stream) == 0);
    assert(liblrhsmm_rs_stream_new_f32(0, 0, 0, &stream) == 0);
    check_f32(stream, 1, 0, 0, 0, 0);
    assert(liblrhsmm_rs_stream_release_f32(&stream) == 0);
    assert(liblrhsmm_rs_stream_builder_new_f32(0, &builder) == 0);
    assert(liblrhsmm_rs_stream_builder_build_f32(builder, &stream) == 0);
    check_f32(stream, 1, 0, 0, 0, 0);
    assert(liblrhsmm_rs_stream_release_f32(&stream) == 0);
    assert(liblrhsmm_rs_stream_builder_release_f32(&builder) == 0);
    assert(liblrhsmm_rs_stream_release_f32(NULL) == 1);
    assert(liblrhsmm_rs_stream_builder_release_f32(NULL) == 1);
}

static void check_f64(const LiblrhsmmRsStreamF64 *stream, double weight, size_t count,
                    size_t first_mixtures, size_t first_dimensions, double mean) {
    LiblrhsmmRsStreamLayoutF64 layout;
    assert(liblrhsmm_rs_stream_layout_f64(stream, &layout) == 0);
    assert(memcmp(&layout.weight, &weight, sizeof(weight)) == 0);
    assert(layout.emissions == count);
    if (count) {
        LiblrhsmmRsGaussianF64 *child = NULL;
        assert(liblrhsmm_rs_stream_get_emission_f64(stream, 0, &child) == 0);
        LiblrhsmmRsGaussianLayout shape;
        assert(liblrhsmm_rs_gaussian_layout_f64(child, &shape) == 0);
        assert(shape.mixtures == first_mixtures && shape.dimensions == first_dimensions);
        double result = 0;
        assert(liblrhsmm_rs_gaussian_copy_field_f64(child, 1, 0, &result, 1) == 0);
        assert(memcmp(&result, &mean, sizeof(mean)) == 0);
        assert(liblrhsmm_rs_gaussian_release_f64(&child) == 0 && child == NULL);
    }
}
static void exercise_f64(void) {
    LiblrhsmmRsGaussianF64 *a = NULL, *b = NULL, *child = NULL;
    LiblrhsmmRsStreamF64 *stream = NULL, *copy = NULL, *created = NULL, *built = NULL, *rebuilt = NULL;
    LiblrhsmmRsStreamBuilderF64 *builder = NULL, *builder_copy = NULL;
    assert(liblrhsmm_rs_stream_new_f64(2, 1, 2, &stream) == 0);
    check_f64(stream, 1, 2, 1, 2, 0);
    assert(liblrhsmm_rs_gaussian_new_f64(1, 1, &a) == 0);
    assert(liblrhsmm_rs_gaussian_new_f64(2, 3, &b) == 0);
    double fields[4] = { 0.125, -0.0, INFINITY, -2.5 };
    for (uint32_t field = 0; field < 4; ++field) {
        assert(liblrhsmm_rs_gaussian_set_field_f64(a, field, 0, &fields[field], 1) == 0);
        assert(liblrhsmm_rs_gaussian_set_field_f64(b, field, 0, &fields[field], 1) == 0);
    }
    const LiblrhsmmRsGaussianF64 *inputs[] = {a, b, a};
    assert(liblrhsmm_rs_stream_replace_f64(stream, -0.0, inputs, 3) == 0);
    assert(liblrhsmm_rs_stream_create_f64(-0.0, inputs, 3, &created) == 0);
    assert(liblrhsmm_rs_stream_clone_f64(stream, &copy) == 0);
    check_f64(copy, -0.0, 3, 1, 1, -0.0);
    assert(liblrhsmm_rs_stream_set_weight_f64(stream, 0.5) == 0);
    assert(liblrhsmm_rs_stream_set_emission_f64(stream, 0, b) == 0);
    check_f64(stream, 0.5, 3, 2, 3, -0.0);
    assert(liblrhsmm_rs_stream_get_emission_f64(stream, 0, &child) == 0);
    for (uint32_t field = 0; field < 4; ++field) {
        double result;
        assert(liblrhsmm_rs_gaussian_copy_field_f64(child, field, 0, &result, 1) == 0);
        assert(memcmp(&result, &fields[field], sizeof(result)) == 0);
    }
    double changed = 99;
    assert(liblrhsmm_rs_gaussian_set_field_f64(child, 1, 0, &changed, 1) == 0);
    assert(liblrhsmm_rs_gaussian_release_f64(&child) == 0);
    check_f64(stream, 0.5, 3, 2, 3, -0.0);
    assert(liblrhsmm_rs_stream_set_emission_f64(stream, 3, a) == 2);
    assert(liblrhsmm_rs_stream_get_emission_f64(stream, 3, &child) == 2 && child == NULL);
    const LiblrhsmmRsGaussianF64 *invalid[] = {a, NULL};
    assert(liblrhsmm_rs_stream_replace_f64(stream, 9, invalid, 2) == 1);
    check_f64(stream, 0.5, 3, 2, 3, -0.0);
    LiblrhsmmRsStreamF64 *failure = stream;
    assert(liblrhsmm_rs_stream_create_f64(1, NULL, 2, &failure) == 1 && failure == stream);
    assert(liblrhsmm_rs_stream_new_f64(1, 0, 1, &failure) == 3 && failure == stream);
    assert(liblrhsmm_rs_stream_clone_f64(stream, NULL) == 1);
    assert(liblrhsmm_rs_stream_builder_new_f64(2, &builder) == 0);
    LiblrhsmmRsStreamLayoutF64 layout;
    assert(liblrhsmm_rs_stream_builder_layout_f64(builder, &layout) == 0);
    assert(layout.weight == 1 && layout.emissions == 2);
    uint32_t present = 99;
    assert(liblrhsmm_rs_stream_builder_has_emission_f64(builder, 0, &present) == 0 && present == 0);
    child = a;
    assert(liblrhsmm_rs_stream_builder_get_emission_f64(builder, 0, &child) == 3 && child == a);
    assert(liblrhsmm_rs_stream_builder_build_f64(builder, &failure) == 3 && failure == stream);
    assert(liblrhsmm_rs_stream_builder_set_weight_f64(builder, -0.0) == 0);
    assert(liblrhsmm_rs_stream_builder_set_emission_f64(builder, 0, a) == 0);
    assert(liblrhsmm_rs_stream_builder_set_emission_f64(builder, 1, b) == 0);
    assert(liblrhsmm_rs_stream_builder_has_emission_f64(builder, 0, &present) == 0 && present == 1);
    assert(liblrhsmm_rs_stream_builder_clone_f64(builder, &builder_copy) == 0);
    assert(liblrhsmm_rs_stream_builder_build_f64(builder, &built) == 0);
    child = NULL;
    assert(liblrhsmm_rs_stream_builder_get_emission_f64(builder, 0, &child) == 0);
    assert(liblrhsmm_rs_gaussian_set_field_f64(child, 1, 0, &changed, 1) == 0);
    assert(liblrhsmm_rs_gaussian_release_f64(&child) == 0);
    assert(liblrhsmm_rs_stream_builder_clear_emission_f64(builder, 0) == 0);
    assert(liblrhsmm_rs_stream_builder_build_f64(builder, &failure) == 3 && failure == stream);
    present = 99;
    assert(liblrhsmm_rs_stream_builder_has_emission_f64(builder, 2, &present) == 2 && present == 99);
    assert(liblrhsmm_rs_stream_builder_clear_emission_f64(builder, 2) == 2);
    assert(liblrhsmm_rs_stream_builder_set_emission_f64(builder, 2, a) == 2);
    assert(liblrhsmm_rs_stream_builder_get_emission_f64(builder, 2, &child) == 2 && child == NULL);
    assert(liblrhsmm_rs_gaussian_release_f64(&a) == 0);
    assert(liblrhsmm_rs_gaussian_release_f64(&b) == 0);
    assert(liblrhsmm_rs_stream_builder_release_f64(&builder) == 0 && builder == NULL);
    assert(liblrhsmm_rs_stream_builder_release_f64(&builder) == 0);
    assert(liblrhsmm_rs_stream_builder_build_f64(builder_copy, &rebuilt) == 0);
    check_f64(built, -0.0, 2, 1, 1, -0.0);
    check_f64(rebuilt, -0.0, 2, 1, 1, -0.0);
    assert(liblrhsmm_rs_stream_builder_release_f64(&builder_copy) == 0);
    assert(liblrhsmm_rs_stream_release_f64(&stream) == 0 && stream == NULL);
    check_f64(copy, -0.0, 3, 1, 1, -0.0);
    check_f64(created, -0.0, 3, 1, 1, -0.0);
    assert(liblrhsmm_rs_stream_release_f64(&copy) == 0);
    assert(liblrhsmm_rs_stream_release_f64(&created) == 0);
    assert(liblrhsmm_rs_stream_release_f64(&built) == 0);
    assert(liblrhsmm_rs_stream_release_f64(&rebuilt) == 0);
    assert(liblrhsmm_rs_stream_create_f64(-0.0, NULL, 0, &stream) == 0);
    check_f64(stream, -0.0, 0, 0, 0, 0);
    assert(liblrhsmm_rs_stream_replace_f64(stream, 1, NULL, 0) == 0);
    check_f64(stream, 1, 0, 0, 0, 0);
    assert(liblrhsmm_rs_stream_release_f64(&stream) == 0);
    assert(liblrhsmm_rs_stream_new_f64(0, 0, 0, &stream) == 0);
    check_f64(stream, 1, 0, 0, 0, 0);
    assert(liblrhsmm_rs_stream_release_f64(&stream) == 0);
    assert(liblrhsmm_rs_stream_builder_new_f64(0, &builder) == 0);
    assert(liblrhsmm_rs_stream_builder_build_f64(builder, &stream) == 0);
    check_f64(stream, 1, 0, 0, 0, 0);
    assert(liblrhsmm_rs_stream_release_f64(&stream) == 0);
    assert(liblrhsmm_rs_stream_builder_release_f64(&builder) == 0);
    assert(liblrhsmm_rs_stream_release_f64(NULL) == 1);
    assert(liblrhsmm_rs_stream_builder_release_f64(NULL) == 1);
}

int main(void) { exercise_f32(); exercise_f64(); puts("stream: all 38 exports passed"); return 0; }
