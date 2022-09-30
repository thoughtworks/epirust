FROM debian:stable-slim AS chef
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

RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:stable-slim AS pre-builder
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    libfreetype6-dev libfontconfig1-dev

FROM gcr.io/distroless/cc-debian11:nonroot
COPY --from=pre-builder /lib/x86_64-linux-gnu/ /lib/x86_64-linux-gnu/
COPY --from=builder --chown=nonroot:nonroot /app/target/release/epirust-orchestrator app/
COPY --from=builder --chown=nonroot:nonroot /app/orchestrator/config app/config/
WORKDIR app
