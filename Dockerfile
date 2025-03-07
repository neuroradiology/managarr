FROM rust:1.85 AS builder
WORKDIR /usr/src

# Download and compile Rust dependencies in an empty project and cache as a separate Docker layer
RUN USER=root cargo new --bin managarr-temp

WORKDIR /usr/src/managarr-temp
COPY Cargo.* .
RUN cargo build --release
# remove src from empty project
RUN rm -r src
COPY src ./src
# remove previous deps
RUN rm ./target/release/deps/managarr*

RUN --mount=type=cache,target=/volume/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release --bin managarr
RUN mv target/release/managarr .

FROM debian:stable-slim

# Copy the compiled binary from the builder container
COPY --from=builder --chown=nonroot:nonroot /usr/src/managarr-temp/managarr /usr/local/bin

ENTRYPOINT [ "/usr/local/bin/managarr" ]
