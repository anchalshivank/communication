#!/bin/bash

DIRECTORY="./"
OUTPUT_FILE="project_structure.txt"  # Specify the output file

# Find command with exclusions
result=$(find "$DIRECTORY" \
    \( -type d \( -name "yagna-builds" -o -name ".git" -o -name ".github" -o -name "node_modules" -o -name "test" -o -name "tests" -o -name "target" \) -prune \) -o \
    \( -path "$DIRECTORY/static" -prune \) -o \
    \( -type f ! -name "*.pyc" ! -name "*.log" ! -name ".DS_Store" ! -iname "*license*" \
    ! -iname "*.jpg" ! -iname "*.db" ! -iname "*.png" ! -iname "*.blend" ! -iname "*.gif" \
    ! -iname "*.toml" ! -iname "*.jpeg" ! -iname "*.svg" ! -iname "*.db-shm" ! -iname "*.bmp" \
    ! -iname "*.base64" ! -iname "*.key" ! -iname "*.pem" ! -iname "*.pub" ! -iname "*.pdf" ! -iname "*.der" ! -iname "*.dir" -print \))

# Prepare output
output="===== Project Directory Structure =====\n"

# First, capture only directories for the separate section
directories=$(find "$DIRECTORY" -type d \
    \( -name "yagna-builds" -o -name ".git" -o -name ".github" -o -name "node_modules" -o -name "test" -o -name "tests" -o -name "target" \) -prune -o -print)

# Add directories to output
while IFS= read -r dir; do
    output+="$dir\n"
done <<< "$directories"

# Now add the files and their contents
output+="\n===== Project File Contents =====\n"

while IFS= read -r path; do
    if [ -f "$path" ]; then
        output+="File: $path\n"

        # Use 'cat' with '2>/dev/null' to suppress errors from files with null bytes
        file_content=$(cat "$path" 2>/dev/null)

        # Check if file_content is empty or null, then add accordingly
        if [ -z "$file_content" ]; then
            output+="(Empty or binary file)\n"
        else
            output+="$file_content\n"
        fi
        output+="----------------\n"
    fi
done <<<"$result"

# Save output to a text file
echo -e "$output" > "$OUTPUT_FILE"
echo "Directory and file structure, including content (with specific exclusions), has been saved to $OUTPUT_FILE."
