FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY ./ ./

COPY ./.ssh /root/.ssh

RUN chmod 600 ~/.ssh/id_rsa
RUN chmod 600 ~/.ssh/id_rsa.pub

RUN ssh-keyscan -t rsa git.dealtech.com > ~/.ssh/known_hosts

RUN cargo build --release


FROM debian:bullseye-slim

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/alma ./

CMD ["./alma"]
