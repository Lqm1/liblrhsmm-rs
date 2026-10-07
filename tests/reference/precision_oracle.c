/* Exercise unmodified upstream model and density functions in both FP modes. */
#include <stdio.h>
#include "common.h"
#include "model.h"
#include "inference.h"

int main(void) {
    lrh_precompute_duration = 0;
    lrh_duration *duration = lrh_create_duration();
    duration->mean = 4.25;
    duration->var = 2.5;
    duration->floor = 2;
    duration->ceil = 6;
    lrh_inference_duration_weight = 0.75;
    const double temperatures[] = {0.5, 1.0, 1.75};
    for (int frame = 0; frame <= 8; ++frame) {
        for (int t = 0; t < 3; ++t) {
            lrh_daem_temperature = temperatures[t];
            printf("duration %d %d %.17g %.17g %.17g\n", frame, t,
                (double)lrh_duration_logp(duration, frame),
                (double)lrh_duration_prob_lg(duration, frame),
                lrh_duration_prob(duration, frame));
        }
    }
    lrh_gmm *gmm = lrh_create_gmm(1, 2);
    gmm->mean[0] = 1.25; gmm->mean[1] = -2.5;
    gmm->var[0] = 2.5; gmm->var[1] = 0.75;
    gmm->vfloor[0] = 0.1; gmm->vfloor[1] = 0.2;
    lrh_gmm_mixinc(gmm, 5, 0.2);
    for (int k = 0; k < gmm->nmix; ++k) {
        printf("mixture %d 0 %.17g", k, (double)gmm->weight[k]);
        for (int d = 0; d < 2; ++d) {
            int index = k * 2 + d;
            printf(" %.17g %.17g %.17g", (double)gmm->mean[index],
                (double)gmm->var[index], (double)gmm->vfloor[index]);
        }
        puts("");
    }
    lrh_gmm_precompute(gmm);
    for (int i = 0; i < 5; ++i) {
        FP_TYPE observation[] = {i * 0.3 - 0.5, i * -0.2 + 0.75};
        for (int k = 0; k < gmm->nmix; ++k) {
            printf("component %d %d %.17g %.17g\n", i, k,
                (double)lrh_gmm_mixprob_lg(gmm, observation, k),
                lrh_gmm_mixprob(gmm, observation, k));
        }
        for (int t = 0; t < 3; ++t) {
            lrh_daem_temperature = temperatures[t];
            printf("output %d %d %.17g %.17g\n", i, t,
                (double)lrh_gmm_outputprob_lg(gmm, observation),
                lrh_gmm_outputprob(gmm, observation));
        }
    }
    lrh_delete_duration(duration);
    lrh_delete_gmm(gmm);
    return 0;
}
