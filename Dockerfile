FROM rust:1.57-slim-bullseye AS builder
WORKDIR /usr/src/mate
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/mate /usr/local/bin/mate
CMD ["mate"]
