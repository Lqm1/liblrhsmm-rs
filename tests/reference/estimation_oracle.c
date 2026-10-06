#include <stdio.h>
#include "common.h"
#include "estimate.h"

int main(void) {
    int counts[] = {3, 3}, mixtures[] = {2, 1}, dimensions[] = {2, 1};
    lrh_model *model = lrh_create_model(2, counts, 3, mixtures, dimensions);
    for (int l = 0; l < 2; l++)
        for (int s = 0; s < 3; s++) {
            lrh_gmm *g = model->streams[l]->gmms[s];
            for (int i = 0; i < g->nmix * g->ndim; i++) {
                g->mean[i] = s * 0.5f + i * 0.25f + l;
                g->var[i] = 0.75f + i * 0.25f;
            }
        }
    model->streams[1]->weight = 0.5f;
    for (int s = 0; s < 3; s++) { model->durations[s]->mean = 4.0f; model->durations[s]->var = 1.0f; }
    lrh_observ *observ = lrh_create_observ(2, 12, dimensions);
    lrh_seg *seg = lrh_create_seg(2, 3);
    for (int s = 0; s < 3; s++) {
        seg->time[s] = (s + 1) * 4; seg->durstate[s] = s;
        seg->outstate[0][s] = s; seg->outstate[1][s] = 2 - s;
    }
    for (int t = 0; t < 12; t++)
        for (int l = 0; l < 2; l++)
            for (int d = 0; d < dimensions[l]; d++) lrh_obm(observ, t, d, l) = t * 0.125f + d * 0.5f + l;
    lrh_inference_stprune = 1; lrh_inference_stprune_full_slope = 0.5f;
    for (int mode = 0; mode <
#ifdef FRACTIONAL_ORACLE
        4
#else
        2
#endif
        ; mode++)
        for (int temperature = 1; temperature <= 2; temperature++) {
            lrh_daem_temperature = temperature * 0.5f;
            /* Rebuild the duration cache for each independent temperature case. */
            for (int s = 0; s < 3; s++) model->durations[s]->_tmp_term = LRH_UNINITIALIZED;
            lrh_model_stat *stat = lrh_model_stat_from_model(model);
#ifdef FRACTIONAL_ORACLE
            const float radii[] = {0.25f, 0.5f, 1.75f, 2.5f};
            lrh_inference_stprune = radii[mode];
            float total = lrh_estimate(stat, model, observ, seg);
#else
            float total = mode == 0 ? lrh_estimate(stat, model, observ, seg) : lrh_estimate_geometric(stat, model, observ, seg);
#endif
            printf("total %d %d %.9g\n", mode, temperature, total);
            for (int s = 0; s < 3; s++)
                printf("duration %d %d %d %.9g %.9g %.9g\n", mode, temperature, s,
                    stat->durations[s]->valsum, stat->durations[s]->sqrsum, stat->durations[s]->ocpsum);
            for (int l = 0; l < 2; l++)
                for (int s = 0; s < 3; s++) {
                    lrh_gmm_stat *g = stat->streams[l]->gmms[s];
                    printf("mixture %d %d %d %d %.9g", mode, temperature, l, s, g->ocpsum);
                    for (int k = 0; k < g->nmix; k++) printf(" %.9g", g->weightsum[k]);
                    for (int i = 0; i < g->nmix * g->ndim; i++) printf(" %.9g %.9g", g->valsum[i], g->sqrsum[i]);
                    puts("");
                }
            lrh_delete_model_stat(stat);
        }
    lrh_delete_model(model); lrh_delete_observ(observ); lrh_delete_seg(seg);
    return 0;
}
