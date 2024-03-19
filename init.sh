#!/bin/bash

# Define the array of resolutions
# resolutions=(10000 20000 30000 40000 50000)

# Loop through resolutions
# for resolution in "${resolutions[@]}"
for ((resolution=10; resolution<=1000; resolution+=10))
do
    # Loop through thread counts from 1 to 1024
    for ((thread_count=1; thread_count<=1024; thread_count*=2))
    do
        # Execute the metal code with the current thread count and resolution
        cargo run --profile opt-3 -- -n metal-final -m run --thread-count $thread_count -r $resolution
    done
done
