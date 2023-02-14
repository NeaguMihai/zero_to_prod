FROM rust:1.66.1-slim as builder

WORKDIR /usr/src/app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo install --path .

COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/release/zero2prod /usr/local/bin/zero2prod

EXPOSE ${SERVER_PORT}

CMD ["zero2prod"]