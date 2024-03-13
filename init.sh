#!/bin/bash

# # RUST / SEQUENTIAL PART
# cargo run --profile opt-0 -- -s run -e run -n opt-0-seq -r 1000
# cargo run --profile opt-1 -- -s run -e run -n opt-1-seq -r 1000
# cargo run --profile opt-2 -- -s run -e run -n opt-2-seq -r 1000
# cargo run --profile opt-3 -- -s run -e run -n opt-3-seq -r 1000

# # RUST / PARALLEL PART
# cargo run --profile opt-0 -- -m run -e run -n opt-0-metal -r 1000
# cargo run --profile opt-1 -- -m run -e run -n opt-1-metal -r 1000
# cargo run --profile opt-2 -- -m run -e run -n opt-2-metal -r 1000
# cargo run --profile opt-3 -- -m run -e run -n opt-3-metal -r 1000

# # Changing resolution
# # resolution 2000
# cargo run --profile opt-0 -- -s run -e run -n opt-0-seq -r 2000

# changing resolutions for metal
# cargo run --profile opt-3 -- -n metal -m run --thread-count 16 -r 100000
for resolution in "${resolutions[@]}"; do
	for i in {1..1024}; do
		cargo run --profile opt-0 -- -m run -e run -n opt-0-metal --thread-count $i -r $resolution
	done
done

