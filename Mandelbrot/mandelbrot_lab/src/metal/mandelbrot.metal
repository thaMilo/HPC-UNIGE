#include <metal_stdlib>
using namespace metal;

// Define complex number struct, no (easy) complex lib in metal :(

struct Complex {
    float re, im;
};

// Metal compute kernel function
kernel void generateMandelbrotSet(device float* image [[buffer(0)]],
                                  device float* STEP [[buffer(1)]],
                                  device int* MIN_X [[buffer(2)]],
                                  device int* MIN_Y [[buffer(3)]],
                                  device int* WIDTH [[buffer(4)]],
                                  device int* HEIGHT [[buffer(5)]],
                                  device int* iterations [[buffer(6)]],
                                  uint pos [[thread_position_in_grid]]
                                  ) {
    int col = pos % WIDTH[0] ;
    int row = pos / WIDTH[0] ;

    Complex constant_complex_number = {(float)(MIN_X[0] + col * STEP[0]), (float)(MIN_Y[0] + row * STEP[0])};
    Complex z = {0.0f, 0.0f};
    
    for ( int i = 0; i < iterations[0]; i++ ) {
        float temp = z.re * z.re -  z.im * z.im + constant_complex_number.re;
        z.im = 2.0f * z.re * z.im + constant_complex_number.im;
        z.re = temp;
        if (sqrt(z.re * z.re + z.im * z.im) > 2.0f) {
            image[row*col] = i;
        }else{
            image[row*col] = 0;
        }
    }
}
