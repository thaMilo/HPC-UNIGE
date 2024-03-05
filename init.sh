#!/bin/bash

# RUST / METAL PART
# compile the shaders located in the metal folder
make opt-1
./target/opt-1/mandelbrot_lab --help
./target/opt-1/mandelbrot_lab --name elia -x -2 -X 1 -y -1 -Y 1 -r 1000  -d 2 -i 1000 -s run -m no_exec -v vis
./target/opt-3/mandelbrot_lab --name milo -x -2 -X 1 -y -1 -Y 1 -r 1000  -d 2 -i 1000 -s run -m no_exec -v vis
