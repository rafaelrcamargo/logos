#!/bin/bash

# create new tmux session
tmux new-session -d -s my_project

# get list of subdirectories in services folder (excluding utils)
subdirs=(services/*/)

# create a new pane for each remaining subdirectory and start cargo watch
for dir in "${subdirs[@]}"; do
  if [[ "$dir" == "services/utils/" ]]; then
    continue
  fi

  project_name=$(basename "$dir")
  tmux split-window -h "cargo watch -q -c -x \"run --bin ${project_name}\""\; select-layout tiled
done

# attach to the tmux session
tmux select-pane -t 1
# delete the first pane
tmux kill-pane -t 0

tmux attach-session -t my_project
