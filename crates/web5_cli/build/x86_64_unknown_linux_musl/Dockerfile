FROM --platform=linux/amd64 alpine:latest

# Install system dependencies
RUN apk add --no-cache \
    build-base \
    musl-dev \
    openssl-dev \
    linux-headers \
    rustup \
    libgcc \
    libstdc++ \
    curl \
    git \
    perl \
    make \
    bash \
    openssl-libs-static

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --no-modify-path --default-toolchain 1.74.0
ENV PATH="/root/.cargo/bin:${PATH}"

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
COPY bindings/web5_uniffi_wrapper ./bindings/web5_uniffi_wrapper
COPY bindings/web5_uniffi ./bindings/web5_uniffi
COPY crates/web5 ./crates/web5
COPY crates/web5_cli ./crates/web5_cli

# Build the static lib (override the lib type)
RUN sed -i 's/crate-type = \["cdylib"\]/crate-type = \["staticlib"\]/' bindings/web5_uniffi/Cargo.toml

RUN cargo build --release --package web5_uniffi

# Compile as a dynamic lib (.so) from our static lib (.a) while keeping dependencies self-contained
RUN gcc -shared -o target/release/libweb5_uniffi.so -Wl,--whole-archive \
    target/release/libweb5_uniffi.a -Wl,--no-whole-archive -static-libgcc \
    -L/usr/lib -lssl -lcrypto -Wl,-Bdynamic -fPIC

# Set the entrypoint, so that we can `docker cp` the build output
CMD tail -f /dev/null
