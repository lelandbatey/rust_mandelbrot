#!/bin/bash

mkdir -p frames

RESOLUTION=1920
THREADS=6

# Some real good coordinates:
# -x -0.701000092002025 -y 0.351000000999792

#for i in $(seq -w  0 0.002 16)
for i in $(seq -w  0 0.01 51)
do
	PGMPATH="./frames/img$i.pgm"
	PNGPATH="./frames/img$i.png"
	echo "./target/release/mandelbrot --max_iters 400 -r $RESOLUTION -t $THREADS -x -0.701000092002025 -y 0.351000000999792 -z \"$i\" > $PGMPATH"
	./target/release/mandelbrot --max_iters 400 -r "$RESOLUTION" -t "$THREADS" -x -0.701000092002025 -y 0.351000000999792 -z "$i" > "$PGMPATH"
	convert "$PGMPATH" "$PNGPATH"
	rm "$PGMPATH"
done
