# HPC - Mandelbrot :rocket:

## :book: Project Overview
This project implements the Mandelbrot set visualization algorithm in a high-performance computing environment using MPI, OpenMP, and CUDA. The goal is to explore and demonstrate the efficiencies and challenges of parallel computing in handling large-scale data visualization tasks.

## :sparkles: Features
- **OpenMP Implementation**: Shared memory parallelism for efficient multi-threading on single-node multi-core processors.
- **CUDA Implementation**: GPU acceleration to leverage high-throughput computing power of modern GPUs.
- **MPI Implementation**: Distributed memory parallelism for visualization of large datasets across multiple nodes.

## :wrench: Getting Started
### :arrow_down: Installation
1. Clone the repository:

    ```bash
    git clone https://github.com/SamuelePignone/HPC-Mandelbrot.git
    ```

2. Navigate to the project directory:
    
    ```bash
    cd HPC-Mandelbrot
    ```

3. Install dependencies (Intel and CUDA):

    ```bash
    source /opt/intel/oneapi/setvars.sh; NVARCH=`uname -s`_`uname -m`; export NVARCH; NVCOMPILERS=/opt/nvidia/hpc_sdk; export NVCOMPILERS; MANPATH=$MANPATH:$NVCOMPILERS/$NVARCH/23.7/compilers/man; export MANPATH; PATH=$NVCOMPILERS/$NVARCH/23.7/compilers/bin:$PATH; export PATH;
    ```

4. Compile the code (Makefile provided):

    ```bash
    make all
    ```

5. (Optional) Generate images using the python script:

    ```bash
    make render
    ```

## :file_folder: Folder Structure
| Directory | Description |
|-----------|-------------|
| `/src` | Source code for Sequential, OpenMP, MPI and CUDA implementations. |
| &emsp;`/sequential` | Sequential code |
| &emsp;`/openmp` | OpenMP-specific code. |
| &emsp;`/mpi` | MPI-specific code. |
| &emsp;`/cuda` | CUDA-specific code. |
| &emsp;`/image` | Code for image generation in python. |
| `/report` | Files for the final report. |
| &emsp;`/images` | Images for the report. |
| `/output` | Output results |
| &emsp;`/rendered` | Rendered images |
| &emsp;`/different_optimizations` | Output results for different sequential optimizations. |
| &emsp;`/different_threads` | Output results for different OpenMP threads. |
| &emsp;`/different_nodes` | Output results for different MPI nodes. |
| `/bin` | Executables. |

## :mailbox: Contact
- Samuele Pignone - s4838155@studenti.unige.it
- Nicolò Guainazzo - s4486891@studenti.unige.it