#include "fht.h"
#include <arm_neon.h>

/* ARM NEON implementation of Fast Hadamard Transform */

/* Helper for log_n = 1: size 2 */
static inline void helper_float_1(float *buf) {
    float u = buf[0];
    float v = buf[1];
    buf[0] = u + v;
    buf[1] = u - v;
}

/* Helper for log_n = 2: size 4 */
static inline void helper_float_2(float *buf) {
    /* Load [a, b, c, d] */
    float a = buf[0], b = buf[1], c = buf[2], d = buf[3];

    /* Stage 1: pairs with stride 1 */
    float t0 = a + b;
    float t1 = a - b;
    float t2 = c + d;
    float t3 = c - d;

    /* Stage 2: pairs with stride 2 */
    buf[0] = t0 + t2;
    buf[1] = t1 + t3;
    buf[2] = t0 - t2;
    buf[3] = t1 - t3;
}

/* Helper for log_n = 3: size 8 using NEON */
static inline void helper_float_3(float *buf) {
    /* Load data */
    float32x4_t v0 = vld1q_f32(buf);      /* [0,1,2,3] */
    float32x4_t v1 = vld1q_f32(buf + 4);  /* [4,5,6,7] */

    /* Stage 1: butterfly stride 1 */
    /* We need to process pairs: (0,1), (2,3), (4,5), (6,7) */
    float32x2_t lo0 = vget_low_f32(v0);   /* [0,1] */
    float32x2_t hi0 = vget_high_f32(v0);  /* [2,3] */
    float32x2_t lo1 = vget_low_f32(v1);   /* [4,5] */
    float32x2_t hi1 = vget_high_f32(v1);  /* [6,7] */

    /* Transpose pairs to separate even/odd */
    float32x2x2_t t0 = vtrn_f32(lo0, lo0);  /* [[0,0],[1,1]] */
    float32x2x2_t t1 = vtrn_f32(hi0, hi0);  /* [[2,2],[3,3]] */
    float32x2x2_t t2 = vtrn_f32(lo1, lo1);  /* [[4,4],[5,5]] */
    float32x2x2_t t3 = vtrn_f32(hi1, hi1);  /* [[6,6],[7,7]] */

    float32x2_t s0 = vadd_f32(t0.val[0], t0.val[1]);  /* [0+1, 0+1] -> use [0] */
    float32x2_t d0 = vsub_f32(t0.val[0], t0.val[1]);  /* [0-1, 0-1] -> use [0] */
    float32x2_t s1 = vadd_f32(t1.val[0], t1.val[1]);
    float32x2_t d1 = vsub_f32(t1.val[0], t1.val[1]);
    float32x2_t s2 = vadd_f32(t2.val[0], t2.val[1]);
    float32x2_t d2 = vsub_f32(t2.val[0], t2.val[1]);
    float32x2_t s3 = vadd_f32(t3.val[0], t3.val[1]);
    float32x2_t d3 = vsub_f32(t3.val[0], t3.val[1]);

    /* Reconstruct as [s0[0], d0[0], s1[0], d1[0]] and [s2[0], d2[0], s3[0], d3[0]] */
    v0 = vcombine_f32(vzip_f32(s0, d0).val[0], vzip_f32(s1, d1).val[0]);
    v1 = vcombine_f32(vzip_f32(s2, d2).val[0], vzip_f32(s3, d3).val[0]);

    /* Stage 2: butterfly stride 2 */
    lo0 = vget_low_f32(v0);
    hi0 = vget_high_f32(v0);
    lo1 = vget_low_f32(v1);
    hi1 = vget_high_f32(v1);

    v0 = vcombine_f32(vadd_f32(lo0, hi0), vsub_f32(lo0, hi0));
    v1 = vcombine_f32(vadd_f32(lo1, hi1), vsub_f32(lo1, hi1));

    /* Stage 3: butterfly stride 4 */
    float32x4_t sum = vaddq_f32(v0, v1);
    float32x4_t diff = vsubq_f32(v0, v1);

    vst1q_f32(buf, sum);
    vst1q_f32(buf + 4, diff);
}

/* Generic recursive helper for larger sizes */
static void helper_float_recursive(float *buf, int log_n) {
    if (log_n <= 3) {
        if (log_n == 1) helper_float_1(buf);
        else if (log_n == 2) helper_float_2(buf);
        else if (log_n == 3) helper_float_3(buf);
        return;
    }

    int n = 1 << log_n;
    int half = n / 2;

    /* Recursively transform both halves */
    helper_float_recursive(buf, log_n - 1);
    helper_float_recursive(buf + half, log_n - 1);

    /* Combine: butterfly with stride = half */
    /* Use NEON for the final combining step */
    for (int i = 0; i < half; i += 4) {
        if (i + 4 <= half) {
            float32x4_t a = vld1q_f32(buf + i);
            float32x4_t b = vld1q_f32(buf + half + i);
            float32x4_t sum = vaddq_f32(a, b);
            float32x4_t diff = vsubq_f32(a, b);
            vst1q_f32(buf + i, sum);
            vst1q_f32(buf + half + i, diff);
        } else {
            /* Handle remaining elements */
            for (int j = i; j < half; j++) {
                float a = buf[j];
                float b = buf[half + j];
                buf[j] = a + b;
                buf[half + j] = a - b;
            }
        }
    }
}

/* Main entry point for float */
int fht_float(float *buf, int log_n) {
    if (log_n < 0 || log_n > 30) {
        return -1;
    }
    if (log_n == 0) {
        return 0;
    }

    helper_float_recursive(buf, log_n);
    return 0;
}

/* ========== Double precision versions ========== */

/* Helper for log_n = 1: size 2 */
static inline void helper_double_1(double *buf) {
    double u = buf[0];
    double v = buf[1];
    buf[0] = u + v;
    buf[1] = u - v;
}

/* Helper for log_n = 2: size 4 */
static inline void helper_double_2(double *buf) {
    /* NEON has 2 doubles per 128-bit vector */
    float64x2_t v0 = vld1q_f64(buf);
    float64x2_t v1 = vld1q_f64(buf + 2);

    /* Stage 1: butterfly within each pair */
    double t0 = vgetq_lane_f64(v0, 0);
    double t1 = vgetq_lane_f64(v0, 1);
    v0 = vsetq_lane_f64(t0 + t1, v0, 0);
    v0 = vsetq_lane_f64(t0 - t1, v0, 1);

    t0 = vgetq_lane_f64(v1, 0);
    t1 = vgetq_lane_f64(v1, 1);
    v1 = vsetq_lane_f64(t0 + t1, v1, 0);
    v1 = vsetq_lane_f64(t0 - t1, v1, 1);

    /* Stage 2: butterfly with stride 2 */
    float64x2_t sum = vaddq_f64(v0, v1);
    float64x2_t diff = vsubq_f64(v0, v1);

    vst1q_f64(buf, sum);
    vst1q_f64(buf + 2, diff);
}

/* Generic recursive helper for larger sizes */
static void helper_double_recursive(double *buf, int log_n) {
    if (log_n <= 2) {
        if (log_n == 1) helper_double_1(buf);
        else if (log_n == 2) helper_double_2(buf);
        return;
    }

    int n = 1 << log_n;
    int half = n / 2;

    /* Recursively transform both halves */
    helper_double_recursive(buf, log_n - 1);
    helper_double_recursive(buf + half, log_n - 1);

    /* Combine: butterfly with stride = half */
    for (int i = 0; i < half; i += 2) {
        if (i + 2 <= half) {
            float64x2_t a = vld1q_f64(buf + i);
            float64x2_t b = vld1q_f64(buf + half + i);
            float64x2_t sum = vaddq_f64(a, b);
            float64x2_t diff = vsubq_f64(a, b);
            vst1q_f64(buf + i, sum);
            vst1q_f64(buf + half + i, diff);
        } else {
            /* Handle remaining elements */
            for (int j = i; j < half; j++) {
                double a = buf[j];
                double b = buf[half + j];
                buf[j] = a + b;
                buf[half + j] = a - b;
            }
        }
    }
}

/* Main entry point for double */
int fht_double(double *buf, int log_n) {
    if (log_n < 0 || log_n > 30) {
        return -1;
    }
    if (log_n == 0) {
        return 0;
    }

    helper_double_recursive(buf, log_n);
    return 0;
}
