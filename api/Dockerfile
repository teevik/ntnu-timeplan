FROM rust:1.75-alpine as builder
WORKDIR /app
COPY Cargo.toml /app/
COPY Cargo.lock /app/
COPY src /app/src/
RUN apk add --no-cache musl-dev
RUN cargo build --bin ntnu-timeplan-api --release 

FROM scratch as runtime
WORKDIR /app
COPY --from=builder /app/target/release/ntnu-timeplan-api /app/ntnu-timeplan-api
ENTRYPOINT ["/app/ntnu-timeplan-api"]
