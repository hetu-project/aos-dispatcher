FROM rust:1.80.1-bullseye AS builder
WORKDIR /usr/src/app
COPY . ./
RUN cargo build --release



FROM debian:bullseye AS prod
RUN  apt-get update && apt-get -y install libpq5

WORKDIR /usr/src/app

COPY . ./

COPY --from=builder /usr/src/app/target/release/aos-dispatcher ./

CMD ["./aos-dispatcher"]