FROM gocd/gocd-agent-debian-10:v19.12.0
# install toolchain
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

ENV PATH=/home/go/.cargo/bin:$PATH
