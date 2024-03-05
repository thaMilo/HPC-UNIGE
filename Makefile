.PHONY: all clean opt-0 opt-3

# Set the target directories
TARGET_DIR_OPT_0 := ./target/opt-0
TARGET_DIR_OPT_1 := ./target/opt-1
TARGET_DIR_OPT_2 := ./target/opt-2
TARGET_DIR_OPT_3 := ./target/opt-3
TARGET_DIR_OPT_native := ./target/opt-native

# Metal directory
METAL_DIR := ./src/mandelbrot/metal
# Set Metal shader source files
METAL_SRC := $(wildcard $(METAL_DIR)/*.metal)
METAL_AIR := $(METAL_SRC:.metal=.air)
METAL_LIB := $(METAL_SRC:.metal=.metallib)

# Define the Metal compiler
METAL_COMPILER := xcrun -sdk macosx metal
METALLIB_COMPILER := xcrun -sdk macosx metallib

# Compile the Metal shader source files
$(METAL_AIR): $(METAL_SRC)
	$(METAL_COMPILER) -c $< -o $@

# Create the Metal library
$(METAL_LIB): $(METAL_AIR)
	$(METALLIB_COMPILER) $< -o $@

# Define the target executables
TARGET_OPT_0 := $(TARGET_DIR_OPT_0)/mandelbrot_lab
TARGET_OPT_1 := $(TARGET_DIR_OPT_1)/mandelbrot_lab
TARGET_OPT_2 := $(TARGET_DIR_OPT_2)/mandelbrot_lab
TARGET_OPT_3 := $(TARGET_DIR_OPT_3)/mandelbrot_lab
TARGET_OPT_native := $(TARGET_DIR_OPT_native)/mandelbrot_lab

# Default target
all: $(TARGET_OPT_0) $(TARGET_OPT_1) $(TARGET_OPT_2) $(TARGET_OPT_3) $(TARGET_OPT_native)

# Build the target executable with opt-0 profile
opt-0: $(TARGET_OPT_0)

$(TARGET_OPT_0): $(METAL_LIB) $(wildcard src/*.rs)
	cargo build --profile opt-0

# Build the target executable with opt-1 profile
opt-1: $(TARGET_OPT_1)

$(TARGET_OPT_1): $(METAL_LIB) $(wildcard src/*.rs)
	cargo build --profile opt-1

# Build the target executable with opt-2 profile
opt-2: $(TARGET_OPT_2)

$(TARGET_OPT_2): $(METAL_LIB) $(wildcard src/*.rs)
	cargo build --profile opt-2

# Build the target executable with opt-3 profile
opt-3: $(TARGET_OPT_3)

$(TARGET_OPT_3): $(METAL_LIB) $(wildcard src/*.rs)
	cargo build --profile opt-3

# Build the target executable with opt-native profile
opt-native: $(TARGET_OPT_native)

$(TARGET_OPT_native): $(METAL_LIB) $(wildcard src/*.rs)
	cargo build --profile opt-native

# Clean the target directories
clean:
	cargo clean
	rm -f $(METAL_AIR) $(METAL_LIB)