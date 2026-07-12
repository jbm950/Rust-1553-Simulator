#!/usr/bin/env bash

set -euo pipefail

command -v tmux >/dev/null || {
    echo "tmux is required."
    exit 1
}

command -v cargo >/dev/null || {
    echo "cargo is required."
    exit 1
}

SESSION="1553-Sim"
MINIMALPROMPT="export PS1='$ '; clear"
ENTER="C-m"

# Only create tmux session if it doesn't already exist
if ! tmux has-session -t "$SESSION" 2>/dev/null; then
    tmux new-session -d -s "$SESSION" -n 'Bus/RTs' -x "$(tput cols)" -y "$(tput lines)"

    # Turn on border titles for the whole session
    tmux set-option -t "$SESSION" pane-border-status top
    tmux set-option -t "$SESSION" pane-border-format "#{pane_title}"

    # Pane: Bus
    tmux select-pane -t "$SESSION" -T "Bus"
    tmux send-keys -t "$SESSION" "$MINIMALPROMPT" $ENTER
    tmux send-keys -t "$SESSION" "cargo run --bin bus" $ENTER

    # Pane: GPS RT
    tmux split-window -t "$SESSION" -h
    tmux select-pane -t "$SESSION" -T "GPS RT"
    tmux send-keys -t "$SESSION" "$MINIMALPROMPT" $ENTER
    tmux send-keys -t "$SESSION" "cargo run --bin gps -- 13" $ENTER

    # Pane: Power RT
    tmux split-window -t "$SESSION" -v
    tmux select-pane -t "$SESSION" -T "Power RT"
    tmux send-keys -t "$SESSION" "$MINIMALPROMPT" $ENTER
    tmux send-keys -t "$SESSION" "cargo run --bin power -- 5" $ENTER

    # Window: Bus Controller
    tmux new-window -t "$SESSION" -n "BC"
    tmux select-pane -t "$SESSION" -T "Bus Controller - press 'q' to exit TUI"
    tmux send-keys -t "$SESSION" "$MINIMALPROMPT" $ENTER
    tmux send-keys -t "$SESSION" "cargo run --bin bus_controller" $ENTER
fi

if [[ "${1:-}" != "--no-attach" ]]; then
    tmux attach-session -t "$SESSION"
fi
