FROM public.ecr.aws/docker/library/rust:slim-buster

ENV WORKDIR /usr/src/myapp
WORKDIR ${WORKDIR}

RUN apt-get update && apt-get install -y zip musl-tools make perl
RUN rustup target add x86_64-unknown-linux-musl

# Create blank project where we first install deps and cache that layer
RUN cargo init

# We want dependencies cached, so copy those first.
COPY ./lambda/Cargo.toml /usr/src/myapp/

RUN cargo build --release --target x86_64-unknown-linux-musl

# Then build the app code
COPY ./lambda/src/ /usr/src/myapp/src

RUN touch /usr/src/myapp/src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl


RUN cp ${WORKDIR}/target/x86_64-unknown-linux-musl/release/edge-creds-pusher bootstrap
RUN zip function.zip bootstrap

