#include "fht.h"
#include <string.h>

// Fast copy implementation with SIMD optimizations
// Include architecture-specific headers
#if (defined(__x86_64__) || defined(__i386__))
#  include <x86intrin.h>
#elif (defined(__aarch64__) || defined(__ARM_NEON))
#  include <arm_neon.h>
#endif

#define FAST_COPY_MEMCPY_THRESHOLD ((size_t)1ull << 20)

void *fast_copy(void *out, void *in, size_t n) {
    // For large copies, use memcpy
    if (n >= FAST_COPY_MEMCPY_THRESHOLD) {
        return memcpy(out, in, n);
    }

#if defined(__AVX2__)
    // AVX2: 256-bit vectors (32 bytes)
    if (n >= 32) {
        size_t count = n >> 5;
        __m256 *ov = (__m256 *)out;
        __m256 *iv = (__m256 *)in;
        for (; count--; ) {
            _mm256_storeu_ps((float *)(ov++), _mm256_loadu_ps((float *)(iv++)));
        }
        return out;
    }
#endif

#if defined(__SSE2__) || defined(__x86_64__)
    // SSE2: 128-bit vectors (16 bytes) - enabled by default on x86_64
    if (n >= 16) {
        size_t count = n >> 4;
        __m128 *ov = (__m128 *)out;
        __m128 *iv = (__m128 *)in;
        for (; count--; ) {
            _mm_storeu_ps((float *)(ov++), _mm_loadu_ps((float *)(iv++)));
        }
        return out;
    }
#endif

#if defined(__aarch64__) || defined(__ARM_NEON)
    // ARM NEON: 128-bit vectors (16 bytes)
    if (n >= 16) {
        size_t count = n >> 4;
        float *ov = (float *)out;
        const float *iv = (const float *)in;
        for (; count--; ov += 4, iv += 4) {
            vst1q_f32(ov, vld1q_f32(iv));
        }
        return out;
    }
#endif

    // Fallback for small sizes or unsupported architectures
    return memcpy(out, in, n);
}

// Include SIMD implementation
#ifdef __AVX__
#include "fht_avx.c"
#elif defined(__aarch64__) || defined(__ARM_NEON)
#include "fht_neon.c"
#else
#include "fht_sse.c"
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
