FROM rust:alpine AS builder

# Install build dependencies for compiling sqlite C code
RUN apk add --no-cache musl-dev gcc make pkgconfig

WORKDIR /usr/src/app

# Copy the generated library and the workspace
COPY generate-lib/ /usr/src/app/generate-lib/
COPY rust-workspace/ /usr/src/app/rust-workspace/

# Build statically using musl
WORKDIR /usr/src/app/rust-workspace
RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Create the minimal runtime image
FROM scratch

# Copy the statically linked binary from the builder
COPY --from=builder /usr/src/app/rust-workspace/target/x86_64-unknown-linux-musl/release/fifa-world-cup-2026-service-workspace /app

# Run the binary by default
ENTRYPOINT ["/app"]
