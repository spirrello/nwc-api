# builder
ARG RUST_VERSION=1.74.0

FROM rust:${RUST_VERSION}-buster AS builder

COPY . /usr/src/app
WORKDIR /usr/src/app

RUN cargo build --release

FROM public.ecr.aws/ubuntu/ubuntu:latest
# WORKDIR /var/task

# This is a required dependecy.
ENV LIBSSL_FILE=libssl1.1_1.1.1f-1ubuntu2.22_amd64.deb
RUN apt update && apt install wget -y && \
    wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/${LIBSSL_FILE} && \
    dpkg -i ${LIBSSL_FILE} && rm ${LIBSSL_FILE} && \
    apt-get -s dist-upgrade | grep "^Inst" | grep -i securi | awk -F " " {'print $2'} | xargs apt-get install

COPY --from=builder /usr/src/app/target/release/nwc-api /usr/local/bin/nwc-api
COPY entrypoint.sh entrypoint.sh
RUN chmod +x entrypoint.sh
# ENTRYPOINT [ "DATABASE_URL=$DATABASE_URL", "/usr/local/bin/nwc-api" ]
ENTRYPOINT ["./entrypoint.sh"]

