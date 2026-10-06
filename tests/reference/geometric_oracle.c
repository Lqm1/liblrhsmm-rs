#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "model.h"
#include "data.h"
#include "inference.h"
#include "math-funcs.h"

int main(void) {
    for (int states = 1; states <= 3; states += 2) {
        int frames = states == 1 ? 1 : 12;
        lrh_model *model = lrh_create_model(0, NULL, states, NULL, NULL);
        lrh_seg *segmentation = lrh_create_seg(0, states);
        for (int state = 0; state < states; state++) {
            model->durations[state]->mean = state == 0 ? 0.5f : state == 1 ? 2.75f : 5.0f;
            model->durations[state]->var = 1.0f;
            segmentation->durstate[state] = state;
        }
        float *emissions = calloc(frames * states, sizeof(float));
        for (int frame = 0; frame < frames; frame++)
            for (int state = 0; state < states; state++) {
                float residual = frame - state * 4.0f;
                emissions[frame * states + state] = -residual * residual * 0.25f;
            }
        for (int temperature = 1; temperature <= 2; temperature++) {
            lrh_daem_temperature = temperature * 0.5f;
            for (int band = 0; band < (states == 1 ? 1 : 2); band++) {
                lrh_inference_stprune_full_slope = band == 0 ? 1.0f : 0.34f;
                float *forward = lrh_forward_geometric(model, segmentation, emissions, frames);
                float *backward = lrh_backward_geometric(model, segmentation, emissions, frames);
                float *best = NULL;
                int *boundaries = lrh_viterbi_geometric(model, segmentation, emissions, frames, &best);
                for (int index = 0; index < frames * states; index++)
                    printf("scores %d %d %.9g %.9g %d %.9g %.9g %.9g\n", states, frames,
                        lrh_daem_temperature, lrh_inference_stprune_full_slope, index,
                        forward[index], backward[index], best[index]);
                for (int state = 0; state < states; state++)
                    printf("boundary %d %d %.9g %.9g %d %d\n", states, frames,
                        lrh_daem_temperature, lrh_inference_stprune_full_slope, state, boundaries[state]);
                for (int state = 0; state < states; state++)
                    printf("total %d %d %.9g %.9g %d %.9g %.9g %.9g\n", states, frames,
                        lrh_daem_temperature, lrh_inference_stprune_full_slope, state,
                        lrh_total(forward, backward, frames, states, state),
                        lrh_total_pseudo(forward, frames, states), lrh_total_backward_b(backward, emissions, states));
                float total = lrh_total_backward_b(backward, emissions, states);
                float *occupancy = lrh_stocp_geometric(model, segmentation, forward, backward, total, frames);
                lrh_mempool *pool = lrh_create_mempool(4096);
                lrh_pslice *transitions = lrh_sttran_geometric(model, segmentation, forward, backward, emissions, total, frames, pool);
                for (int index = 0; index < frames * states; index++)
                    printf("occupancy %d %d %.9g %.9g %d %.9g %d %.9g %.9g\n", states, frames,
                        lrh_daem_temperature, lrh_inference_stprune_full_slope, index, occupancy[index],
                        transitions[index].p != NULL, transitions[index].p ? transitions[index].p[0] : 0.0f,
                        transitions[index].p ? transitions[index].p[1] : 0.0f);
                free(occupancy); free(transitions); lrh_delete_mempool(pool);
                free(forward); free(backward); free(best); free(boundaries);
            }
        }
        free(emissions);
        lrh_delete_seg(segmentation);
        lrh_delete_model(model);
    }
    for (int index = 0; index < 64; index++) {
        float x = index * 0.25f;
        printf("math %.9g %.9g %.9g %.9g\n", x, lrh_lse(x * 0.125f, -x), lrh_negexp(-x), lrh_exp(-x));
    }
    return 0;
}
