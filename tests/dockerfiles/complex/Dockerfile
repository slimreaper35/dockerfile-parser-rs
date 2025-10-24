FROM --platform=linux/amd64 docker.io/library/rust:1.70-slim-buster AS builder

# Set up metadata
LABEL maintainer="John Doe" version="1.0"

ARG CARGO_HOME=/usr/local/cargo
ARG RUSTUP_HOME=/usr/local/rustup

# Ensure Cargo bin is in PATH and enable full backtraces
ENV PATH="${CARGO_HOME}/bin:${PATH}" RUST_BACKTRACE="full"

# Use strict shell options for safety
SHELL ["bash", "-euxo", "pipefail", "-c"]

# Install stable toolchain and cargo-chef for caching
RUN rustup default stable
RUN cargo install cargo-chef --locked

WORKDIR /app

RUN <<EOF
cat > .cargo/config.toml <<CONFIG
[alias]
b = "build"
c = "check"
t = "test"
r = "run"
CONFIG
EOF

# Copy helper scripts
COPY scripts/ /app/scripts/

ADD https://raw.githubusercontent.com/rust-lang/rust/master/README.md /app/README.md

# Make scripts executable
RUN chmod +x /app/scripts/install-deps.sh

# Cache dependency resolution
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

# Copy source and build
COPY . ./
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo build --release --locked

FROM --platform=linux/amd64 docker.io/library/debian:stable-slim

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/myapp /app/myapp

# Default runtime environment
ENV RUST_ENV="production" RUST_LOG="info" TZ="UTC"

# Use lightweight shell
SHELL ["sh", "-c"]

# Persist data and logs
VOLUME ["/app/data", "/app/logs"]

# Expose application ports
EXPOSE 8080/tcp 8080/udp

# Graceful shutdown signal
STOPSIGNAL SIGINT

# Run as non-root user
USER 1001

# Entrypoint and default command
ENTRYPOINT ["./myapp"]
CMD ["--serve", "--config", "/app/config.toml"]
