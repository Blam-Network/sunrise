# Use an official Rust image as the base
FROM rust:1.83.0-bullseye AS builder

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller image for the final stage
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/sunrise /app/

# Expose the port the application will run on
EXPOSE 8000

# Command to run the application
CMD ["/app/sunrise"]
