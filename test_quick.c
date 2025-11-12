#include "fht.h"
#include <stdio.h>

static int test_defines(void) {
    printf("%s\n", __func__);

    #ifdef __AVX__
    printf("__AVX__ is defined\n");
    #endif

    #ifdef __AVX2__
    printf("__AVX2__ is defined\n");
    #endif

    #ifdef __SSE2__
    printf("__SSE2__ is defined\n");
    #endif

    #ifdef __aarch64__
    printf("__aarch64__ is defined\n");
    #endif

    #ifdef __ARM_NEON
    printf("__ARM_NEON is defined\n");
    #endif
    return 0;
}

static int test_fast_copy(void) {
    printf("\n%s\n", __func__);

    float input[4] = {1.0, 2.0, 3.0, 4.0};
    float output[4] = {0.0, 0.0, 0.0, 0.0};

    printf("Input:  [%f, %f, %f, %f]\n", input[0], input[1], input[2], input[3]);
    printf("Output before: [%f, %f, %f, %f]\n", output[0], output[1], output[2], output[3]);

    fast_copy(output, input, 4 * sizeof(float));

    printf("Output after:  [%f, %f, %f, %f]\n", output[0], output[1], output[2], output[3]);

    return 0;
}

static int test_inplace(void) {
    printf("\n%s\n", __func__);

    float data[4] = {1.0, -1.0, 1.0, -1.0};

    printf("Input:  [%f, %f, %f, %f]\n", data[0], data[1], data[2], data[3]);

    int result = fht_float(data, 2);

    printf("Output: [%f, %f, %f, %f]\n", data[0], data[1], data[2], data[3]);
    printf("Return value: %d\n", result);

    return 0;
}

static int test_oop(void) {
    printf("\n%s\n", __func__);

    float input[4] = {1.0, -1.0, 1.0, -1.0};
    float output[4] = {0.0, 0.0, 0.0, 0.0};

    printf("Input:  [%f, %f, %f, %f]\n", input[0], input[1], input[2], input[3]);
    printf("Output before: [%f, %f, %f, %f]\n", output[0], output[1], output[2], output[3]);

    int result = fht_float_oop(input, output, 2);

    printf("Output after:  [%f, %f, %f, %f]\n", output[0], output[1], output[2], output[3]);
    printf("Return value: %d\n", result);

    return 0;
}

int main(void) {
    test_defines();
    test_fast_copy();
    test_inplace();
    test_oop();
    return 0;
}
