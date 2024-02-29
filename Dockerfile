# 
ARG RUST_VERSION=1.76.0
ARG APP_NAME=school-management-api
FROM rust:${RUST_VERSION}-buster AS build
ARG APP_NAME
# 
WORKDIR app
# 
RUN cargo new cached-deps
RUN cargo new --lib cached-deps/entity
RUN cargo new --lib cached-deps/migration
RUN cargo new --lib cached-deps/service
#
COPY ./Cargo.lock ./cached-deps/Cargo.lock
COPY ./Cargo.toml ./cached-deps/Cargo.toml
COPY ./entity/Cargo.toml ./cached-deps/entity/Cargo.toml
COPY ./migration/Cargo.toml ./cached-deps/migration/Cargo.toml
COPY ./service/Cargo.toml ./cached-deps/service/Cargo.toml
# 
WORKDIR cached-deps
# 
RUN cargo build --release
# 
WORKDIR app
# 
COPY . .
# 
RUN cargo build --workspace --locked --release
RUN cp ./target/release/$APP_NAME /app/server
RUN cp ./target/release/migration /app/migration
RUN cp ./start.sh /app/start.sh
RUN cp ./wait-for.sh /app/wait-for.sh
# 
FROM debian:buster-slim AS final
# 
WORKDIR /school-api
# 
RUN apt update
RUN apt install pkg-config openssl netcat -y
# Copy the executable from the "build" stage.
COPY --from=build /app/server . 
COPY --from=build /app/migration .
COPY --from=build /app/start.sh .
COPY --from=build /app/wait-for.sh .
RUN chmod +x ./start.sh
RUN chmod +x ./wait-for.sh
RUN sed -i -e 's/\r$//' start.sh
RUN sed -i -e 's/\r$//' wait-for.sh
# Expose the port that the school-apilication listens on.
EXPOSE 8080
CMD ["/school-api/server"]
ENTRYPOINT ["/school-api/start.sh"]
