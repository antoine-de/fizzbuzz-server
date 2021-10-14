FROM rust:latest as builder

RUN apt-get update && apt-get -y install musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1
ADD Cargo.toml Cargo.lock ./
ADD src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip /target/x86_64-unknown-linux-musl/release/fizzbuzz_server

# multi stage build to get an image with only one executable (build with musle for this)
FROM scratch
COPY --from=builder /target/x86_64-unknown-linux-musl/release/fizzbuzz_server fizzbuzz_server
ENTRYPOINT ["/fizzbuzz_server"]
