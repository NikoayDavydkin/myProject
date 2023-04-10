FROM lukemathwalker/cargo-chef:latest AS chef

WORKDIR /usr/src/app


FROM chef AS planner

COPY ./ ./

RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder

WORKDIR /usr/src/app

COPY ./.ssh /root/.ssh

RUN chmod 600 ~/.ssh/id_rsa
RUN chmod 600 ~/.ssh/id_rsa.pub

RUN ssh-keyscan -t rsa git.dealtech.com > ~/.ssh/known_hosts

COPY --from=planner /usr/src/app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./ ./

RUN cargo build --release


FROM debian:bullseye-slim

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/cerebro /usr/local/bin

CMD ["/usr/local/bin/cerebro"]
