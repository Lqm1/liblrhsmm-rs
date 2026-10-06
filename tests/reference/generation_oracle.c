#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "generate.h"
#include "math-funcs.h"

static unsigned position;
int rand(void) {
    const int values[] = {RAND_MAX / 8, RAND_MAX / 4, RAND_MAX / 2,
        RAND_MAX / 4 * 3, RAND_MAX / 8 * 7, RAND_MAX};
    return values[position++ % 6];
}
int main(void) {
    for (int index = 0; index < 6; index++) {
        position = index;
        float value = lrh_gaussian_random(-0.75f, 1.25f);
        printf("gaussian %d %.9g\n", index, value);
    }
    float weights[] = {0.25f, 0.5f, 0.1f};
    for (int index = 0; index < 6; index++) {
        position = index;
        printf("choice %d %d\n", index, lrh_random_choose(weights, 3));
    }
    for (int scenario = 0; scenario < 2; scenario++) {
        int counts[] = {2, 2}, mixtures[] = {2, 1}, dimensions[] = {2, 1};
        lrh_model *model = lrh_create_model(2, counts, 2, mixtures, dimensions);
        for (int state = 0; state < 2; state++) {
            model->durations[state]->mean = scenario == 0 ? 2.5f - state : 0.0f;
            model->durations[state]->var = scenario == 0 ? 0.75f - state * 0.5f : 0.0f;
            model->durations[state]->floor = 9; /* Constraints are unused by generation. */
        }
        for (int stream = 0; stream < 2; stream++)
            for (int state = 0; state < 2; state++) {
                lrh_gmm *g = model->streams[stream]->gmms[state];
                if (stream == 0) { g->weight[0] = 0.25f; g->weight[1] = 0.75f; }
                for (int index = 0; index < g->nmix * g->ndim; index++) {
                    g->mean[index] = state + index * 0.5f + stream * 2.0f;
                    g->var[index] = scenario == 0 ? 0.25f + index * 0.25f : 0.0f;
                }
            }
        lrh_seg *sequence = lrh_create_seg(2, 2);
        for (int state = 0; state < 2; state++) {
            sequence->durstate[state] = state;
            sequence->outstate[0][state] = state; sequence->outstate[1][state] = 1 - state;
        }
        position = 0;
        lrh_observ *observation = lrh_generate(model, sequence);
        printf("boundaries %d %d %d\n", scenario, sequence->time[0], sequence->time[1]);
        for (int time = 0; time < observation->nt; time++) {
            printf("frame %d %d", scenario, time);
            for (int stream = 0; stream < 2; stream++)
                for (int dimension = 0; dimension < dimensions[stream]; dimension++)
                    printf(" %.9g", lrh_obm(observation, time, dimension, stream));
            puts("");
        }
        printf("draws %d %u\n", scenario, position);
        lrh_delete_observ(observation); lrh_delete_seg(sequence); lrh_delete_model(model);
    }
    return 0;
}
