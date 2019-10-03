#ifndef STACK_H_INCLUDED
#define STACK_H_INCLUDED

#include <stdbool.h>

#include "value.h"

#define STACK_CAP 4096

typedef struct {
    Value   elements[STACK_CAP];
    int64_t height;
} Stack;

Stack           stk_init    (void);                // create an empty stack

bool            stk_push    (Stack *s, Value val); // returns false if failed, i.e. full
OptionalValue   stk_pop     (Stack *s);            // returns value if one exists

#endif // STACK_H_INCLUDED
