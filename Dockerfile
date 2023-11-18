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

RUN cp /usr/src/target/release/client /usr/local/bin/client
RUN cp /usr/src/target/release/server /usr/local/bin/server

EXPOSE 36601

# CMD ["client"]
ENTRYPOINT ["server"]
