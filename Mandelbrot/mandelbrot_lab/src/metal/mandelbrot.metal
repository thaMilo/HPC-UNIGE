#include <metal_stdlib>
using namespace metal;

// Define complex number struct, no (easy) complex lib in metal :(

struct Complex {
    float re, im;
};

Complex pow(Complex c) {
    Complex result;
    result.re = c.re * c.re - c.im * c.im;
    result.im = c.re * c.im + c.im * c.re;
    return result;
}

Complex add(Complex a, Complex b) {
    Complex result;
    result.re = a.re + b.re;
    result.im = a.im + b.im;
    return result;
}

// Metal compute kernel function
kernel void generateMandelbrotSet(device float* image [[buffer(0)]],
                                  device float* STEP [[buffer(1)]],
                                  device int* MIN_X [[buffer(2)]],
                                  device int* MIN_Y [[buffer(3)]],
                                  device int* WIDTH [[buffer(4)]],
                                  device int* HEIGHT [[buffer(5)]],
                                  device int* iterations [[buffer(6)]],
                                  uint2 pos [[thread_position_in_grid]]
                                  ) {
    Complex constant_complex_number = {
        (float)((float)pos.x * STEP[0] + (float)MIN_X[0]),
        (float)((float)pos.y * STEP[0] + (float)MIN_Y[0])
    };

    Complex z = {0.0f, 0.0f};

    for ( int i = 0; i < iterations[0]; i++ ) {
        z = pow(z);
        z.re += constant_complex_number.re;
        z.im += constant_complex_number.im;

        if ((z.re * z.re + z.im * z.im) >= 4.0f) {
            image[pos.x + ( WIDTH[0] * pos.y )] = i;
            return;
        }
    }
}
