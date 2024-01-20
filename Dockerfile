FROM rust:latest as builder

RUN apt-get update && apt-get install -y musl-tools ca-certificates cmake musl-dev pkg-config libssl-dev librust-openssl-sys-dev
COPY . .
RUN rustup update
RUN rustup target add x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.8
RUN apk add openssl-dev
RUN apk --no-cache add ca-certificates
COPY --from=builder /target/x86_64-unknown-linux-musl/release/backend .
EXPOSE 8080
CMD ["/backend"]
