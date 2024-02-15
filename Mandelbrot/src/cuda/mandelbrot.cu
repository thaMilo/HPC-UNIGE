#include <iostream>
#include <fstream>
#include <chrono>
#include <complex>
#include <iomanip>
#include <cuda.h>

#define MIN_X -2
#define MAX_X 1
#define MIN_Y -1
#define MAX_Y 1

#define RATIO_X (MAX_X - MIN_X)
#define RATIO_Y (MAX_Y - MIN_Y)

#define ITERATIONS 1000

using namespace std;

__device__ int mandelbrot(int col, int row, double step, int minX, int minY, int iterations)
{
	const complex<double> c(minX + col * step, minY + row * step);
	complex<double> z(0, 0);

	int count = 0;
	while (abs(z) < 2 && count <= iterations)
	{
		z = pow(z, 2) + c;
		count++;
	}

	if (count < iterations)
	{
		return count;
	}
	else
	{
		return 0;
	}
}

__global__ void generateMandelbrotSet(int *image, double step, int minX, int minY, int iterations, int WIDTH, int HEIGHT)
{
	int col = blockIdx.x * blockDim.x + threadIdx.x;
	int row = blockIdx.y * blockDim.y + threadIdx.y;

	if (col >= WIDTH || row >= HEIGHT)
		return;

	int index = row * WIDTH + col;

	image[index] = mandelbrot(col, row, step, minX, minY, iterations);
}

int main(int argc, char **argv)
{
	int RESOLUTION = 1000;
	int threads = 2;

	if (argc < 2)
	{
		cout << "Please specify the output file as a parameter." << endl;
		return -1;
	} else {
		if (argc == 3)
		{
			RESOLUTION = atoi(argv[2]);
			threads = atoi(argv[3]);
		}
	}

	cout << "Resolution: " << RESOLUTION << endl;

	int WIDTH = (RATIO_X * RESOLUTION);
	int HEIGHT = (RATIO_Y * RESOLUTION);

	int STEP = ((double)RATIO_X / WIDTH);

	int *image;
	int *d_image;
	size_t img_size = WIDTH * HEIGHT * sizeof(int);

	cudaMalloc(&d_image, img_size);

	// Allocate memory for the image on the host
	image = new int[HEIGHT * WIDTH];

	// Initialize all pixels to -1
	fill_n(image, WIDTH * HEIGHT, -1);

	// Copy the image to the device
	cudaMemcpy(d_image, image, img_size, cudaMemcpyHostToDevice);

	dim3 blockSize(threads, threads);
	dim3 gridSize((WIDTH + blockSize.x - 1) / blockSize.x, (HEIGHT + blockSize.y - 1) / blockSize.y);

	auto start = chrono::steady_clock::now();

	generateMandelbrotSet<<<gridSize, blockSize>>>(d_image, STEP, MIN_X, MIN_Y, ITERATIONS, WIDTH, HEIGHT);

	// Copy the image back to the host
	cudaMemcpy(image, d_image, img_size, cudaMemcpyDeviceToHost);

	// Check if all pixels were computed
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

	auto end = chrono::steady_clock::now();
	cout << "Time elapsed: "
		 << fixed << setprecision(2)
		 << chrono::duration<double>(end - start).count()
		 << " seconds." << endl;

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