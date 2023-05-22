# Set the base image to Rust's official image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Rust project files into the container
COPY . .

# Build the Rust project
RUN cargo build --release --example tapo_p110

# Set the default command to run the built binary
CMD ["./target/release/examples/tapo_p110"]
