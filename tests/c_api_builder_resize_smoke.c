#include "liblrhsmm_rs.h"
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#define VERIFY(SUFFIX, STREAM_OWNER, MODEL_OWNER, STREAM_LAYOUT) do { \
    STREAM_OWNER *stream = NULL; \
    MODEL_OWNER *model = NULL; \
    STREAM_LAYOUT stream_layout; \
    struct LiblrhsmmRsModelLayout model_layout; \
    assert(liblrhsmm_rs_stream_builder_new_##SUFFIX(2, &stream) == 0); \
    assert(liblrhsmm_rs_stream_builder_set_weight_##SUFFIX(stream, 0.75) == 0); \
    assert(liblrhsmm_rs_stream_builder_resize_##SUFFIX(stream, 4) == 0); \
    assert(liblrhsmm_rs_stream_builder_resize_##SUFFIX(stream, (size_t)INT32_MAX + 1) == 3); \
    assert(liblrhsmm_rs_stream_builder_layout_##SUFFIX(stream, &stream_layout) == 0); \
    assert(stream_layout.emissions == 4 && stream_layout.weight == 0.75); \
    assert(liblrhsmm_rs_stream_builder_resize_##SUFFIX(stream, 0) == 0); \
    assert(liblrhsmm_rs_stream_builder_release_##SUFFIX(&stream) == 0); \
    assert(liblrhsmm_rs_model_builder_new_##SUFFIX(2, 2, &model) == 0); \
    assert(liblrhsmm_rs_model_builder_resize_##SUFFIX(model, 4, 3) == 0); \
    assert(liblrhsmm_rs_model_builder_resize_##SUFFIX(model, 1, (size_t)INT32_MAX + 1) == 3); \
    assert(liblrhsmm_rs_model_builder_layout_##SUFFIX(model, &model_layout) == 0); \
    assert(model_layout.streams == 4 && model_layout.durations == 3); \
    assert(liblrhsmm_rs_model_builder_resize_##SUFFIX(model, 0, 0) == 0); \
    assert(liblrhsmm_rs_model_builder_release_##SUFFIX(&model) == 0); \
    assert(liblrhsmm_rs_model_builder_resize_##SUFFIX(NULL, 1, 1) == 1); \
} while (0)

int main(void) {
    VERIFY(f32, struct LiblrhsmmRsStreamBuilderF32, struct LiblrhsmmRsModelBuilderF32, struct LiblrhsmmRsStreamLayoutF32);
    VERIFY(f64, struct LiblrhsmmRsStreamBuilderF64, struct LiblrhsmmRsModelBuilderF64, struct LiblrhsmmRsStreamLayoutF64);
    puts("Actual C caller: four builder resize exports passed.");
    return 0;
}
