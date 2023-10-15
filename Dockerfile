# Stage 1: Build the application
# ---

FROM rust:1 AS builder
WORKDIR /app
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml

RUN apt-get -y update && apt-get -y upgrade
RUN cargo build --release

# Stage 2: Build the final container
# ---

FROM ubuntu:latest as runner
WORKDIR /app
COPY ./Rocket.toml ./Rocket.toml
COPY --from=builder /app/target/release/compare_steam_games /usr/local/bin/compare_steam_games
ENTRYPOINT [ "compare_steam_games" ]