/* Isolate the exact division expressions used by the public math helpers. */
#include <stdio.h>

int log_index(FP_TYPE value) { return (int)(value / 0.001); }
int exponential_index(FP_TYPE value) { return (int)(value / 0.002); }
int log_difference_index(FP_TYPE a, FP_TYPE b) {
    FP_TYPE maximum = a > b ? a : b;
    FP_TYPE minimum = a > b ? b : a;
    return (int)((maximum - minimum) / 0.001);
}
int negative_index(FP_TYPE value) { return (int)((-value) / 0.002); }

int main(void) {
    volatile FP_TYPE log_value = 0.043;
    volatile FP_TYPE exponential_value = 0.086;
    printf("%d %d %d %d\n", log_index(log_value), exponential_index(exponential_value),
        log_difference_index(0, -log_value), negative_index(-exponential_value));
    return 0;
}
