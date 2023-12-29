FROM rust:1.75 as builder
WORKDIR /app
COPY Cargo.toml /app/
COPY src /app/src/
RUN cargo build --bin ntnu-timeplan-api --release

FROM debian:bullseye-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/ntnu-timeplan-api /app/ntnu-timeplan-api
ENTRYPOINT ["/app/ntnu-timeplan-api"]
