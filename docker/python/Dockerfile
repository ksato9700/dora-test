FROM python:3.12-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    unzip
ADD https://raw.githubusercontent.com/dora-rs/dora/main/install.sh /install.sh
# ADD ./install.sh /install.sh
RUN SHELL=/bin/bash sh /install.sh && rm /install.sh
ENV PATH="/root/.dora/bin/:$PATH"
RUN pip install dora-rs
ADD entrypoint.sh /entrypoint.sh
WORKDIR /work
CMD [ "/bin/sh", "/entrypoint.sh" ]
