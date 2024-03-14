# See https://www.docker.com/blog/simplify-your-deployments-using-the-rust-official-image/

# Use official Rust image
FROM rust:1.61.0

# Set working directory
WORKDIR /usr/src/rustcached

# Copy all files locally to image
COPY ./ ./

# Need to install any dependencies below if needed.
# Leaving blank for now. I assume we will need to install
# some networking deps that are required at runtime.

# Build application
RUN cargo build --release

# Run the server binary. It is found in /target/release
CMD ["./target/release/server"]