# builder
ARG RUST_VERSION=1.75.0

FROM rust:${RUST_VERSION}-buster AS builder

WORKDIR /usr/src/app

COPY src /usr/src/app/src
COPY .sqlx /usr/src/app/.sqlx
COPY migrations /usr/src/app/migrations
COPY build.rs /usr/src/app/build.rs
COPY Cargo.toml /usr/src/app/Cargo.toml


RUN SQLX_OFFLINE=true cargo build --release


RUN mv /usr/src/app/target/release/nwc-api . && \
    rm -rf target


RUN apt update && apt install wget -y && \
    apt-get -s dist-upgrade | grep "^Inst" | grep -i securi | awk -F " " {'print $2'} | xargs apt-get install

RUN cargo install sqlx-cli

COPY entrypoint.sh entrypoint.sh
RUN chmod +x entrypoint.sh
ENTRYPOINT ["./entrypoint.sh"]

