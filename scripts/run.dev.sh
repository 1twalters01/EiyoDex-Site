#!/bin/bash

# Run Rust proxy in background
cargo run --bin proxy &
RUST_PID=$!

# Function to clean up background process on exit
cleanup() {
  kill $RUST_PID
  wait $RUST_PID 2>/dev/null
}

# Trap SIGINT and SIGTERM (Ctrl+C) to call cleanup
trap cleanup SIGINT SIGTERM

# Run npm dev in foreground
npm run dev

# After npm run dev exits, cleanup Rust proxy
cleanup

