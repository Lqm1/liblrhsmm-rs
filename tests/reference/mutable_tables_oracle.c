/* Mutate public tables while compiling the library sources unchanged. */
#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "math-funcs.h"
#include "estimate.h"

int main(void) {
    for (int mode = 0; mode < 2; mode++) {
        if (mode) for (int i = 0; i < 7000; i++) {
            lrh_lse_table[i] = lrh_lse_table[i] + 0.015625;
            lrh_exp_table[i] = lrh_exp_table[i] * 0.875;
        }
        int counts[] = {3, 3}, mixtures[] = {2, 1}, dimensions[] = {2, 1};
        lrh_model *model = lrh_create_model(2, counts, 3, mixtures, dimensions);
        for (int l = 0; l < 2; l++) for (int s = 0; s < 3; s++) {
            lrh_gmm *g = model->streams[l]->gmms[s];
            for (int i = 0; i < g->nmix * g->ndim; i++) {
                g->mean[i] = s * 0.5 + i * 0.25 + l;
                g->var[i] = 0.75 + i * 0.25;
            }
        }
        model->streams[1]->weight = 0.5;
        for (int s = 0; s < 3; s++) {
            model->durations[s]->mean = 4.0; model->durations[s]->var = 1.0;
        }
        lrh_observ *observ = lrh_create_observ(2, 12, dimensions);
        lrh_seg *seg = lrh_create_seg(2, 3);
        for (int s = 0; s < 3; s++) {
            seg->time[s] = (s + 1) * 4; seg->durstate[s] = s;
            seg->outstate[0][s] = s; seg->outstate[1][s] = 2 - s;
        }
        for (int t = 0; t < 12; t++) for (int l = 0; l < 2; l++)
            for (int d = 0; d < dimensions[l]; d++)
                lrh_obm(observ, t, d, l) = t * 0.125 + d * 0.5 + l;
        lrh_inference_stprune_full_slope = 0.5;
        lrh_daem_temperature = 1.0;
        printf("helper %d %.17g %.17g\n", mode,
            (double)lrh_lse(0, 0), (double)lrh_negexp(-0.125));
        FP_TYPE *emissions = lrh_sample_outputprob_lg_full(model, observ, seg);
        FP_TYPE *forward = lrh_forward_geometric(model, seg, emissions, 12);
        FP_TYPE *backward = lrh_backward_geometric(model, seg, emissions, 12);
        for (int i = 0; i < 36; i++)
            printf("scores %d %d %.17g %.17g\n", mode, i,
                (double)forward[i], (double)backward[i]);
        lrh_model_stat *stat = lrh_model_stat_from_model(model);
        printf("total %d %.17g\n", mode,
            (double)lrh_estimate_geometric(stat, model, observ, seg));
        for (int s = 0; s < 3; s++)
            printf("duration %d %d %.17g %.17g %.17g\n", mode, s,
                (double)stat->durations[s]->valsum, (double)stat->durations[s]->sqrsum,
                (double)stat->durations[s]->ocpsum);
        for (int l = 0; l < 2; l++) for (int s = 0; s < 3; s++) {
            lrh_gmm_stat *g = stat->streams[l]->gmms[s];
            printf("mixture %d %d %d %.17g", mode, l, s, (double)g->ocpsum);
            for (int k = 0; k < g->nmix; k++) printf(" %.17g", (double)g->weightsum[k]);
            for (int i = 0; i < g->nmix * g->ndim; i++)
                printf(" %.17g %.17g", (double)g->valsum[i], (double)g->sqrsum[i]);
            puts("");
        }
        free(emissions); free(forward); free(backward);
        lrh_delete_model_stat(stat); lrh_delete_model(model);
        lrh_delete_observ(observ); lrh_delete_seg(seg);
    }
    return 0;
}
