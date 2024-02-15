#include <iostream>
#include <fstream>
#include <chrono>
#include <complex>
#include <iomanip>
#include <cuda_runtime.h>
#include <cuComplex.h>
#include <sys/time.h>

#define MIN_X -2
#define MAX_X 1
#define MIN_Y -1
#define MAX_Y 1

#define RATIO_X (MAX_X - MIN_X)
#define RATIO_Y (MAX_Y - MIN_Y)

#define WIDTH (RATIO_X * RESOLUTION)
#define HEIGHT (RATIO_Y * RESOLUTION)

#define STEP ((double)RATIO_X / WIDTH)

#define ITERATIONS 1000

using namespace std;

float time_diff(struct timeval *start, struct timeval *end) {
  	return (end->tv_sec - start->tv_sec) + 1e-6 * (end->tv_usec - start->tv_usec);
}

__device__ int mandelbrot(int col, int row, double step, int minX, int minY, int iterations)
{
	cuDoubleComplex c = make_cuDoubleComplex(col * step + minX, row * step + minY);
    cuDoubleComplex z = make_cuDoubleComplex(0, 0);

    int count = 0;
    while (cuCabs(z) < 2 && count < iterations)
    {
        z = cuCadd(cuCmul(z, z), c);
        count++;
    }

   	return (count < iterations) ? count : 0;
}

__global__ void generateMandelbrotSet(int *image, double step, int minX, int minY, int iterations)
{
	int col = blockIdx.x * blockDim.x + threadIdx.x;
	int row = blockIdx.y * blockDim.y + threadIdx.y;

	if (col >= WIDTH || row >= HEIGHT)
		return;

	int index = row * WIDTH + col;

	image[index] = mandelbrot(col, row, step, minX, minY, iterations);
}

int main(int argc, char *argv[])
{
	int threads = 1;
	if (argc < 2)
	{
		cout << "Please specify the output file as a parameter." << endl;
		return -1;
	}

	if (argc > 2)
	{
		threads = atoi(argv[2]);
	}

	cout << "Threads: " << threads << endl;
	cout << "Resolution: " << RESOLUTION << endl;

	int *image;
	int *d_image;
	size_t img_size = WIDTH * HEIGHT * sizeof(int);
	
	struct timeval start, stop, op_start, op_stop;
	cudaEvent_t startEvent, stopEvent, startOpEvent, stopOpEvent;
	
	cudaEventCreate(&startEvent); cudaEventCreate(&stopEvent);
	cudaEventCreate(&startOpEvent); cudaEventCreate(&stopOpEvent);

	gettimeofday(&start, NULL);
	cudaEventRecord(startEvent);

	cudaMalloc(&d_image, img_size);

	// Allocate memory for the image on the host
	image = new int[HEIGHT * WIDTH];

	// Initialize all pixels to -1
	fill_n(image, WIDTH * HEIGHT, -1);

	// Copy the image to the device
	cudaMemcpy(d_image, image, img_size, cudaMemcpyHostToDevice);

	dim3 blockSize(threads, threads);
	dim3 gridSize((WIDTH + blockSize.x - 1) / blockSize.x, (HEIGHT + blockSize.y - 1) / blockSize.y);

	//auto start = chrono::steady_clock::now();

	gettimeofday(&op_start, NULL);
	cudaEventRecord(startOpEvent);
	generateMandelbrotSet<<<gridSize, blockSize>>>(d_image, STEP, MIN_X, MIN_Y, ITERATIONS);
	gettimeofday(&op_stop, NULL);
	cudaEventRecord(stopOpEvent);
	cudaEventSynchronize(stopOpEvent);

	// Copy the image back to the host
	cudaMemcpy(image, d_image, img_size, cudaMemcpyDeviceToHost);

	cudaEventRecord(stopEvent);
	cudaEventSynchronize(stopEvent);
	gettimeofday(&stop, NULL);

	// Check if all pixels were computed
	/*
	bool allPixelsComputed = true;
	for (int i = 0; i < HEIGHT * WIDTH; i++)
	{
		if (image[i] == -1)
		{ // replace -1 with the value you use for uninitialized pixels
			allPixelsComputed = false;
			break;
		}
	}

	if (allPixelsComputed)
	{
		cout << "All pixels were computed." << endl;
	}
	else
	{
		cout << "Not all pixels were computed." << endl;
	}
	*/

	printf("Time spent: %0.8f ms\n", time_diff(&start, &stop)*1000);
  	printf("Op_Time spent: %0.8f ms\n", time_diff(&op_start, &op_stop)*1000);

	// Print time
	cout << "Time of operations: ";
	float time, timeOp;
	cudaEventElapsedTime(&timeOp, startOpEvent, stopOpEvent);
	cout << fixed << setprecision(2) << timeOp << " ms" << endl;

	cout << "Time total: ";
	cudaEventElapsedTime(&time, startEvent, stopEvent);
	cout << fixed << setprecision(2) << time << " ms" << endl;

	/*
	auto end = chrono::steady_clock::now();
	cout << "Time elapsed: "
		 << fixed << setprecision(2)
		 << chrono::duration<double>(end - start).count()
		 << " seconds." << endl;
	*/

	ofstream matrix_out(argv[1], ios::trunc);
	if (!matrix_out.is_open())
	{
		cout << "Unable to open file." << endl;
		return -2;
	}

	for (int row = 0; row < HEIGHT; row++)
	{
		for (int col = 0; col < WIDTH; col++)
		{
			matrix_out << image[row * WIDTH + col];
			if (col < WIDTH - 1)
				matrix_out << ',';
		}
		if (row < HEIGHT - 1)
			matrix_out << '\n';
	}
	matrix_out.close();

	delete[] image;
	cudaFree(d_image);

	return 0;
}