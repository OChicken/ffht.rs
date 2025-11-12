#include "fht.h"

// Include SIMD implementation
#ifdef __AVX__
#include "FFHT/fht_avx.c"
#elif defined(__aarch64__) || defined(__ARM_NEON)
#include "fht_neon.c"
#else
#include "FFHT/fht_sse.c"
#endif

// Define out-of-place functions here (after fast_copy is defined)
int fht_float_oop(float *in, float *out, int log_n) {
    fast_copy(out, in, sizeof(float) << log_n);
    return fht_float(out, log_n);
}

int fht_double_oop(double *in, double *out, int log_n) {
    fast_copy(out, in, sizeof(double) << log_n);
    return fht_double(out, log_n);
}
