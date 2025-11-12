#include <stdio.h>

int main() {
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
    
    printf("Done\n");
    return 0;
}
