#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SCMalloc(sz) malloc(sz)
#define SCFree(x) free(x)

#define DEBUG_VALIDATE_BUG_ON(cond) assert(!(cond))

#define MIN(x, y) ( ((x)<(y))?(x):(y) )

#define likely(expr) __builtin_expect(!!(expr), 1)
#define unlikely(expr) __builtin_expect(!!(expr), 0)
