#ifndef STRING_H_INCLUDED
#define STRING_H_INCLUDED

#include <assert.h>
#include <stdint.h>

// A UTF-8 string that knows its length.
// *NEVER* free or reallocate directly.
//
// The len field is signed to guard against
// overflow bugs, particularily when iterating
// backwards or over substrings.
//
// While the StrRef is valid:
// bytes != NULL
// bytes[len - 1] == '\0'
// len > 0
//
// If these conditions fail, the StrRef belongs
// to a deallocated Str, which uses this state
// defensively to represent deleted boxed strings.
typedef struct {
    char*   bytes;
    int64_t len;
} StrRef;

// An owned string that can be freed
// at any time by the owner.
//
// *NEVER* semantically copy this type,
// only move it.
//
// It should be used as a unique pointer.
//
// The ref field may be read, but should
// and must only be written to when the
// StrRef is unique (i.e. not borrowed)!
typedef struct {
    StrRef ref;
} Str;

inline char* str_data(Str string) {
    assert(string.ref.bytes);
    assert(string.ref.len > 0);

    return string.ref.bytes;
}

#define cstr str_data

inline int64_t str_len(Str string) {
    assert(string.ref.len > 0);

    return string.ref.len;
}

#define slen str_len

Str     str_alloc       (int64_t len); // where len > 0
void    str_free        (Str string);

Str     str_from_i64    (int64_t x);
Str     str_from_f64    (double x);

#endif // STRING_H_INCLUDED
