FROM --platform=$BUILDPLATFORM rust:1.57 AS builder
ARG TARGETPLATFORM
WORKDIR /usr/src/mate
COPY . .
RUN case "$TARGETPLATFORM" in \
      "linux/arm/v7") rustup armv7-unknown-linux-musleabihf ;; \
      *) continue ;; \
    esac
RUN rustup target add $(cat /rust_target.txt)
RUN cargo install --path .

FROM --platform=$BUILDPLATFORM alpine:3.14
COPY --from=builder /usr/local/cargo/bin/mate /usr/local/bin/mate
CMD ["mate"]
