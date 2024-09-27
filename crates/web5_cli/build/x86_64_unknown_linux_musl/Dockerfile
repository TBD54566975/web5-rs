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
    bash \
    openssl-dev \
    openssl-libs-static # TODO remove above two once we remove reqwest

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --no-modify-path --default-toolchain 1.74.0
ENV PATH="/root/.cargo/bin:${PATH}"

# TODO remove once we remove reqwest
# Set environment variables to ensure vendored OpenSSL is used
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig
ENV LIBRARY_PATH="/usr/lib:/usr/local/lib"

# Copy the source code to the container
WORKDIR /usr/src/myapp
COPY Cargo.toml ./
COPY bindings/web5_c ./bindings/web5_c
COPY bindings/web5_uniffi_wrapper ./bindings/web5_uniffi_wrapper
COPY bindings/web5_uniffi ./bindings/web5_uniffi
COPY bindings/web5_wasm ./bindings/web5_wasm
COPY crates/http-std ./crates/http-std
COPY crates/web5 ./crates/web5
COPY crates/web5_cli ./crates/web5_cli

RUN cargo build --release --package web5_cli

# Set the entrypoint, so that we can `docker cp` the build output
CMD tail -f /dev/null