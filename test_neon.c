/* Simple test program for ARM NEON FHT implementation */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>
#include "fht.h"

#define MAX_LOG_N 10

/* Reference naive FHT for verification */
static void fht_naive_float(float *buf, int n) {
    if (n == 1) return;

    int half = n / 2;
    float *tmp = (float *)malloc(n * sizeof(float));

    /* Recursively transform both halves */
    fht_naive_float(buf, half);
    fht_naive_float(buf + half, half);

    /* Combine with butterfly */
    for (int i = 0; i < half; i++) {
        float a = buf[i];
        float b = buf[half + i];
        tmp[i] = a + b;
        tmp[half + i] = a - b;
    }

    for (int i = 0; i < n; i++) {
        buf[i] = tmp[i];
    }

    free(tmp);
}

static int test_correctness(int log_n) {
    int n = 1 << log_n;
    float *buf1 = (float *)malloc(n * sizeof(float));
    float *buf2 = (float *)malloc(n * sizeof(float));

    /* Initialize with random data */
    srand(42);
    for (int i = 0; i < n; i++) {
        buf1[i] = buf2[i] = (float)rand() / RAND_MAX * 2.0f - 1.0f;
    }

    /* Run both versions */
    fht_float(buf1, log_n);
    fht_naive_float(buf2, n);

    /* Compare results */
    float max_error = 0.0f;
    for (int i = 0; i < n; i++) {
        float error = fabsf(buf1[i] - buf2[i]);
        if (error > max_error) max_error = error;
    }

    int passed = (max_error < 1e-4f);
    printf("log_n=%2d (n=%6d): max_error=%.2e ... %s\n",
           log_n, n, max_error, passed ? "PASS" : "FAIL");

    free(buf1);
    free(buf2);

    return passed;
}

static void benchmark(int log_n, int iterations) {
    int n = 1 << log_n;
    float *buf = (float *)malloc(n * sizeof(float));

    /* Initialize data */
    for (int i = 0; i < n; i++) {
        buf[i] = (float)i;
    }

    /* Warmup */
    fht_float(buf, log_n);

    /* Benchmark */
    clock_t start = clock();
    for (int iter = 0; iter < iterations; iter++) {
        fht_float(buf, log_n);
    }
    clock_t end = clock();

    double time_sec = (double)(end - start) / CLOCKS_PER_SEC / iterations;
    double time_us = time_sec * 1e6;

    printf("log_n=%2d (n=%8d): %.3f us per transform\n", log_n, n, time_us);

    free(buf);
}

int main(int argc, char **argv) {
    (void)argc;  /* Unused */
    (void)argv;  /* Unused */
    printf("ARM NEON FHT Implementation Test\n");
    printf("=================================\n\n");

    printf("Correctness tests:\n");
    int all_passed = 1;
    for (int log_n = 1; log_n <= MAX_LOG_N; log_n++) {
        if (!test_correctness(log_n)) {
            all_passed = 0;
        }
    }

    if (all_passed) {
        printf("\nAll correctness tests PASSED!\n\n");
    } else {
        printf("\nSome tests FAILED!\n\n");
        return 1;
    }

    printf("Performance benchmarks:\n");
    benchmark(8, 10000);   /* 256 elements */
    benchmark(10, 10000);  /* 1024 elements */
    benchmark(12, 1000);   /* 4096 elements */
    benchmark(16, 100);    /* 65536 elements */
    benchmark(20, 10);     /* 1M elements */

    printf("\nAll tests completed successfully!\n");
    return 0;
}
