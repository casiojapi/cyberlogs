#!/bin/bash

output=$(./target/release/cyberlogs)
nvim "$output"
