#include "fht.h"
#include <stdio.h>

int main() {
    float input[4] = {1.0, -1.0, 1.0, -1.0};
    float output[4] = {0.0, 0.0, 0.0, 0.0};
    
    printf("Input:  [%f, %f, %f, %f]\n", input[0], input[1], input[2], input[3]);
    printf("Output before: [%f, %f, %f, %f]\n", output[0], output[1], output[2], output[3]);
    
    int result = fht_float_oop(input, output, 2);
    
    printf("Output after:  [%f, %f, %f, %f]\n", output[0], output[1], output[2], output[3]);
    printf("Return value: %d\n", result);
    
    return 0;
}
