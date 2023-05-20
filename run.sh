#!/bin/bash

# Get the directory of the current script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Use the directory variable in the path to the binary
output=$("${SCRIPT_DIR}/target/release/cyberlogs")

nvim "$output"
