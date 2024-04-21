#!/bin/zsh

# Directory containing the .wav files
directory=$1

# Output file
output="combined.wav"

# Check if directory is provided
if [[ -z "$directory" ]]; then
  echo "Usage: $0 <directory>"
  exit 1
fi

# Check if 'sox' is installed
if ! command -v sox &> /dev/null; then
    echo "This script requires 'sox' but it's not installed. Install it and try again."
    exit 1
fi

# Navigate to the directory
cd "$directory" || exit

# Initialize an array to hold the file names
wav_files=()

# Populate the array with the names of the .wav files, properly handling spaces
for file in *.wav; do
  echo "Processing $file..."
  wav_files+=("$file")
done

echo "Combining .wav files in $directory..."

# Concatenate all .wav files into one, using the array to handle spaces in filenames
sox "${wav_files[@]}" "$output"

echo "All .wav files have been combined into $output"