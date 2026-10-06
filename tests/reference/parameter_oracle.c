#include <stdio.h>
#include "common.h"
#include "model.h"

int main(void) {
    lrh_duration *duration = lrh_create_duration();
    duration->mean = 4.25f;
    duration->var = 2.5f;
    duration->floor = 2;
    duration->ceil = 6;
    lrh_inference_duration_weight = 0.75f;
    for (int frame = 0; frame <= 8; frame++)
        printf("duration %d %.9g\n", frame, lrh_duration_logp(duration, frame));
    lrh_gmm *gmm = lrh_create_gmm(1, 2);
    gmm->mean[0] = 1.25f; gmm->mean[1] = -2.5f;
    gmm->var[0] = 2.5f; gmm->var[1] = 0.75f;
    gmm->vfloor[0] = 0.1f; gmm->vfloor[1] = 0.2f;
    lrh_gmm_mixinc(gmm, 5, 0.2f);
    for (int mixture = 0; mixture < gmm->nmix; mixture++) {
        printf("mixture %d %.9g", mixture, gmm->weight[mixture]);
        for (int dimension = 0; dimension < gmm->ndim; dimension++) {
            int index = mixture * gmm->ndim + dimension;
            printf(" %.9g %.9g %.9g", gmm->mean[index], gmm->var[index], gmm->vfloor[index]);
        }
        puts("");
    }
    lrh_delete_gmm(gmm);
    lrh_delete_duration(duration);
    return 0;
}
