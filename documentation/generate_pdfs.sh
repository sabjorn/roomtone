#!/usr/bin/env bash
echo "Run from root of Git repo"

INPUT_FILE="README.md"
OUTPUT_FILE="./documentation/pdfs/project_description.pdf"
echo "converting $INPUT_FILE to $OUTPUT_FILE"
docker run -v `pwd`:/source jagregory/pandoc $INPUT_FILE -s -V geometry:margin=1.1in -V urlcolor=cyan -o $OUTPUT_FILE

INPUT_FILE="./documentation/md/technical_outline.md"
OUTPUT_FILE="./documentation/pdfs/technical_outline.pdf"
echo "converting $INPUT_FILE to $OUTPUT_FILE"
docker run -v `pwd`:/source jagregory/pandoc $INPUT_FILE -s -V geometry:margin=1.1in -V urlcolor=cyan -o $OUTPUT_FILE