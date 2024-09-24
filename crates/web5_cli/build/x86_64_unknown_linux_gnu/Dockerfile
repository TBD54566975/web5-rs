FROM --platform=linux/amd64 ubuntu:22.04

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    libssl-dev \
    pkg-config

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the source code to the container
WORKDIR /usr/src/myapp
COPY Cargo.toml ./
COPY bindings/web5_uniffi_wrapper ./bindings/web5_uniffi_wrapper
COPY bindings/web5_uniffi ./bindings/web5_uniffi
COPY bindings/web5_wasm ./bindings/web5_wasm
COPY crates/http-std ./crates/http-std
COPY crates/web5 ./crates/web5
COPY crates/web5_cli ./crates/web5_cli

# Execute the build
RUN cargo build --release --package web5_cli

# Set the entrypoint, so that we can `docker cp` the build output
CMD tail -f /dev/null