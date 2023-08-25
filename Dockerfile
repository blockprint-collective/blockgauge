FROM rust:bullseye AS builder
ADD . /app/
WORKDIR app
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/blockgauge /usr/local/bin/
WORKDIR /blockgauge
