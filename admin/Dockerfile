FROM rust:latest

RUN cargo install trunk

RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app

COPY ./ ./

RUN ["trunk", "build", "--release"]

CMD ["trunk", "serve", "--release"]
