#include "alloc.h"

#include <assert.h>
#include <stdlib.h>

static void oom(void);

void* mem_alloc(size_t alloc_size) {
    assert(alloc_size > 0);

    void* alloc = malloc(alloc_size);

    if (!alloc) {
        oom();
    }

    return alloc;
}

void mem_free(void* alloc) {
    free(alloc);
}

static void oom() {
    exit(EXIT_FAILURE);
}
