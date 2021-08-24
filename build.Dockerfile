FROM rust:latest

ARG UID
ARG GID
RUN \
  groupadd -o -g "$GID" host && \
  useradd -o -u "$UID" -g "$GID" -s /bin/bash -m dev

USER dev

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-pack

