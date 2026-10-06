#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "model.h"
#include "data.h"
#include "inference.h"

static void add(lrh_seg *seg, int source, int delta, float probability) {
    int size = 0;
    while (seg->djump_out[source][size++] != 1) {}
    seg->djump_out[source] = realloc(seg->djump_out[source], (size + 1) * sizeof(int));
    seg->pjump_out[source] = realloc(seg->pjump_out[source], (size + 1) * sizeof(float));
    seg->djump_out[source][size] = 1;
    seg->pjump_out[source][size] = seg->pjump_out[source][size - 1];
    seg->djump_out[source][size - 1] = delta;
    seg->pjump_out[source][size - 1] = probability;
}
int main(void) {
    for (int scenario = 0; scenario < 4; scenario++) {
        lrh_model *model = lrh_create_model(0, NULL, 4, NULL, NULL);
        lrh_seg *seg = lrh_create_seg(0, 4);
        for (int state = 0; state < 4; state++) {
            model->durations[state]->mean = 2.5f + state * 0.25f;
            model->durations[state]->var = 0.75f + state * 0.25f;
            model->durations[state]->floor = state == 1 ? 2 : -1;
            model->durations[state]->ceil = 7;
            seg->time[state] = (state + 1) * 3;
            seg->durstate[state] = state;
        }
        if (scenario > 0) {
            add(seg, 0, 2, 0.2f);
            add(seg, 1, 0, 0.15f);
            add(seg, 2, -1, 0.1f);
            for (int state = 0; state < 3; state++) {
                int last = 0;
                while (seg->djump_out[state][last] != 1) last++;
                seg->pjump_out[state][last] = 0.8f;
            }
        }
        lrh_seg_buildjumps(seg);
        if (scenario == 3) model->durations[1]->ceil = 3;
        lrh_daem_temperature = scenario == 2 ? 0.5f : 1.0f;
        lrh_inference_duration_weight = 0.75f;
        lrh_inference_stprune = scenario == 2 ? 1 : 5;
        lrh_inference_duration_extra = 3;
        lrh_inference_duration_extra_factor = 0.5f;
        float emissions[48];
        for (int frame = 0; frame < 12; frame++)
            for (int state = 0; state < 4; state++) {
                float residual = frame - state * 3.0f;
                emissions[frame * 4 + state] = -residual * residual * 0.125f;
                if (scenario == 3) {
                    const int desired[] = {0, 0, 2, 2, 1, 1, 1, 1, 2, 3, 3, 3};
                    emissions[frame * 4 + state] = state == desired[frame] ? 0.0f : -50.0f;
                }
            }
        float *begin = NULL;
        float *end = lrh_forward(model, seg, emissions, 12, &begin);
        int *path = lrh_viterbi(model, seg, emissions, 12, NULL);
        for (int index = 0; index < 48; index++)
            printf("score %d %d %.9g %.9g\n", scenario, index, begin[index], end[index]);
        for (int index = 0; path[index * 2] != -1; index++)
            printf("path %d %d %d %d\n", scenario, index, path[index * 2], path[index * 2 + 1]);
        free(begin); free(end); free(path);
        lrh_delete_seg(seg);
        lrh_delete_model(model);
    }
    return 0;
}
