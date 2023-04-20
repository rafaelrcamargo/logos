#!/bin/bash

# Set the directory containing the projects
projects_dir="./services/"

# Create an empty list to hold the commands and paths
commands=()

# Loop through each directory in the projects directory
for dir in "$projects_dir"/*/; do
  # Extract the project name from the directory name
  project_name=$(basename "$dir")

  # Filter out the utils directory
  if [ "$project_name" = "utils" ]; then
    continue
  fi

  # Add the command and path to the list
  commands+="'cargo watch -q -c -x \"run --bin ${project_name}\"' "
done

# Start all projects with concurrently
echo "pnpm concurrently ${commands[*]}"
