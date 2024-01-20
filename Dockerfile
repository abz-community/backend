FROM rust:latest as build

RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev librust-openssl-sys-dev
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/api-service
COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo install -—release —-target=x86_64-unknown-linux-musl

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/api-service /usr/local/bin/api-service

CMD ["api-service"]
