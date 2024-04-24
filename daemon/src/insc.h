/* 
 *  Additional module for 'daemon' intrinsics functions and datatypes
 * */

/* Static global application timer */
#include <stdint.h>

static volatile union {
    uint64_t bits;
} GLOBAL_TIMER;
