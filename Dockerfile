FROM rust:1.70-alpine3.18 as builder
RUN apk add musl-dev
WORKDIR /usr/src/rs_api
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo install --path .


FROM alpine:3.18
COPY --from=builder /usr/local/cargo/bin/rs_api /usr/local/bin/rs_api
EXPOSE 8080
CMD ["rs_api"]