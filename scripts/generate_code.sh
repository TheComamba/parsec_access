#!/bin/bash

# Check if python is available
if command -v python &> /dev/null
then
    PYTHON_CMD=python
elif command -v python3 &> /dev/null
then
    PYTHON_CMD=python3
else
    echo "Python is not installed on your system."
    exit 1
fi

cd $(git rev-parse --show-toplevel)

# Create a virtual environment
$PYTHON_CMD -m venv scripts/venv

# Activate the virtual environment
source scripts/venv/bin/activate

$PYTHON_CMD -m pip install --upgrade pip > /dev/null

# Install necessary modules
pip install beautifulsoup4 requests > /dev/null

# Execute the script
$PYTHON_CMD scripts/generate_code.py

# Deactivate the virtual environment
deactivate

cargo fmt
