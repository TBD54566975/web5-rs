#!/bin/bash

for dir in */; do
    echo "Entering directory: $dir"
    (cd "$dir" && ./build)
    echo "Finished building in $dir"
    echo "------------------------"
done

echo "All builds completed"
