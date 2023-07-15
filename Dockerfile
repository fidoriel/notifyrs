FROM rust:1.70-alpine3.18 as builder
RUN apk add musl-dev pkgconfig openssl-dev
WORKDIR /usr/src/notifyrs
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo install --path .


FROM alpine:3.18
COPY --from=builder /usr/local/cargo/bin/notifyrs /usr/local/bin/notifyrs
EXPOSE 8080
CMD ["notifyrs"]