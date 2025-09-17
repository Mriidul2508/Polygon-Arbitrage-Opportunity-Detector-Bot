# Use the official Rust image as a base
FROM rust:1-slim-buster AS builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build the application in release mode
RUN cargo build --release

# --- Final Stage ---
# Use a minimal image for the final container
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/polygon-arbitrage-bot .

# Copy configuration and ABI files
COPY config ./config
COPY src/abi ./src/abi

# Set the command to run the bot when the container starts
CMD ["./polygon-arbitrage-bot"]
