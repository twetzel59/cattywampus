#ifndef ALLOC_H_INCLUDED
#define ALLOC_H_INCLUDED

#include <string.h>

void*   mem_alloc  (size_t alloc_size); // where alloc_size > 0
void    mem_free   (void* alloc);

#endif // ALLOC_H_INCLUDED
