FROM docker.io/rust:1-buster
WORKDIR /app
COPY . /app
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release  
RUN --mount=type=cache,target=/app/target cp /app/target/release/blog /usr/bin/
RUN chmod -R +r static templates
WORKDIR /
CMD ["blog"]
