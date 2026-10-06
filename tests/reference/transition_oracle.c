#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "data.h"

static void add(lrh_seg *segmentation, int source, int delta, float probability) {
    int size = 0;
    while (segmentation->djump_out[source][size++] != 1) {}
    segmentation->djump_out[source] = realloc(segmentation->djump_out[source], (size + 1) * sizeof(int));
    segmentation->pjump_out[source] = realloc(segmentation->pjump_out[source], (size + 1) * sizeof(float));
    segmentation->djump_out[source][size] = 1;
    segmentation->pjump_out[source][size] = segmentation->pjump_out[source][size - 1];
    segmentation->djump_out[source][size - 1] = delta;
    segmentation->pjump_out[source][size - 1] = probability;
}

int main(void) {
    lrh_seg *segmentation = lrh_create_seg(0, 4);
    add(segmentation, 0, 2, 0.2f);
    add(segmentation, 1, 0, 0.3f);
    add(segmentation, 2, -2, 0.4f);
    add(segmentation, 3, -2, 0.5f);
    add(segmentation, 0, -100, 0.1f);
    add(segmentation, 3, 100, 0.1f);
    for (int state = 0; state < 4; state++) {
        int last = 0;
        while (segmentation->djump_out[state][last] != 1) last++;
        segmentation->pjump_out[state][last] = 0.6f + state * 0.1f;
    }
    for (int pass = 0; pass < 2; pass++) {
        lrh_seg_buildjumps(segmentation);
        for (int state = 0; state < 4; state++) {
            int index = 0;
            do {
                printf("%d %d %d %d %.9g\n", pass, state, index,
                    segmentation->djump_in[state][index], segmentation->pjump_in[state][index]);
            } while (segmentation->djump_in[state][index++] != -1);
        }
    }
    lrh_delete_seg(segmentation);
    return 0;
}
