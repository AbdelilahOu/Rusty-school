# 
ARG RUST_VERSION=1.76.0
ARG APP_NAME=school-management-api
FROM rust:${RUST_VERSION}-buster AS build
ARG APP_NAME
# 
WORKDIR /app
# BUILD THE APP
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=entity,target=entity \
    --mount=type=bind,source=migration,target=migration \
    --mount=type=bind,source=service,target=service \
    --mount=type=bind,source=start.sh,target=start.sh \
    --mount=type=bind,source=wait-for.sh,target=wait-for.sh \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server && \
cp ./target/release/migration /bin/migration && \
cp ./start.sh /bin/start.sh && \
cp ./wait-for.sh /bin/wait-for.sh
# 
FROM debian:buster-slim AS final
# 
WORKDIR /school-api
# 
RUN apt update
RUN apt install pkg-config openssl netcat -y
# Copy the executable from the "build" stage.
COPY --from=build /bin/server . 
COPY --from=build /bin/migration .
COPY --from=build /bin/start.sh .
COPY --from=build /bin/wait-for.sh .
# 
RUN chmod +x ./start.sh
RUN chmod +x ./wait-for.sh
#
RUN sed -i -e 's/\r$//' start.sh
RUN sed -i -e 's/\r$//' wait-for.sh
# Expose the port that the school-apilication listens on.
EXPOSE 8080
CMD ["/school-api/server"]
ENTRYPOINT ["/school-api/start.sh"]
