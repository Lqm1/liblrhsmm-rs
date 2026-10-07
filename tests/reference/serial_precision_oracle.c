#include <stdio.h>
#include "common.h"
#include "serial.h"

static bool reader(cmp_ctx_t *ctx, void *data, size_t size) { return fread(data, 1, size, (FILE *)ctx->buf) == size; }
static size_t writer(cmp_ctx_t *ctx, const void *data, size_t size) { return fwrite(data, 1, size, (FILE *)ctx->buf); }
static FILE *open_file(cmp_ctx_t *ctx, const char *directory, const char *name, const char *mode) {
    char path[4096];
    if (snprintf(path, sizeof(path), "%s/%s", directory, name) >= sizeof(path)) return NULL;
    FILE *file = fopen(path, mode);
    if (file) cmp_init(ctx, file, reader, writer);
    return file;
}

#define ROUND_TRIP(type, name, read_fn, write_fn, delete_fn) do { \
    FILE *file = open_file(&context, argv[1], name, "rb"); \
    if (!file) return 3; \
    type *value = read_fn(&context); fclose(file); \
    if (!value) return 4; \
    file = open_file(&context, argv[2], name, "wb"); \
    if (!file || !write_fn(&context, value)) return 5; \
    fclose(file); delete_fn(value); \
} while (0)

int main(int argc, char **argv) {
    if (argc != 3) return 2;
    cmp_ctx_t context;
    FILE *file = open_file(&context, argv[1], "c-segmentation.bin", "rb");
    if (!file) return 3;
    lrh_seg *segment = lrh_read_seg(&context); fclose(file);
    if (!segment) return 4;
    printf("first jump %.17g\n", segment->pjump_out[0][0]);
    if (segment->pjump_out[0][0] != 0.25) return 9;
    lrh_delete_seg(segment);
    ROUND_TRIP(lrh_duration, "c-standalone-duration.bin", lrh_read_duration, lrh_write_duration, lrh_delete_duration);
    ROUND_TRIP(lrh_gmm, "c-standalone-gmm.bin", lrh_read_gmm, lrh_write_gmm, lrh_delete_gmm);
    ROUND_TRIP(lrh_stream, "c-standalone-stream.bin", lrh_read_stream, lrh_write_stream, lrh_delete_stream);
    ROUND_TRIP(lrh_model, "current-c-model.hsmm", lrh_read_model, lrh_write_model, lrh_delete_model);
    ROUND_TRIP(lrh_observ, "c-observation.bin", lrh_read_observ, lrh_write_observ, lrh_delete_observ);
    ROUND_TRIP(lrh_seg, "c-segmentation.bin", lrh_read_seg, lrh_write_seg, lrh_delete_seg);
    ROUND_TRIP(lrh_observset, "c-observation-set.bin", lrh_read_observset, lrh_write_observset, lrh_delete_observset);
    ROUND_TRIP(lrh_segset, "c-segmentation-set.bin", lrh_read_segset, lrh_write_segset, lrh_delete_segset);
    lrh_duration *duration = lrh_create_duration();
    duration->mean = 1.0 + 0x1p-25;
    duration->var = 1.0 + 3.0 * 0x1p-25;
    duration->vfloor = -0.0;
    file = open_file(&context, argv[2], "c-double-wire-duration.bin", "wb");
    if (!file || !lrh_write_duration(&context, duration)) return 5;
    fclose(file); lrh_delete_duration(duration);
    return 0;
}
