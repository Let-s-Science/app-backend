FROM rust:latest AS builder

RUN update-ca-certificates

WORKDIR /backend

COPY ./ .

RUN SQLX_OFFLINE=true cargo build --release

FROM debian:buster-slim

WORKDIR /backend

# Copy our build
COPY --from=builder /backend/target/release/letsscience-backend ./

ENV RUST_LOG=trace

CMD ["/backend/letsscience-backend"]