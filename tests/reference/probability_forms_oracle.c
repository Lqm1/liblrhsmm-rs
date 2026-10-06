#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "model.h"
#include "data.h"
#include "inference.h"

int main(void) {
    lrh_duration *duration = lrh_create_duration();
    duration->mean = 4.25f; duration->var = 2.5f;
    duration->floor = 2; duration->ceil = 6;
    lrh_inference_duration_weight = 0.75f;
    for (int temperature = 1; temperature <= 2; temperature++) {
        lrh_daem_temperature = temperature * 0.5f;
        int counts[] = {1}, mixtures[] = {1}, dimensions[] = {1};
        lrh_model *holder = lrh_create_model(1, counts, 1, mixtures, dimensions);
        lrh_delete_duration(holder->durations[0]); holder->durations[0] = duration;
        duration->_tmp_term = LRH_UNINITIALIZED;
        lrh_model_precompute(holder);
        for (int frame = 0; frame <= 8; frame++)
            printf("duration %d %d %.9g %.9g\n", temperature, frame,
                lrh_duration_prob_lg(duration, frame), lrh_duration_prob(duration, frame));
        holder->durations[0] = NULL; lrh_delete_model(holder);
    }
    lrh_delete_duration(duration);
    int counts[] = {3, 3}, mixtures[] = {2, 1}, dimensions[] = {2, 1};
    lrh_model *model = lrh_create_model(2, counts, 3, mixtures, dimensions);
    for (int stream = 0; stream < 2; stream++) {
        model->streams[stream]->weight = stream == 0 ? 0.5f : 1.25f;
        for (int state = 0; state < 3; state++) {
            lrh_gmm *g = model->streams[stream]->gmms[state];
            for (int index = 0; index < g->nmix * g->ndim; index++) {
                g->mean[index] = state * 0.5f + index * 0.25f;
                g->var[index] = 0.75f + index * 0.25f;
            }
        }
    }
    lrh_observ *observation = lrh_create_observ(2, 12, dimensions);
    lrh_seg *segmentation = lrh_create_seg(2, 3);
    for (int state = 0; state < 3; state++) {
        segmentation->time[state] = (state + 1) * 4;
        segmentation->outstate[0][state] = state;
        segmentation->outstate[1][state] = 2 - state;
    }
    for (int time = 0; time < 12; time++)
        for (int stream = 0; stream < 2; stream++)
            for (int dimension = 0; dimension < dimensions[stream]; dimension++)
                lrh_obm(observation, time, dimension, stream) = time * 0.125f + dimension * 0.5f;
    lrh_daem_temperature = 0.75f; lrh_inference_stprune = 1;
    float *result = lrh_sample_outputprob(model, observation, segmentation);
    for (int index = 0; index < 36; index++) printf("emission %d %.9g\n", index, result[index]);
    free(result); lrh_delete_observ(observation); lrh_delete_seg(segmentation); lrh_delete_model(model);
    return 0;
}
