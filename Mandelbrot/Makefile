CC = icc
CFLAGS = -Wall -diag-disable=10441

OUTPUT_DIR = ./output/
BIN_DIR = ./bin/

SEQUENTIAL_DIR = ./src/sequential/
OPENMP_DIR = ./src/openmp/
CUDA_DIR = ./src/cuda/
MPI_DIR = ./src/mpi/

all: setup clear_screen run_sequential run_openmp run_cuda run_mpi

clear_screen:
	clear

setup:
	@echo "Setting up the environment..."
	@mkdir -p $(BIN_DIR)
	@mkdir -p $(OUTPUT_DIR)
	@mkdir -p $(OUTPUT_DIR)/rendered/
	@mkdir -p $(OUTPUT_DIR)/different_optimizations/
	@mkdir -p $(OUTPUT_DIR)/different_threads/
	@mkdir -p $(OUTPUT_DIR)/different_nodes/

# Compile implementations
compile_sequential:
	$(CC) $(CFLAGS) -fast -xsse3 $(SEQUENTIAL_DIR)mandelbrot.cpp -o $(BIN_DIR)seq_mandelbrot

compile_openmp:
	$(CC) -DRESOLUTION=1000 $(CFLAGS) -O3 -xsse3 -fopenmp $(OPENMP_DIR)mandelbrot.cpp -o $(BIN_DIR)openmp_mandelbrot

compile_cuda:
	nvc++ -DRESOLUTION=1000 $(CUDA_DIR)mandelbrot_cuComplex.cu -o $(BIN_DIR)cuda_mandelbrot

compile_mpi:
	mpi$(CC) $(MPI_DIR)mandelbrot.cpp -o $(BIN_DIR)mpi_mandelbrot

compile_all: compile_sequential compile_openmp compile_cuda compile_mpi
# ---------------------------

# Run single implementation
run_sequential: compile_sequential
	@echo "Sequential mandelbrot with -fast -xsse3 binary compiled."
	$(BIN_DIR)seq_mandelbrot $(OUTPUT_DIR)sequential_out.txt

run_openmp: compile_openmp
	@echo "OpenMP mandelbrot with binary compiled."
	$(BIN_DIR)openmp_mandelbrot $(OUTPUT_DIR)openmp_out.txt

run_cuda: compile_cuda
	@echo "CUDA mandelbrot binary compiled."
	$(BIN_DIR)cuda_mandelbrot $(OUTPUT_DIR)cuda_out.txt 32

run_mpi: compile_mpi
	@echo "MPI mandelbrot binary compiled."
	mpirun -np 4 $(BIN_DIR)mpi_mandelbrot $(OUTPUT_DIR)mpi_out.txt
# ---------------------------

all_sequential: clear_screen
	@echo "Compiling sequential mandelbrot with different optimizations..."
	$(CC) $(CFLAGS) -O0 $(SEQUENTIAL_DIR)mandelbrot.cpp -o $(BIN_DIR)seq_mandelbrot_O0
	$(CC) $(CFLAGS) -O1 $(SEQUENTIAL_DIR)mandelbrot.cpp -o $(BIN_DIR)seq_mandelbrot_O1
	$(CC) $(CFLAGS) -O2 $(SEQUENTIAL_DIR)mandelbrot.cpp -o $(BIN_DIR)seq_mandelbrot_O2
	$(CC) $(CFLAGS) -O3 $(SEQUENTIAL_DIR)mandelbrot.cpp -o $(BIN_DIR)seq_mandelbrot_O3
	$(CC) $(CFLAGS) -fast -mavx2 $(SEQUENTIAL_DIR)mandelbrot.cpp -o $(BIN_DIR)seq_mandelbrot_fast_mavx2
	$(CC) $(CFLAGS) -fast $(SEQUENTIAL_DIR)mandelbrot.cpp -o $(BIN_DIR)seq_mandelbrot_fast
	$(CC) $(CFLAGS) -fast -xsse3 $(SEQUENTIAL_DIR)mandelbrot.cpp -o $(BIN_DIR)seq_mandelbrot_fast_xsse3
	@echo ""
	@echo "Running sequential mandelbrot with different optimizations..."
	@echo ""
	@echo "Sequential mandelbrot with -O0 binary compiled."
	$(BIN_DIR)seq_mandelbrot_O0 $(OUTPUT_DIR)different_optimizations/seq_out_O0.txt
	@echo "Sequential mandelbrot with -O1 binary compiled."
	$(BIN_DIR)seq_mandelbrot_O1 $(OUTPUT_DIR)different_optimizations/seq_out_O1.txt
	@echo "Sequential mandelbrot with -O2 binary compiled."
	$(BIN_DIR)seq_mandelbrot_O2 $(OUTPUT_DIR)different_optimizations/seq_out_O2.txt
	@echo "Sequential mandelbrot with -O3 binary compiled."
	$(BIN_DIR)seq_mandelbrot_O3 $(OUTPUT_DIR)different_optimizations/seq_out_O3.txt
	@echo "Sequential mandelbrot with -fast binary compiled."
	$(BIN_DIR)seq_mandelbrot_fast $(OUTPUT_DIR)different_optimizations/seq_out_fast.txt
	@echo "Sequential mandelbrot with -fast and -mavx2 (larger instruction set) binary compiled."
	$(BIN_DIR)seq_mandelbrot_fast_mavx2 $(OUTPUT_DIR)different_optimizations/seq_out_fast_mavx2.txt
	@echo "Sequential mandelbrot with -fast and -xsse3 (smaller instruction set) binary compiled."
	$(BIN_DIR)seq_mandelbrot_fast_xsse3 $(OUTPUT_DIR)different_optimizations/seq_out_fast_xsse3.txt

all_cuda: clear_screen
	@echo ""
	nvc++ -DRESOLUTION=1000 $(CUDA_DIR)mandelbrot_cuComplex.cu -o $(BIN_DIR)cuda_mandelbrot
	nvc++ -DRESOLUTION=1000 -acc $(CUDA_DIR)mandelbrot_cuComplex.cu -o $(BIN_DIR)cuda_mandelbrot_acc
	nvc++ -DRESOLUTION=1000 -acc=multicore $(CUDA_DIR)mandelbrot_cuComplex.cu -o $(BIN_DIR)cuda_mandelbrot_acc_multicore
	@echo ""
	@echo "Running CUDA mandelbrot with different optimizations..."
	@echo ""
	@echo ""
	@echo "Running CUDA mandelbrot with default settings..."
	for threads in 2 4 8 16 32; do \
		$(BIN_DIR)cuda_mandelbrot $(OUTPUT_DIR)different_optimizations/cuda_out.txt $$threads; \
	done
	@echo ""
	@echo "Running CUDA mandelbrot with -acc flag..."
	for threads in 2 4 8 16 32; do \
		$(BIN_DIR)cuda_mandelbrot_acc $(OUTPUT_DIR)different_optimizations/cuda_out_acc.txt $$threads; \
	done
	@echo ""
	@echo "Running CUDA mandelbrot with -acc=multicore flag..."
	for threads in 2 4 8 16 32; do \
		$(BIN_DIR)cuda_mandelbrot_acc_multicore $(OUTPUT_DIR)different_optimizations/cuda_out_acc_multicore.txt $$threads; \
	done

omp_speedup: clear_screen
	@echo ""
	for threads in `seq 1 24`; do \
		for resolution in 1000 2000 5000; do \
			echo "Running OpenMP mandelbrot with $$threads threads and $$resolution resolution..."; \
			$(CC) -DRESOLUTION=$$resolution $(CFLAGS) -O3 -xsse3 -fopenmp $(OPENMP_DIR)mandelbrot.cpp -o $(BIN_DIR)openmp_mandelbrot; \
			$(BIN_DIR)openmp_mandelbrot $(OUTPUT_DIR)/different_threads/openmp_out_$$threads.txt -t $$threads; \
		done \
	done
	@echo ""

cuda_test: clear_screen
	@echo ""
	for threads in 2 4 8 16 32; do \
		for resolution in 1000 2000 5000; do \
			echo "Running CUDA mandelbrot with $$threads threads and $$resolution resolution..."; \
			nvc++ -DRESOLUTION=$$resolution $(CUDA_DIR)mandelbrot_cuComplex.cu -o $(BIN_DIR)cuda_mandelbrot; \
			$(BIN_DIR)cuda_mandelbrot out.txt $$threads; \
		done \
	done
	@echo ""

mpi_speedup: clear_screen compile_mpi
	for node in `seq 1 24`; do \
		mpirun -np $$node $(BIN_DIR)mpi_mandelbrot $(OUTPUT_DIR)/different_threads/mpi_out_$$node.txt; \
	done

render:
	python3 ./src/utils/image/render.py

accuracy:
	python3 ./src/utils/metrics/compute_accuracy.py

plot:
	python3 ./src/utils/plot/plot_speed_up_omp.py
	python3 ./src/utils/plot/plot_speed_up_mpi.py

clean:
	rm -f $(BIN_DIR)*