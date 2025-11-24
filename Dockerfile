FROM rust:alpine3.20 AS chef
RUN apk add --no-cache musl-dev gcc openssl-dev openssl-libs-static pkgconfig
# Install nightly toolchain
RUN rustup toolchain install nightly && \
    rustup default nightly
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build app
COPY . .
RUN cargo build --release
RUN strip target/release/ferrumc

# Minimal runtime
FROM alpine:3.20
WORKDIR /app
RUN addgroup -S ferrumc && adduser -S ferrumc -G ferrumc
COPY --from=builder /app/target/release/ferrumc /app/
RUN chown -R ferrumc:ferrumc /app

# leaving this out for now since we dont have world loading
# Once we get the website working, we should put the file on our own server, instead of a contributors side project.
# RUN wget --progress=dot:giga https://aboleth.ai/static/ferumc-121-exampleworld.tar.gz -P /app
# RUN mkdir -p /app/import && \
#   tar -xzvf /app/ferumc-121-exampleworld.tar.gz -C /app/import
# RUN ./ferrumc --import
# RUN rm /app/ferumc-121-exampleworld.tar.gz && \
#   rm -rf /app/import

USER ferrumc
EXPOSE 25565
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s \
  CMD nc -z localhost 25565 || exit 1
CMD ["./ferrumc"]
