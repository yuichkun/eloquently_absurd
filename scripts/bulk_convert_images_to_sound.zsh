#!/bin/zsh

setopt nullglob

# Directory containing the images
image_directory="$1"

# Check if the directory argument is provided and exists
if [[ -z "$image_directory" || ! -d "$image_directory" ]]; then
    echo "Please provide a valid directory."
    exit 1
fi

resized_directory="${image_directory}/resized"
if [[ ! -d "$resized_directory" ]]; then
    echo "Creating resized directory..."
    mkdir "$resized_directory"
fi

sounds_directory="${image_directory}/sounds"
if [[ ! -d "$sounds_directory" ]]; then
    echo "Creating sounds directory..."
    mkdir "$sounds_directory"
fi

# Loop through all image files in the given directory
for image_path in "$image_directory"/*.{jpg,jpeg,png,gif,JPG}; do
    # Check if the file exists to avoid processing invalid entries
    if [[ ! -e "$image_path" ]]; then
        continue
    fi

    # Extract filename without extension
    filename=$(basename "$image_path")
    filename_without_ext="${filename%.*}"

    echo "Processing $filename..."

    # Define the new filename for the resized image
     resized_image_path="${resized_directory}/${filename_without_ext}_resized.jpg"

    echo "Resizing $filename to 500px width..."

    # Resize the image to 500x500 pixels, cropping as necessary to maintain aspect ratio
    convert "$image_path" -resize 500x500^ -gravity center -crop 500x500+0+0 +repage "$resized_image_path"

    echo "Resized image saved as $resized_image_path"

    # Define the output filename for the sound file
    sound_output_path="${sounds_directory}/${filename_without_ext}.wav"

    # # Run the image-to-sound command
    image-to-sound "$resized_image_path" "$sound_output_path"
done

echo "Processing complete."