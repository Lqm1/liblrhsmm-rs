#include <stdio.h>
#include "common.h"
#include "estimate.h"

int main(void) {
    for (int scenario = 0; scenario < 5; scenario++) {
        lrh_duration *duration = lrh_create_duration();
        lrh_duration_stat *stat = lrh_duration_stat_from_duration(duration);
        duration->mean = 7.0f; duration->var = 3.0f;
        duration->vfloor = scenario == 2 ? 2.5f : 0.0f;
        stat->valsum = scenario == 3 ? 1.5f : 9.0f;
        stat->sqrsum = 29.0f;
        stat->ocpsum = scenario == 4 ? 1e-8f : 3.0f;
        lrh_duration_update(duration, stat, scenario == 3);
        printf("duration %d %.9g %.9g\n", scenario, duration->mean, duration->var);
        lrh_delete_duration_stat(stat); lrh_delete_duration(duration);
    }
    for (int scenario = 0; scenario < 4; scenario++) {
        int count = scenario == 0 ? 1 : 2;
        lrh_gmm *gmm = lrh_create_gmm(count, 2);
        lrh_gmm_stat *stat = lrh_gmm_stat_from_gmm(gmm);
        stat->ocpsum = scenario == 3 ? 1e-8f : 4.0f;
        for (int k = 0; k < count; k++) {
            stat->weightsum[k] = scenario == 2 && k == 1 ? 1e-8f : 2.0f + k;
            gmm->weight[k] = 0.5f;
            for (int i = 0; i < 2; i++) {
                int index = k * 2 + i;
                gmm->mean[index] = -7.0f; gmm->var[index] = 8.0f;
                gmm->vfloor[index] = i == 1 ? 0.75f : 0.0f;
                stat->valsum[index] = i == 0 ? -3.5f : 6.0f;
                stat->sqrsum[index] = i == 0 ? 9.25f : 12.0f;
            }
        }
        lrh_gmm_update(gmm, stat, 0);
        for (int k = 0; k < count; k++)
            printf("mixture %d %d %.9g %.9g %.9g %.9g %.9g\n", scenario, k,
                   gmm->weight[k], gmm->mean[k * 2], gmm->var[k * 2],
                   gmm->mean[k * 2 + 1], gmm->var[k * 2 + 1]);
        lrh_delete_gmm_stat(stat); lrh_delete_gmm(gmm);
    }
    return 0;
}
