#include <stdio.h>
#include "common.h"
#include "model.h"
#include "mempool.h"

int main(void) {
    int emissions[] = {1}, components[] = {3}, dimensions[] = {2};
    lrh_model *model = lrh_create_model(1, emissions, 1, components, dimensions);
    const double variances[] = {0.75, 1.25, 2.5, 3.75, 0.125, 0.5};
    for (int i = 0; i < 6; i++) model->streams[0]->gmms[0]->var[i] = variances[i];
    model->durations[0]->mean = 4.25;
    model->durations[0]->var = 2.5;
    model->durations[0]->floor = 2;
    model->durations[0]->ceil = 6;
    lrh_precompute_duration = 8;
    lrh_daem_temperature = 0.75;
    lrh_inference_duration_weight = 0.5;
    lrh_model_precompute(model);
    for (int k = 0; k < 3; k++)
        printf("gaussian %d %.17g\n", k, (double)model->streams[0]->gmms[0]->_tmp_term[k]);
    printf("duration %.17g\n", (double)model->durations[0]->_tmp_term);
    for (int i = 0; i < 8; i++)
        printf("cache %d %.17g\n", i, (double)model->durations[0]->_tmp_prep[i]);
    lrh_delete_model(model);

    lrh_mempool *pool = lrh_create_mempool(8);
    const size_t requests[] = {3, 6, 12, 0, 2};
    for (int step = 0; step < 5; step++) {
        lrh_mempool_calloc(pool, 1, requests[step]);
        int index = 0;
        for (lrh_mempool_page *page = pool->first; page; page = page->next)
            printf("page %d %d %zu %zu\n", step, index++, page->top, page->capacity);
    }
    lrh_delete_mempool(pool);
    return 0;
}
