#!/bin/bash

output=$(./target/debug/cyberlogs)
nvim "$output"
