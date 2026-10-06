#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "common.h"
#include "model.h"
#include "data.h"
#include "serial.h"

static bool reader(cmp_ctx_t *ctx, void *data, size_t count) {
    return fread(data, 1, count, (FILE *)ctx->buf) == count;
}
static size_t writer(cmp_ctx_t *ctx, const void *data, size_t count) {
    return fwrite(data, 1, count, (FILE *)ctx->buf);
}
int main(int argc, char **argv) {
    if (argc != 3) return 2;
    FILE *input = fopen(argv[1], "rb");
    if (!input) return 3;
    cmp_ctx_t context;
    cmp_init(&context, input, reader, writer);
    lrh_model *model = lrh_read_model(&context);
    long offset = ftell(input);
    fclose(input);
    if (!model) { fprintf(stderr, "C reader rejected model at offset %ld\n", offset); return 4; }
    FILE *output = fopen(argv[2], "wb");
    if (!output) return 5;
    cmp_init(&context, output, reader, writer);
    if (!lrh_write_model(&context, model)) return 6;
    fclose(output);
    printf("streams=%d durations=%d\n", model->nstream, model->nduration);
    lrh_delete_model(model);
    return 0;
}
