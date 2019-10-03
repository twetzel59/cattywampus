#include "value.h"

extern inline Value val_i32(int32_t);
extern inline Value val_f32(float);

Str val_to_str(Value val) {
    switch(val.type) {
    case VAL_I32:
        return str_from_i64(val.val_i32); // widen to 64-bit
    case VAL_F32:
        return str_from_f64(val.val_f32); // widen to double
    default:
        assert(0);
    }
}
