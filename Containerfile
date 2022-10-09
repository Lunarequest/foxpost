FROM docker.io/rust:alpine as build-env
WORKDIR /app
RUN apk add musl-dev pkgconf openssl-dev libpq-dev
COPY . /app
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --target=x86_64-unknown-linux-musl \
    && cp target/release/blog

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/blog /
COPY --from=build-env /app/templates /
CMD ["./blog"]
