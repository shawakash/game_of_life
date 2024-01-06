# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to leverage Docker layer caching
COPY ./Cargo.toml ./Cargo.lock ./

# Build a minimal Rust project to cache dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src target

# Copy the entire source code
COPY ./ .

# Build the actual Rust application
RUN cargo build --release

# Create a new image from a smaller base image
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy only the built binary from the previous stage
COPY --from=builder /app/target/release/game_of_life .

# Expose any necessary ports
EXPOSE 8080

# Define the command to run your application
CMD ["./game_of_life"]
