FROM rust:slim-bullseye AS builder
WORKDIR /usr/src/mate
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/mate /usr/local/bin/mate
CMD ["mate"]
