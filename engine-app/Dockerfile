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
    sh -s -- --default-toolchain nightly -y

ENV PATH=/root/.cargo/bin:$PATH

RUN cargo +nightly -Z sparse-registry install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo +nightly chef cook -Z sparse-registry --release --recipe-path recipe.json
COPY . .
RUN cargo +nightly -Z sparse-registry build --release

FROM debian:stable-slim AS pre-builder
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    libfreetype6-dev libfontconfig1-dev jq

FROM gcr.io/distroless/cc-debian11:nonroot
COPY --from=pre-builder /usr/lib/x86_64-linux-gnu/ /usr/lib/x86_64-linux-gnu/
COPY --from=pre-builder /lib/x86_64-linux-gnu/ /lib/x86_64-linux-gnu/
COPY --from=pre-builder /usr/bin/jq /usr/bin/
COPY --from=pre-builder /bin/chown /bin/
COPY --from=pre-builder /bin/chmod /bin/
COPY --from=pre-builder /usr/bin/xargs /usr/bin/
COPY --from=pre-builder /bin/ls /bin/
COPY --from=pre-builder /bin/sh /bin/
COPY --from=builder --chown=nonroot:nonroot /app/target/release/engine-app app/epirust-engine
COPY --from=builder --chown=nonroot:nonroot /app/engine/config app/config/
WORKDIR app/
