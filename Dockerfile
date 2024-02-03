# 
ARG RUST_VERSION=1.73.0
ARG APP_NAME=school-management-api
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app
# 
COPY . .
# 
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server
# 
FROM debian:bullseye-slim AS final
# 
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

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["/bin/server"]
