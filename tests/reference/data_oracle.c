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
    int dimensions[] = {2, 1};
    lrh_observ *observation = lrh_create_observ(2, 3, dimensions);
    for (int stream = 0; stream < 2; stream++)
        for (int frame = 0; frame < 3; frame++)
            for (int dimension = 0; dimension < dimensions[stream]; dimension++)
                lrh_obm(observation, frame, dimension, stream) = (frame - 1) * 0.25 + dimension + stream * 10;
    lrh_seg *segmentation = lrh_create_seg(2, 3);
    for (int state = 0; state < 3; state++) {
        segmentation->time[state] = (state + 1) * 2;
        segmentation->durstate[state] = state;
        segmentation->outstate[0][state] = state;
        segmentation->outstate[1][state] = 2 - state;
    }
    segmentation->djump_out[0] = realloc(segmentation->djump_out[0], 2 * sizeof(int));
    segmentation->pjump_out[0] = realloc(segmentation->pjump_out[0], 2 * sizeof(FP_TYPE));
    segmentation->djump_out[0][0] = 2; segmentation->pjump_out[0][0] = 0.25;
    segmentation->djump_out[0][1] = 1; segmentation->pjump_out[0][1] = 0.75;
    segmentation->djump_out[2] = realloc(segmentation->djump_out[2], 3 * sizeof(int));
    segmentation->pjump_out[2] = realloc(segmentation->pjump_out[2], 3 * sizeof(FP_TYPE));
    segmentation->djump_out[2][0] = 0; segmentation->pjump_out[2][0] = 0.2;
    segmentation->djump_out[2][1] = -1; segmentation->pjump_out[2][1] = 0.1;
    segmentation->djump_out[2][2] = 1; segmentation->pjump_out[2][2] = 0.7;
    lrh_observ *observations[] = {observation, observation};
    lrh_seg *segments[] = {segmentation, segmentation};
    lrh_observset observation_set = { observations, 2 };
    lrh_segset segmentation_set = { segments, 2 };
    cmp_ctx_t context;
    FILE *file = open_file(&context, argv[1], "c-observation.bin", "wb");
    if (!file || !lrh_write_observ(&context, observation)) return 3; fclose(file);
    file = open_file(&context, argv[1], "c-segmentation.bin", "wb");
    if (!file || !lrh_write_seg(&context, segmentation)) return 4; fclose(file);
    file = open_file(&context, argv[1], "c-observation-set.bin", "wb");
    if (!file || !lrh_write_observset(&context, &observation_set)) return 5; fclose(file);
    file = open_file(&context, argv[1], "c-segmentation-set.bin", "wb");
    if (!file || !lrh_write_segset(&context, &segmentation_set)) return 6; fclose(file);
    file = open_file(&context, argv[1], "c-segmentation-set.bin", "rb");
    lrh_segset *decoded = file ? lrh_read_segset(&context) : NULL;
    if (!decoded || decoded->nsample != 2 || decoded->samples[1]->djump_out[2][1] != -1) return 7;
    fclose(file);
    lrh_delete_segset(decoded); lrh_delete_seg(segmentation); lrh_delete_observ(observation);
    puts("C observation and segmentation fixtures generated and nested segmentation read verified.");
    return 0;
}
