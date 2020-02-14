FROM debian:stretch-20200130-slim

RUN apt-get update \
    && apt-get install -y curl python3-venv build-essential git=1:2.11.0-3+deb9u5

RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
