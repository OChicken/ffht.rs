#include "fast_copy.h"
#include <stdio.h>

int main() {
    float input[4] = {1.0, 2.0, 3.0, 4.0};
    float output[4] = {0.0, 0.0, 0.0, 0.0};
    
    printf("Input:  [%f, %f, %f, %f]\n", input[0], input[1], input[2], input[3]);
    printf("Output before: [%f, %f, %f, %f]\n", output[0], output[1], output[2], output[3]);
    
    fast_copy(output, input, 4 * sizeof(float));
    
    printf("Output after:  [%f, %f, %f, %f]\n", output[0], output[1], output[2], output[3]);
    
    return 0;
}
