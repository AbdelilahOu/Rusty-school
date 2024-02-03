# BUILD stage
FROM rust:1.73.0 AS Build
# 
RUN cargo new --bin school-management-api
# 
WORKDIR /school-management-api
# 
COPY Cargo.toml Cargo.lock ./
# 
RUN cargo new --lib migration
RUN cargo new --lib entity
RUN cargo new --lib service
# Copy only the necessary files for dependency resolution
COPY migration/Cargo.toml ./migration/Cargo.toml
COPY entity/Cargo.toml ./entity/Cargo.toml
COPY service/Cargo.toml ./service/Cargo.toml
# 
RUN cargo build --workspace --release
# 
RUN rm src/*.rs
RUN rm migration/src/*.rs
RUN rm entity/src/*.rs
RUN rm service/src/*.rs
# 
COPY ./src ./src
COPY ./migration/src ./migration/src
COPY ./entity/src ./entity/src
COPY ./service/src ./service/src
# 
RUN rm migration/src/main.rs
# 
# RUN rm ./target/release/deps/school-management-api
RUN cargo build --workspace --release
# 
FROM rust:1.73.0-slim-buster AS Final
# 
WORKDIR /server
# 
COPY --from=build /school-management-api/target/release/school-management-api .
# 
CMD ["./school-management-api.d"]




















# set wdir
# WORKDIR /app

# Build the application
# RUN cargo build --workspace --release

# check for target folder
# RUN ls -la

# RUN stage
# FROM alpine:latest AS final

# set wdir
# WORKDIR /server

# Copy the executable from the "build" stage
# COPY --from=build /app/target ./
# COPY .env ./release/

# Expose the port that the application listens on
# EXPOSE 8080

# Set the command to run the executable
# CMD ["./release/school-management-api.d"]
