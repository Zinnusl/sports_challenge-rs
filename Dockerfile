FROM debian:buster-slim
FROM rust:latest

# compile the project
WORKDIR /usr/src
ADD . /usr/src

# install libzmq & libprotobuf
RUN apt-get update && apt-get install -y \
    apt-utils \
    libzmq3-dev \
    libprotobuf-dev \
    protobuf-compiler

RUN cargo build --release

EXPOSE 36601

ENV binary client

CMD ["sh", "-c", "cargo run --release --bin ${binary}"]
