#include <stdio.h>
#include "common.h"
#include "estimate.h"

int main(void) {
    int counts[] = {2, 2}, mixtures[] = {1, 1};
    int dimensions[] = {2, 1};
    lrh_model *model = lrh_create_model(2, counts, 2, mixtures, dimensions);
    lrh_observ *observation = lrh_create_observ(2, 6, dimensions);
    lrh_seg *segmentation = lrh_create_seg(2, 3);
    for (int t = 0; t < 6; t++)
        for (int l = 0; l < 2; l++)
            for (int d = 0; d < dimensions[l]; d++)
                lrh_obm(observation, t, d, l) = (t - 2) * 0.25f + d + l * 3.0f;
    int times[] = {2, 2, 6};
    for (int s = 0; s < 3; s++) {
        segmentation->time[s] = times[s];
        segmentation->durstate[s] = s % 2;
        segmentation->outstate[0][s] = s % 2;
        segmentation->outstate[1][s] = 1 - s % 2;
    }
    lrh_model_stat *stat = lrh_model_stat_from_model(model);
    lrh_collect_init(stat, observation, segmentation);
    lrh_collect_init(stat, observation, segmentation);
    for (int s = 0; s < 2; s++)
        printf("duration %d %.9g %.9g %.9g\n", s, stat->durations[s]->valsum, stat->durations[s]->sqrsum, stat->durations[s]->ocpsum);
    for (int l = 0; l < 2; l++)
        for (int s = 0; s < 2; s++) {
            lrh_gmm_stat *g = stat->streams[l]->gmms[s];
            printf("mixture %d %d %.9g %.9g", l, s, g->ocpsum, g->weightsum[0]);
            for (int d = 0; d < g->ndim; d++) printf(" %.9g %.9g", g->valsum[d], g->sqrsum[d]);
            puts("");
        }
    lrh_delete_model_stat(stat); lrh_delete_model(model);
    lrh_delete_observ(observation); lrh_delete_seg(segmentation);
    return 0;
}
