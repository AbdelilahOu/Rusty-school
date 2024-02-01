ARG RUST_VERSION=1.73.0
FROM rust:${RUST_VERSION} AS build
WORKDIR /app

COPY . .

RUN cargo build --workspace --locked --release

FROM alpine:latest AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

WORKDIR /app

# Copy the executable from the "build" stage.
COPY --from=build /app/target/release/school-management-api .

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["/app/server"]
