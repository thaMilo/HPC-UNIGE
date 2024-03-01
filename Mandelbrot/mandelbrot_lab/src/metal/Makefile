# Makefile for compiling Metal shader file and creating Metal library

# Compiler
METAL_COMPILER = xcrun -sdk macosx metal
METALLIB_COMPILER = xcrun -sdk macosx metallib

# Source file
SOURCE_FILE = mandelbrot.metal

# Intermediate file
INTERMEDIATE_FILE = mandelbrot.air

# Output file
OUTPUT_FILE = mandelbrot.metallib

all: $(OUTPUT_FILE)

# Compile rule
$(INTERMEDIATE_FILE): $(SOURCE_FILE)
	$(METAL_COMPILER) -c $(SOURCE_FILE) -o $(INTERMEDIATE_FILE)

# Create Metal library rule
$(OUTPUT_FILE): $(INTERMEDIATE_FILE)
	$(METALLIB_COMPILER) $(INTERMEDIATE_FILE) -o $(OUTPUT_FILE)

# Phony target to clean compiled files
.PHONY: clean
clean:
	rm -f $(INTERMEDIATE_FILE) $(OUTPUT_FILE)
