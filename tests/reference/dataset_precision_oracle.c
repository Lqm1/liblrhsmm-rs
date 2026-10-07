#include <float.h>
#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "data.h"

int main(void) {
    int dimensions[] = {2, 1}, times[] = {1, 3, 3, 6, 8};
    lrh_observ *observation = lrh_create_observ(2, 8, dimensions);
    lrh_seg *segmentation = lrh_create_seg(2, 5);
    for (int stream = 0; stream < 2; stream++)
        for (int i = 0; i < 8 * dimensions[stream]; i++)
            observation->data[stream][i] = 1.0 + (i + stream + 1) * DBL_EPSILON;
    for (int state = 0; state < 5; state++) {
        segmentation->time[state] = times[state];
        segmentation->durstate[state] = 10 + state;
        segmentation->outstate[0][state] = 20 + state;
        segmentation->outstate[1][state] = 40 - state;
    }
    lrh_observ *observations[] = {observation};
    lrh_seg *segmentations[] = {segmentation};
    lrh_observset observation_set = {observations, 1};
    lrh_segset segmentation_set = {segmentations, 1};
    int units[] = {1, 2, 3, 5, 8};
    for (int u = 0; u < 5; u++) {
        lrh_dataset *dataset = lrh_regroup_data(&observation_set, &segmentation_set, units[u]);
        if (!dataset) return 2;
        printf("count %d %d\n", units[u], dataset->observset->nsample);
        for (int sample = 0; sample < dataset->observset->nsample; sample++) {
            lrh_observ *ob = dataset->observset->samples[sample];
            lrh_seg *sg = dataset->segset->samples[sample];
            for (int stream = 0; stream < 2; stream++)
                for (int i = 0; i < ob->nt * dimensions[stream]; i++)
                    printf("value %d %d %d %d %.17g\n", units[u], sample, stream, i, ob->data[stream][i]);
            for (int state = 0; state < sg->nseg; state++)
                printf("state %d %d %d %d %d %d %d\n", units[u], sample, state,
                    sg->time[state], sg->durstate[state], sg->outstate[0][state], sg->outstate[1][state]);
        }
        lrh_delete_observset(dataset->observset);
        lrh_delete_segset(dataset->segset);
        free(dataset);
    }
    lrh_delete_observ(observation);
    lrh_delete_seg(segmentation);
    return 0;
}
