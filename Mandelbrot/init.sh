#!/bin/bash

# RUST / METAL PART
# compile the shaders located in the metal folder
xcrun -sdk macosx metal -c mandelbrot_lab/src/metal/mandelbrot.metal -o mandelbrot_lab/src/metal/mandelbrot.air && xcrun -sdk macosx metallib mandelbrot_lab/src/metal/mandelbrot.air -o mandelbrot_lab/src/metal/mandelbrot.metallib && cd mandelbrot_lab && cargo run --release && cd ..
