#ifndef _FHT_H_
#define _FHT_H_
#include <string.h>
#include <stdlib.h>
#if (defined(__x86_64__) || defined(__i386__))
#  include <x86intrin.h>
#elif (defined(__aarch64__) || defined(__ARM_NEON))
#  include <arm_neon.h>
#endif

#ifndef FAST_COPY_MEMCPY_THRESHOLD
#  define FAST_COPY_MEMCPY_THRESHOLD ((size_t)1ull << 20)
#endif

#ifdef FHT_HEADER_ONLY
#  define _STORAGE_ static inline
#else
#  define _STORAGE_
#endif

#ifdef __cplusplus
extern "C" {
#endif

// These functions all assume that the size of memory being copied is a power of 2.

#if _FEATURE_AVX512F
// If n is less than 64, defaults to memcpy. Otherwise, being a power of 2, we can just use unaligned stores and loads.
_STORAGE_ void *fast_copy(void *out, void *in, size_t n) {
    if(n >= FAST_COPY_MEMCPY_THRESHOLD) {
        return memcpy(out, in, n);
    }
    if(n < 64) {
        return memcpy(out, in, n);
    }
    n >>= 6;
    for(__m512 *ov = (__m512 *)out, *iv = (__m512 *)in; n--;) {
        _mm512_storeu_ps((float *)(ov++), _mm512_loadu_ps((float *)(iv++)));
    }
    return out;
}
#elif __AVX2__
// If n is less than 32, defaults to memcpy. Otherwise, being a power of 2, we can just use unaligned stores and loads.
_STORAGE_ void *fast_copy(void *out, void *in, size_t n) {
    if(n >= FAST_COPY_MEMCPY_THRESHOLD) {
        return memcpy(out, in, n);
    }
    if(n < 32) {
        return memcpy(out, in, n);
    }
    n >>= 5;
    for(__m256 *ov = (__m256 *)out, *iv = (__m256 *)in; n--;) {
        _mm256_storeu_ps((float *)(ov++), _mm256_loadu_ps((float *)(iv++)));
    }
    return out;
}
#elif __SSE2__
// If n is less than 16, defaults to memcpy. Otherwise, being a power of 2, we can just use unaligned stores and loads.
_STORAGE_ void *fast_copy(void *out, void *in, size_t n) {
    if(n >= FAST_COPY_MEMCPY_THRESHOLD) {
        return memcpy(out, in, n);
    }
    if(n < 16) {
        return memcpy(out, in, n);
    }
    n >>= 4;
    for(__m128 *ov = (__m128 *)out, *iv = (__m128 *)in; n--;) {
        _mm_storeu_ps((float *)(ov++), _mm_loadu_ps((float *)(iv++)));
    }
    return out;
}
#elif defined(__aarch64__) || defined(__ARM_NEON)
// ARM NEON: 128-bit vectors (16 bytes)
_STORAGE_ void *fast_copy(void *out, void *in, size_t n) {
    if(n >= FAST_COPY_MEMCPY_THRESHOLD) {
        return memcpy(out, in, n);
    }
    if(n < 16) {
        return memcpy(out, in, n);
    }
    n >>= 4;
    for(float *ov = (float *)out, *iv = (float *)in; n--; ov += 4, iv += 4) {
        vst1q_f32(ov, vld1q_f32(iv));
    }
    return out;
}
#else
_STORAGE_ void *fast_copy(void *out, void *in, size_t n) {
    return memcpy(out, in, n);
}
#endif

#ifdef FHT_HEADER_ONLY
#  undef _STORAGE_
#endif

int fht_float(float *buf, int log_n);
int fht_double(double *buf, int log_n);
int fht_float_oop(float *in, float *out, int log_n);
int fht_double_oop(double *in, double *out, int log_n);


#ifdef __cplusplus

} // extern "C"

static inline int fht(float *buf, int log_n) {
    return fht_float(buf, log_n);
}

static inline int fht(double *buf, int log_n) {
    return fht_double(buf, log_n);
}

static inline int fht(float *buf, float *out, int log_n) {
    return fht_float_oop(buf, out, log_n);
}

static inline int fht(double *buf, double *out, int log_n) {
    return fht_double_oop(buf, out, log_n);
}

#endif

#endif
