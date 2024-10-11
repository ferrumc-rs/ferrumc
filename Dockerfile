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
COPY --from=build-stage /app/target/release/ferrumc /app/
COPY --from=build-stage /app/assets/example_world/* /app/import/
RUN ./ferrumc --import 
EXPOSE 25565
CMD ["./ferrumc"]


