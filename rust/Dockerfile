FROM rust:alpine AS builder
RUN apk add --no-cache \
    curl \
    ca-certificates \
    unzip \
    binutils \
    musl-dev
RUN cargo install dora-cli
COPY . /work
WORKDIR /work
RUN cargo build --release
# RUN strip /root/.dora/bin/dora

FROM alpine:3.20
COPY --from=builder /usr/local/cargo/bin/dora /root/.dora/bin/dora
COPY --from=builder /work/target/release/listener_1 /opt/dora/bin/listener_1
COPY --from=builder /work/target/release/talker_1 /opt/dora/bin/talker_1
COPY --from=builder /work/target/release/talker_2 /opt/dora/bin/talker_2
COPY dataflow-prod.yml /opt/dora/etc/dataflow.yml
COPY entrypoint.sh /entrypoint.sh
ENV PATH="/root/.dora/bin/:$PATH"
CMD [ "/bin/sh", "/entrypoint.sh" ]
