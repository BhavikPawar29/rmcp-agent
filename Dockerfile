FROM rust:1.85-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rmcp-agent /usr/local/bin/bhavik-rmcp
RUN touch /app/production-crash.log
CMD ["bhavik-rmcp"]