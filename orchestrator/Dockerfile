FROM debian:stable-slim

USER root
#system dependencies for crates that engine uses
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    ca-certificates curl file \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev cmake pkg-config \
    libfreetype6-dev libfontconfig1-dev xclip

# install toolchain
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH

RUN mkdir orchestrator
COPY . ./orchestrator
WORKDIR ./orchestrator

RUN cargo build --release
