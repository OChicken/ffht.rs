#include "fht.h"
#include <stdio.h>

int main() {
    float data[4] = {1.0, -1.0, 1.0, -1.0};
    
    printf("Input:  [%f, %f, %f, %f]\n", data[0], data[1], data[2], data[3]);
    
    int result = fht_float(data, 2);
    
    printf("Output: [%f, %f, %f, %f]\n", data[0], data[1], data[2], data[3]);
    printf("Return value: %d\n", result);
    
    return 0;
}
