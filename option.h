#ifndef OPTION_H_INCLUDED
#define OPTION_H_INCLUDED

#include <assert.h>
#include <stdbool.h>

#define DECLARE_OPTION(NAME, T) \
    typedef struct {    \
        T value;        \
        bool has;       \
    } NAME

#define SOME(x) { (x), true }
#define NONE    { 0 }

#endif // OPTION_H_INCLUDED
