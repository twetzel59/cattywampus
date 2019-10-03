#include "stack.h"

Stack stk_init(void) {
    return (Stack) { 0 };
}

bool stk_push(Stack *s, Value val) {
    if (s->height == STACK_CAP) {
        return false;
    }

    s->elements[s->height++] = val;

    return true;
}

OptionalValue stk_pop(Stack *s) {
    if (!s->height) {
        return (OptionalValue) NONE;
    }

    return (OptionalValue) SOME(s->elements[--s->height]);
}
