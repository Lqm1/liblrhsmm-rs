#include <stdio.h>
#include "common.h"
#include "model.h"
#include "data.h"
#include "inference.h"

int main(void) {
    const float observations[][2] = {{0.0f, 0.0f}, {1.25f, -2.5f}, {-3.0f, 4.0f}, {100.0f, -100.0f}};
    for (int mixtures = 1; mixtures <= 3; mixtures++) {
        lrh_gmm *gmm = lrh_create_gmm(mixtures, 2);
        for (int component = 0; component < mixtures; component++) {
            gmm->weight[component] = (component + 1.0f) / (mixtures * (mixtures + 1.0f) / 2.0f);
            gmm->mean[component * 2] = 1.25f + component;
            gmm->mean[component * 2 + 1] = -2.5f - component;
            gmm->var[component * 2] = 0.75f + component;
            gmm->var[component * 2 + 1] = 2.5f + component;
        }
        lrh_gmm_precompute(gmm);
        for (int observation = 0; observation < 4; observation++) {
            for (int temperature = 1; temperature <= 2; temperature++) {
                lrh_daem_temperature = temperature * 0.5f;
                printf("%d %d %.9g %.9g %.9g", mixtures, observation, lrh_daem_temperature,
                    lrh_gmm_outputprob_lg(gmm, (float *)observations[observation]),
                    lrh_gmm_outputprob(gmm, (float *)observations[observation]));
                for (int component = 0; component < mixtures; component++)
                    printf(" %.9g %.9g", lrh_gmm_mixprob_lg(gmm, (float *)observations[observation], component),
                        lrh_gmm_mixprob(gmm, (float *)observations[observation], component));
                puts("");
            }
        }
        lrh_delete_gmm(gmm);
    }
    return 0;
}
