ARG RUST_VERSION=1.76.0
ARG APP_NAME=school-management-api

FROM rust:${RUST_VERSION}-slim-buster AS build
ARG APP_NAME

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .

RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --workspace --exclude entity --exclude service --locked --release && \
    cp ./target/release/$APP_NAME /bin/server && \
    cp ./target/release/migration /bin/migration && \
    cp ./start.sh /bin/start.sh && \
    cp ./wait-for.sh /bin/wait-for.sh

FROM debian:buster-slim AS final

WORKDIR /school-api

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl1.1 netcat && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /bin/server .
COPY --from=build /bin/migration .
COPY --from=build /bin/start.sh .
COPY --from=build /bin/wait-for.sh .

RUN chmod +x ./server ./migration ./start.sh ./wait-for.sh && \
    sed -i -e 's/\r$//' start.sh wait-for.sh

EXPOSE 8080

ENTRYPOINT ["/school-api/start.sh"]

CMD ["/school-api/server"]