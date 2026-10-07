/* Exercise the original duration-bound macro, including adjacent FP values. */
#include <stdio.h>
#include "common.h"
#include "model.h"
#include "math-funcs.h"
#include "inference-helper.h"

static FP_TYPE adjacent(FP_TYPE value, double direction) {
    return sizeof(FP_TYPE) == sizeof(float)
        ? (FP_TYPE)nextafterf((float)value, (float)direction)
        : (FP_TYPE)nextafter((double)value, direction);
}

int main(void) {
    const int durations[] = {0, 3, 7}, extras[] = {0, 1, 30}, ceilings[] = {0, 1, 4, 12};
    FP_TYPE third = (FP_TYPE)(1.0 / 3.0);
    const FP_TYPE factors[] = {0, 0.25, 0.5, adjacent(third, -INFINITY), third, adjacent(third, INFINITY), 1};
    const FP_TYPE means[] = {0.5, adjacent(5, -INFINITY), 5, adjacent(5, INFINITY)};
    const FP_TYPE variances[] = {0.25, 1, 2.25};
    for (int d = 0; d < 3; d++) for (int e = 0; e < 3; e++)
    for (int f = 0; f < 7; f++) for (int m = 0; m < 4; m++)
    for (int v = 0; v < 3; v++) for (int c = 0; c < 4; c++) {
        lrh_duration distribution = {0};
        distribution.mean = means[m]; distribution.var = variances[v]; distribution.ceil = ceilings[c];
        lrh_duration *srcdr = &distribution;
        lrh_inference_duration_extra = extras[e];
        lrh_inference_duration_extra_factor = factors[f];
        int maxdur;
        calculate_maxdur(srcdr, durations[d]);
        printf("%d %d %.17g %.17g %.17g %d %d\n", durations[d], extras[e],
            (double)factors[f], (double)means[m], (double)variances[v], ceilings[c], maxdur);
    }
    return 0;
}
