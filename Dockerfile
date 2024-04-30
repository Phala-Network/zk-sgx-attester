FROM --platform=linux/amd64 ubuntu:22.04

# Update default packages
RUN apt-get -qq update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl git

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash
# Add .foundry/bin to PATH
ENV PATH="/root/.foundry/bin:${PATH}"
RUN foundryup

# Install RiscZero
RUN cargo install cargo-binstall
RUN cargo binstall cargo-risczero -y
RUN cargo risczero install

## Build ZK DCAP Verifier
WORKDIR /code
COPY . .
RUN git submodule sync && git submodule update --init --recursive
RUN cargo build --release

ENTRYPOINT ["./target/release/publisher"]