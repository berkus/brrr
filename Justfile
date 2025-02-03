# Run the binary by default
_default: run

# Run the binary
run:
    cargo run --bin carma --release

# Build binary
build:
    cargo build --release

# Clean build
clean:
    cargo clean

# Print list of commands
help:
    @just --list

# Convert the pixmaps to png
convert:
    cargo run --bin convert_to_png --release
