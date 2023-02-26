FROM rust:1.66.1-slim as cache

RUN cargo install cargo-watch
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
WORKDIR /usr/src/temp/app
RUN cargo fetch
RUN mkdir -p src //creates a src folder for a mock app
RUN echo "fn main() {}" > src/main.rs //creates a main.rs file for a mock app
RUN echo "fn main() {}" > src/lib.rs //creates a lib.rs file for a mock app
RUN cargo install --path .
RUN rm -rf ./src target/release/deps/zero2prod* target/release/zero2prod* //removes the mock app

FROM rust:1.66.1-slim as build
WORKDIR /usr/src/app
COPY --from=cache /usr/src/temp/app/target target
COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/release/zero2prod /usr/local/bin/zero2prod

EXPOSE ${SERVER_PORT}

CMD ["zero2prod"]