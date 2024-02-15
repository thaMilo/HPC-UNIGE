#include <iostream>
#include <fstream>
#include <complex>
#include <chrono>
#include <iomanip>
#include <mpi.h>

// Ranges of the set
#define MIN_X -2
#define MAX_X 1
#define MIN_Y -1
#define MAX_Y 1

// Image ratio
#define RATIO_X (MAX_X - MIN_X)
#define RATIO_Y (MAX_Y - MIN_Y)

// Image size
#define WIDTH (RATIO_X * RESOLUTION)
#define HEIGHT (RATIO_Y * RESOLUTION)

#define STEP ((double)RATIO_X / WIDTH)

#define DEGREE 2		// Degree of the polynomial
#define ITERATIONS 1000 // Maximum number of iterations

using namespace std;

int main(int argc, char **argv)
{
    MPI_Init(&argc, &argv);

    int world_size, world_rank;
    MPI_Comm_size(MPI_COMM_WORLD, &world_size);
    MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);

    if (world_rank == 0) {
        cout << "Number of nodes: " << world_size << endl;
        cout << "Resolution: " << RESOLUTION << endl;
    }

    const int total_pixels = HEIGHT * WIDTH;
    const int pixels_per_process = total_pixels / world_size;
    const int start_index = world_rank * pixels_per_process;
    const int end_index = (world_rank + 1) * pixels_per_process;

    int *image;
    int *sub_image = new int[pixels_per_process];

    if (world_rank == 0) {
        image = new int[total_pixels];
    }

    auto start = chrono::steady_clock::now();

    // Each process computes its portion of the image
    for (int pos = start_index; pos < end_index; pos++) {
        const int row = pos / WIDTH;
        const int col = pos % WIDTH;
        const complex<double> c(col * STEP + MIN_X, row * STEP + MIN_Y);

        complex<double> z(0, 0);
        for (int i = 1; i <= ITERATIONS; i++) {
            z = pow(z, 2) + c;

            // If it is convergent
            if (abs(z) >= 2) {
                sub_image[pos - start_index] = i;
                break;
            }
        }
    }

    // Gather results from all processes to the root process
    MPI_Gather(sub_image, pixels_per_process, MPI_INT, image, pixels_per_process, MPI_INT, 0, MPI_COMM_WORLD);

    if (world_rank == 0) {
        auto end = chrono::steady_clock::now();
        cout << "Time elapsed: "
             << fixed << setprecision(2)
             << chrono::duration<double>(end - start).count()
             << " seconds." << endl;
		        // Write the result to a file
        ofstream matrix_out;
        if (argc < 2) {
            cout << "Please specify the output file as a parameter." << endl;
            MPI_Abort(MPI_COMM_WORLD, -1);
        }

        matrix_out.open(argv[1], ios::trunc);
        if (!matrix_out.is_open()) {
            cout << "Unable to open file." << endl;
            MPI_Abort(MPI_COMM_WORLD, -2);
        }

        for (int row = 0; row < HEIGHT; row++) {
            for (int col = 0; col < WIDTH; col++) {
                matrix_out << image[row * WIDTH + col];
                if (col < WIDTH - 1)
                    matrix_out << ',';
            }
            if (row < HEIGHT - 1)
                matrix_out << endl;
        }
        matrix_out.close();
        delete[] image;
    }

    delete[] sub_image;
    MPI_Finalize();
    return 0;
}