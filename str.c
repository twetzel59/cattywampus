#include "str.h"

#include <stdarg.h>
#include <stdio.h>

#include "alloc.h"

extern inline char* str_data(Str string);
extern inline int64_t str_len(Str string);

static Str str_from_snprintf(const char * restrict format, ...);

Str str_alloc(int64_t len) {
    assert(len > 0);

    char* data_buffer = mem_alloc(len);
    data_buffer[len - 1] = '\0';

    return (Str) {
        .ref = (StrRef) {
            .bytes = data_buffer,
            .len = len
        }
    };
}

void str_free(Str string) {
    string.ref.len = 0;

    mem_free(string.ref.bytes);
    string.ref.bytes = NULL;
}

Str str_from_i64(int64_t x) {
    return str_from_snprintf("%ld", x);
}

Str str_from_f64(double x) {
    return str_from_snprintf("%f", x);
}

static Str str_from_snprintf(const char * restrict format, ...) {
    va_list args;

    va_start(args, format);
    int buf_len_required = vsnprintf(NULL, 0, format, args) + 1;
    va_end(args);

    Str target = str_alloc(buf_len_required);

    va_start(args, format);
    vsnprintf(cstr(target), slen(target), format, args);
    va_end(args);

    return target;
}
