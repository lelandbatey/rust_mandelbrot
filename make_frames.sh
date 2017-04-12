#!/bin/bash

mkdir -p frames

for i in $(seq -w  0 0.002 16)
do
	PGMPATH="./frames/img$i.pgm"
	PNGPATH="./frames/img$i.png"
	echo "./target/release/mandelbrot -r 1920 -z \"$i\" > $PGMPATH"
	./target/release/mandelbrot -r 1920 -z "$i" > "$PGMPATH"
	convert "$PGMPATH" "$PNGPATH"
	rm "$PGMPATH"
done
