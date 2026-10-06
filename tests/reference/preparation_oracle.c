#include <stdio.h>
#include "common.h"
#include "model.h"
#include "inference.h"

int main(void) {
    const int lengths[] = {0, 3, 8, 300};
    for (int cache = 0; cache < 4; cache++) {
        for (int temperature = 1; temperature <= 2; temperature++) {
            lrh_model *model = lrh_create_model(0, NULL, 1, NULL, NULL);
            lrh_duration *duration = model->durations[0];
            duration->mean = 4.25f; duration->var = 2.5f;
            duration->floor = 2; duration->ceil = 6;
            lrh_precompute_duration = lengths[cache];
            lrh_daem_temperature = temperature * 0.5f;
            lrh_inference_duration_weight = 0.75f;
            lrh_model_precompute(model);
            for (int frame = 0; frame <= 8; frame++)
                printf("D %d %d %d %.9g %.9g\n", lengths[cache], temperature,
                    frame, lrh_duration_prob_lg(duration, frame),
                    lrh_duration_prob(duration, frame));
            lrh_delete_model(model);
        }
    }
    return 0;
}
