#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "value.h"
#include "stack.h"
#include "str.h"

int main() {
    Stack s = stk_init();

    stk_push(&s, val_i32(87));
    stk_push(&s, val_i32(-32));
    stk_push(&s, val_f32(345.75));

    while(1) {
        OptionalValue opt = stk_pop(&s);
        if(opt.has) {
            Value val = opt.value;

            Str desc = val_to_str(val);
            puts(cstr(desc));
            str_free(desc);
        } else {
            break;
        }
    }

    return 0;
}
