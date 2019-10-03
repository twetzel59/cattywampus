#ifndef VALUE_H_INCLUDED
#define VALUE_H_INCLUDED

#include <stdint.h>

#include "option.h"
#include "str.h"

// Tag for the type of values handled by the runtime.
typedef enum {
    VAL_I32,
    VAL_F32
} ValueTypeTag;

// Tag for the type of values handled by the runtime.
// Holds one distinct value of the type indicated by ``type``.
typedef struct {
    ValueTypeTag type;

    union {
        int32_t val_i32;
        float   val_f32;
    };
} Value;

DECLARE_OPTION(OptionalValue, Value);

inline Value val_i32(int32_t x) {
    return (Value) {
        .type = VAL_I32,
        .val_i32 = x
    };
}

inline Value val_f32(float x) {
    return (Value) {
        .type = VAL_F32,
        .val_f32 = x
    };
}

Str     val_to_str(Value val);

#endif // VALUE_H_INCLUDED
