#include <stdio.h>
#include "common.h"
#include "data.h"
#include "serial.h"

static bool reader(cmp_ctx_t *ctx, void *data, size_t size) { return fread(data, 1, size, (FILE *)ctx->buf) == size; }
static size_t writer(cmp_ctx_t *ctx, const void *data, size_t size) { return fwrite(data, 1, size, (FILE *)ctx->buf); }
static FILE *output(cmp_ctx_t *ctx, const char *directory, const char *name) {
    char path[4096];
    if (snprintf(path, sizeof(path), "%s/%s", directory, name) >= sizeof(path)) return NULL;
    FILE *file = fopen(path, "wb");
    if (file) cmp_init(ctx, file, reader, writer);
    return file;
}
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    int dimensions[] = {2, 1}, times[] = {1, 3, 3, 6, 8};
    lrh_observ *observation = lrh_create_observ(2, 8, dimensions);
    lrh_seg *segmentation = lrh_create_seg(2, 5);
    for (int t = 0; t < 8; t++)
        for (int l = 0; l < 2; l++)
            for (int d = 0; d < dimensions[l]; d++) lrh_obm(observation, t, d, l) = t * 10.0f + l * 100.0f + d;
    for (int s = 0; s < 5; s++) {
        segmentation->time[s] = times[s]; segmentation->durstate[s] = s + 10;
        segmentation->outstate[0][s] = s + 20; segmentation->outstate[1][s] = 40 - s;
    }
    lrh_observ *observations[] = {observation}; lrh_seg *segmentations[] = {segmentation};
    lrh_observset observation_set = {observations, 1}; lrh_segset segmentation_set = {segmentations, 1};
    lrh_dataset *dataset = lrh_regroup_data(&observation_set, &segmentation_set, 2);
    if (!dataset || dataset->observset->nsample != 3) return 3;
    cmp_ctx_t context;
    for (int i = 0; i < 3; i++) {
        char name[80];
        snprintf(name, sizeof(name), "corrected-c-regroup-%d-observation.bin", i);
        FILE *file = output(&context, argv[1], name);
        if (!file || !lrh_write_observ(&context, dataset->observset->samples[i])) return 4;
        fclose(file);
        snprintf(name, sizeof(name), "corrected-c-regroup-%d-segmentation.bin", i);
        file = output(&context, argv[1], name);
        if (!file || !lrh_write_seg(&context, dataset->segset->samples[i])) return 5;
        fclose(file);
    }
    int path[] = {2, 0, 4, 3, 5, 3, 8, 1, -1};
    lrh_seg *resegmented = lrh_seg_shuffle(segmentation, path);
    FILE *file = output(&context, argv[1], "c-resegmentation.bin");
    if (!file || !lrh_write_seg(&context, resegmented)) return 6;
    fclose(file);
    lrh_delete_seg(resegmented);
    lrh_delete_observset(dataset->observset); lrh_delete_segset(dataset->segset); free(dataset);
    lrh_delete_observ(observation); lrh_delete_seg(segmentation);
    return 0;
}
