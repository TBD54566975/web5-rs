FROM --platform=linux/amd64 alpine:latest

# Install system dependencies
RUN apk add --no-cache \
    build-base \
    musl-dev \
    linux-headers \
    rustup \
    libgcc \
    libstdc++ \
    curl \
    git \
    perl \
    make \
    bash

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --no-modify-path --default-toolchain 1.74.0
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the source code to the container
WORKDIR /usr/src/myapp
COPY Cargo.toml ./
COPY bindings/web5_uniffi_wrapper ./bindings/web5_uniffi_wrapper
COPY bindings/web5_uniffi ./bindings/web5_uniffi
COPY crates/http-std ./crates/http-std
COPY crates/web5 ./crates/web5
COPY crates/web5_cli ./crates/web5_cli

RUN cargo build --release --package web5_cli

# Set the entrypoint, so that we can `docker cp` the build output
CMD tail -f /dev/null