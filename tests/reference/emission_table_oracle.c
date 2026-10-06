#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "model.h"
#include "data.h"
#include "inference.h"

int main(void) {
    int ngmm[] = {3, 3}, nmix[] = {2, 1}, ndim[] = {2, 1};
    lrh_model *model = lrh_create_model(2, ngmm, 3, nmix, ndim);
    for (int stream = 0; stream < 2; stream++) {
        model->streams[stream]->weight = stream == 0 ? 0.5f : 1.25f;
        for (int state = 0; state < 3; state++) {
            lrh_gmm *gmm = model->streams[stream]->gmms[state];
            for (int index = 0; index < gmm->nmix * gmm->ndim; index++) {
                gmm->mean[index] = state * 0.5f + index * 0.25f;
                gmm->var[index] = 0.75f + index * 0.25f;
            }
        }
    }
    for (int state = 0; state < 3; state++) {
        model->durations[state]->mean = 4.0f;
        model->durations[state]->var = 1.0f;
    }
    lrh_observ *observation = lrh_create_observ(2, 12, ndim);
    lrh_seg *segmentation = lrh_create_seg(2, 3);
    for (int state = 0; state < 3; state++) {
        segmentation->time[state] = (state + 1) * 4;
        segmentation->durstate[state] = state;
        segmentation->outstate[0][state] = state;
        segmentation->outstate[1][state] = 2 - state;
    }
    for (int frame = 0; frame < 12; frame++)
        for (int stream = 0; stream < 2; stream++)
            for (int dimension = 0; dimension < ndim[stream]; dimension++)
                lrh_obm(observation, frame, dimension, stream) = frame * 0.125f + dimension * 0.5f;
    lrh_daem_temperature = 0.75f;
    lrh_inference_stprune = 1;
    lrh_inference_stprune_full_slope = 0.34f;
    float *tables[] = {
        lrh_sample_outputprob_lg(model, observation, segmentation),
        lrh_sample_outputprob_lg_full(model, observation, segmentation)
    };
    for (int table = 0; table < 2; table++) {
        for (int index = 0; index < 36; index++) printf("%d %d %.9g\n", table, index, tables[table][index]);
        free(tables[table]);
    }
    lrh_delete_seg(segmentation);
    lrh_delete_observ(observation);
    lrh_delete_model(model);
    return 0;
}
