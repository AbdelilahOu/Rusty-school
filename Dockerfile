# 
ARG RUST_VERSION=1.76.0
ARG APP_NAME=school-management-api
FROM rust:${RUST_VERSION}-buster AS build
ARG APP_NAME
WORKDIR /app
# 
COPY . .
# 
RUN cargo build --workspace --locked --release
RUN cp ./target/release/$APP_NAME /bin/server
# 
FROM debian:buster-slim AS final
# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/
# RUN apt update
# RUN apt install pkg-config libssl-dev -y
RUN apt update
RUN apt install pkg-config openssl -y
# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["/bin/server"]
