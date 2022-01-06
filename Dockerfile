FROM rust:1.57-slim-bullseye AS builder
RUN apt update && \
  apt install -y libssl-dev pkg-config
WORKDIR /usr/src/mate
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt update && \
  apt install -y ca-certificates
COPY --from=builder /usr/local/cargo/bin/mate /usr/local/bin/mate
COPY --from=builder /usr/local/cargo/bin/mate-collector /usr/local/bin/mate-collector
COPY --from=builder /usr/local/cargo/bin/mate-api /usr/local/bin/mate-api
CMD ["mate"]
