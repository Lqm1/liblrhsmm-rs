#include <stdio.h>
#include <string.h>
#include "mempool.h"

struct request { int zeroed; size_t number; size_t size; };
static const struct request requests[] = {
    {0, 1, 0}, {0, 1, 3}, {1, 1, 5}, {0, 1, 1},
    {1, 3, 4}, {1, 0, 17}, {0, 1, 2}, {1, 2, 3}
};

int main(void) {
    for (int scenario = 0; scenario < 2; scenario++) {
        lrh_mempool *pool = lrh_create_mempool(scenario == 0 ? 8 : 0);
        unsigned char *allocations[8];
        size_t lengths[8];
        for (size_t i = 0; i < 8; i++) {
            const struct request *request = &requests[i];
            size_t length = request->number * request->size;
            unsigned char *data = request->zeroed
                ? lrh_mempool_calloc(pool, request->number, request->size)
                : lrh_mempool_malloc(pool, length);
            int zero = 1;
            if (request->zeroed)
                for (size_t j = 0; j < length; j++)
                    if (data[j] != 0) zero = 0;
            size_t offset = length == 0 && pool->last->capacity == 0
                ? 0 : (size_t)(data - (unsigned char *)pool->last->data);
            printf("A %d %zu %zu %zu %zu %zu %zu %d\n", scenario, i,
                   pool->npage, pool->totalsize, pool->last->top,
                   offset, length, zero);
            memset(data, (int)i + 1, length);
            allocations[i] = data;
            lengths[i] = length;
        }
        for (size_t i = 0; i < 8; i++) {
            size_t checksum = 0;
            for (size_t j = 0; j < lengths[i]; j++) checksum += allocations[i][j];
            printf("V %d %zu %zu\n", scenario, i, checksum);
        }
        lrh_delete_mempool(pool);
    }
    return 0;
}
