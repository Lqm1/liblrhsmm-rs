#include <stdio.h>
#include "common.h"
#include "serial.h"

static bool reader(cmp_ctx_t *ctx, void *data, size_t size) { return fread(data, 1, size, (FILE *)ctx->buf) == size; }
static size_t writer(cmp_ctx_t *ctx, const void *data, size_t size) { return fwrite(data, 1, size, (FILE *)ctx->buf); }
static FILE *output(cmp_ctx_t *ctx, const char *directory, const char *name) {
    char path[4096];
    int length = snprintf(path, sizeof(path), "%s/%s", directory, name);
    if (length < 0 || (size_t)length >= sizeof(path)) return NULL;
    FILE *file = fopen(path, "wb");
    if (file) cmp_init(ctx, file, reader, writer);
    return file;
}
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    int counts[] = {2, 1}, mixtures[] = {3, 2}, dimensions[] = {2, 1};
    lrh_model *model = lrh_create_model(2, counts, 3, mixtures, dimensions);
    cmp_ctx_t context;
    FILE *file = output(&context, argv[1], "c-construction-model.hsmm");
    if (!file || !lrh_write_model(&context, model)) return 3;
    fclose(file);
    lrh_gmm *g = model->streams[0]->gmms[0];
    g->weight[0] = 0.1f; g->weight[1] = 0.2f; g->weight[2] = 0.7f;
    for (int i = 0; i < 6; i++) { g->mean[i] = i * 0.5f; g->var[i] = 1.0f + i * 0.25f; g->vfloor[i] = i * 0.125f; }
    lrh_gmm *copy = lrh_gmm_copy(g);
    g->mean[0] = 999.0f; g->vfloor[0] = 998.0f; g->weight[0] = 0.8f;
    file = output(&context, argv[1], "c-copy-gmm.bin");
    if (!file || !lrh_write_gmm(&context, copy)) return 4;
    fclose(file);
    lrh_seg *segment = lrh_create_seg(2, 3);
    for (int s = 0; s < 3; s++) {
        segment->time[s] = (s + 1) * 2; segment->durstate[s] = s + 3;
        segment->outstate[0][s] = s + 10; segment->outstate[1][s] = 20 - s;
    }
    for (int s = 0; s < 2; s++) {
        segment->djump_out[s] = realloc(segment->djump_out[s], 2 * sizeof(int));
        segment->pjump_out[s] = realloc(segment->pjump_out[s], 2 * sizeof(float));
        segment->djump_out[s][0] = s == 0 ? 2 : 0;
        segment->pjump_out[s][0] = s == 0 ? 0.25f : 0.1f;
        segment->djump_out[s][1] = 1; segment->pjump_out[s][1] = s == 0 ? 0.75f : 0.9f;
    }
    lrh_seg_buildjumps(segment);
    lrh_seg *segment_copy = lrh_seg_copy(segment);
    lrh_seg *segments[] = {segment, segment}; lrh_segset set = {segments, 2};
    lrh_segset *set_copy = lrh_segset_copy(&set);
    segment->time[0] = 999; segment->outstate[0][0] = 998; segment->pjump_out[0][0] = 0.5f;
    file = output(&context, argv[1], "c-copy-segmentation.bin");
    if (!file || !lrh_write_seg(&context, segment_copy)) return 5;
    fclose(file);
    file = output(&context, argv[1], "c-copy-segmentation-set.bin");
    if (!file || !lrh_write_segset(&context, set_copy)) return 6;
    fclose(file);
    lrh_delete_gmm(copy); lrh_delete_model(model);
    lrh_delete_seg(segment); lrh_delete_seg(segment_copy); lrh_delete_segset(set_copy);
    return 0;
}
