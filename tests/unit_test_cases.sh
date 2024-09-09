#!/bin/bash

# Directory containing the JSON test case files
JSON_DIR="tests/unit_test_cases"

# Initialize error flag
error_flag=0

# Loop through each JSON file in the directory
for json_file in "$JSON_DIR"/*.json; do
    # Extract test names from the JSON file using jq
    test_names=$(jq -r '.test_names[]' "$json_file")

    # Extract the test files from the JSON
    jq -c '.test_files[] | select(type=="object")' "$json_file" | while read test_file; do
        # Extract language and file path
        language=$(echo "$test_file" | jq -r '.language')
        file_path=$(echo "$test_file" | jq -r '.path')

        # Read the content of the test file
        if [[ -f "$file_path" ]]; then
            file_content=$(cat "$file_path")
        else
            echo "  File not found: $file_path"
            error_flag=1  # Set error flag
            continue
        fi

        # Loop through each test name
        for test_name in $test_names; do
            # Check if the test name exists in the file content
            if ! grep -q "$test_name" <<< "$file_content"; then
                echo "  Missing test: $test_name in $file_path"
                error_flag=1  # Set error flag
            fi
        done
    done
done

# Exit with error if any issues were found
if [[ $error_flag -ne 0 ]]; then
    exit 1
fi
