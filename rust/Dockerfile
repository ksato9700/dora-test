FROM rust:alpine AS builder
RUN apk add --no-cache \
    curl \
    ca-certificates \
    unzip \
    binutils \
    musl-dev
ADD https://raw.githubusercontent.com/dora-rs/dora/main/install.sh /install.sh
# ADD ./install.sh /install.sh
RUN SHELL=/bin/bash sh /install.sh && rm /install.sh
COPY . /work
WORKDIR /work
RUN cargo build --release
RUN strip /root/.dora/bin/dora

FROM alpine:3.20
COPY --from=builder /root/.dora /root/.dora
COPY --from=builder /work/target/release/listener_1 /opt/dora/bin/listener_1
COPY --from=builder /work/target/release/talker_1 /opt/dora/bin/talker_1
COPY --from=builder /work/target/release/talker_2 /opt/dora/bin/talker_2
COPY dataflow-prod.yml /opt/dora/etc/dataflow.yml
COPY entrypoint.sh /entrypoint.sh
ENV PATH="/root/.dora/bin/:$PATH"
CMD [ "/bin/sh", "/entrypoint.sh" ]
