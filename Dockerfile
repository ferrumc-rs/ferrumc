FROM rust:alpine3.20 AS build-stage 
# Install build tools (e.g., Maven, Gradle)
# Copy source code
# Build commands (e.g., compile, package)

WORKDIR /app
COPY . /app
RUN cargo build --release
# Stage 2: Runtime environment
FROM alpine:3.20 AS final-stage 
WORKDIR /app

WORKDIR /app
RUN apk add --no-cache wget
COPY --from=build-stage /app/target/release/ferrumc /app/
# Once we get the website working, we should put the file on our own server, instead of a contributors side project.
RUN wget --progress=dot:giga https://aboleth.ai/static/ferumc-121-exampleworld.tar.gz -P /app 
RUN mkdir -p /app/import && \
  tar -xzvf /app/ferumc-121-exampleworld.tar.gz -C /app/import
RUN ./ferrumc --import 
RUN rm /app/ferumc-121-exampleworld.tar.gz && \
  rm -rf /app/import

EXPOSE 25565
CMD ["./ferrumc"]
