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
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    lrh_duration *duration = lrh_create_duration();
    duration->mean = 4.25f; duration->var = 1.5f;
    duration->floor = 2; duration->ceil = 9; duration->fixed_mean = 4; duration->vfloor = 0.25f;
    lrh_stream *stream = lrh_create_stream(2, 1, 2);
    stream->weight = 0.75f;
    lrh_delete_gmm(stream->gmms[1]); stream->gmms[1] = lrh_create_gmm(2, 2);
    for (int state = 0; state < 2; state++) {
        lrh_gmm *g = stream->gmms[state];
        for (int index = 0; index < g->nmix * g->ndim; index++) {
            g->mean[index] = index * 0.5f - state;
            g->var[index] = index * 0.25f + 0.5f;
            g->vfloor[index] = index * 0.125f;
        }
    }
    cmp_ctx_t context;
    FILE *file = open_file(&context, argv[1], "c-standalone-duration.bin", "wb");
    if (!file || !lrh_write_duration(&context, duration)) return 3; fclose(file);
    file = open_file(&context, argv[1], "c-standalone-gmm.bin", "wb");
    if (!file || !lrh_write_gmm(&context, stream->gmms[1])) return 4; fclose(file);
    file = open_file(&context, argv[1], "c-standalone-stream.bin", "wb");
    if (!file || !lrh_write_stream(&context, stream)) return 5; fclose(file);
    file = open_file(&context, argv[1], "c-standalone-duration.bin", "rb");
    lrh_duration *decoded_duration = file ? lrh_read_duration(&context) : NULL;
    if (!decoded_duration || decoded_duration->fixed_mean != 4 || decoded_duration->vfloor != 0.25f) return 6; fclose(file);
    file = open_file(&context, argv[1], "c-standalone-gmm.bin", "rb");
    lrh_gmm *decoded_gmm = file ? lrh_read_gmm(&context) : NULL;
    if (!decoded_gmm || decoded_gmm->nmix != 2 || decoded_gmm->vfloor[3] != 0.375f) return 7; fclose(file);
    file = open_file(&context, argv[1], "c-standalone-stream.bin", "rb");
    lrh_stream *decoded_stream = file ? lrh_read_stream(&context) : NULL;
    if (!decoded_stream || decoded_stream->ngmm != 2 || decoded_stream->weight != 0.75f || decoded_stream->gmms[1]->mean[3] != 0.5f) return 8; fclose(file);
    lrh_delete_duration(duration); lrh_delete_duration(decoded_duration);
    lrh_delete_gmm(decoded_gmm); lrh_delete_stream(stream); lrh_delete_stream(decoded_stream);
    return 0;
}
