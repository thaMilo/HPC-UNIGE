CC = icc
CFLAGS = -Wall -diag-disable=10441

OUTPUT_DIR = ./output/
BIN_DIR = ./bin/

clear_screen:
	clear

setup:
	@echo "Setting up the environment..."
	@mkdir -p $(BIN_DIR)
	@mkdir -p $(OUTPUT_DIR)
	@mkdir -p $(OUTPUT_DIR)/speedup/

compile_mpi:
	mpi$(CC) $(MPI_DIR)mandelbrot.cpp -o $(BIN_DIR)mpi_mandelbrot

run_mpi: compile_mpi
	@echo "MPI mandelbrot binary compiled."
	mpirun -np 4 $(BIN_DIR)mpi_mandelbrot $(OUTPUT_DIR)mpi_out.txt

speedup: compile_mpi
	@echo ""
	for nodes in 16 32 64 128 256 512 1024 2048 3072; do \
		for resolution in 1000 2000 5000; do \
			mpi$(CC) -DRESOLUTION=$$resolution $(MPI_DIR)mandelbrot.cpp -o $(BIN_DIR)mpi_mandelbrot; \
			mpirun -np $$nodes $(BIN_DIR)mpi_mandelbrot $(OUTPUT_DIR)/speedup/mpi_out_$$nodes.txt $$resolution; \
		done \
	done
	@echo ""