#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "model.h"
#include "data.h"
#include "inference.h"

int main(void) {
    int ngmm[] = {3}, nmix[] = {2}, ndim[] = {2};
    lrh_model *model = lrh_create_model(1, ngmm, 3, nmix, ndim);
    lrh_delete_gmm(model->streams[0]->gmms[1]);
    lrh_delete_gmm(model->streams[0]->gmms[2]);
    model->streams[0]->gmms[1] = lrh_create_gmm(3, 2);
    model->streams[0]->gmms[2] = lrh_create_gmm(1, 2);
    for (int state = 0; state < 3; state++) {
        lrh_gmm *gmm = model->streams[0]->gmms[state];
        for (int index = 0; index < gmm->nmix * 2; index++) {
            gmm->mean[index] = state * 0.5f + index * 0.25f;
            gmm->var[index] = 0.75f + index * 0.25f;
        }
        for (int component = 0; component < gmm->nmix; component++)
            gmm->weight[component] = (component + 1.0f) / (gmm->nmix * (gmm->nmix + 1.0f) / 2.0f);
        model->durations[state]->mean = 4.0f;
        model->durations[state]->var = 1.0f;
    }
    lrh_observ *observ = lrh_create_observ(1, 8, ndim);
    lrh_seg *seg = lrh_create_seg(1, 3);
    seg->time[0] = 2; seg->time[1] = 5; seg->time[2] = 8;
    seg->outstate[0][0] = 0; seg->outstate[0][1] = 2; seg->outstate[0][2] = 1;
    float occupancy[24];
    for (int frame = 0; frame < 8; frame++) {
        for (int dimension = 0; dimension < 2; dimension++)
            lrh_obm(observ, frame, dimension, 0) = frame * 0.125f + dimension * 0.5f;
        for (int state = 0; state < 3; state++)
            occupancy[frame * 3 + state] = frame % 3 == 0 ? 0.0f : 0.1f + 0.1f * state + 0.05f * frame;
    }
    lrh_inference_stprune = 1;
    lrh_inference_stprune_full_slope = 0.5f;
    for (int temperature = 1; temperature <= 2; temperature++) {
        lrh_daem_temperature = temperature * 0.5f;
        for (int mode = 0; mode <
#ifdef FRACTIONAL_ORACLE
            4
#else
            2
#endif
            ; mode++) {
            lrh_mempool *pool = lrh_create_mempool(4096);
#ifdef FRACTIONAL_ORACLE
            const float radii[] = {0.25f, 0.5f, 1.75f, 2.5f};
            lrh_inference_stprune = radii[mode];
            lrh_pslice *result = lrh_mixocp(model, observ, seg, occupancy, 0, pool);
#else
            lrh_pslice *result = mode == 0 ? lrh_mixocp(model, observ, seg, occupancy, 0, pool)
                : lrh_mixocp_geometric(model, observ, seg, occupancy, 0, pool);
#endif
            for (int frame = 0; frame < 8; frame++) {
                for (int state = 0; state < 3; state++) {
                    lrh_pslice slice = result[frame * 3 + state];
                    int count = slice.p ? slice.upper : 0;
                    printf("%d %.9g %d %d %d", mode, lrh_daem_temperature, frame, state, count);
                    for (int component = 0; component < count; component++) printf(" %.9g", slice.p[component]);
                    puts("");
                }
            }
            free(result); lrh_delete_mempool(pool);
        }
    }
    lrh_delete_seg(seg); lrh_delete_observ(observ); lrh_delete_model(model);
    return 0;
}
