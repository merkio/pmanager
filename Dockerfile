
FROM rust:1.59-bullseye AS builder

WORKDIR /usr/src/pmanager
COPY . .

RUN apt update && apt install -y gcc-multilib && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

FROM rust:1.59-alpine

COPY --from=builder /usr/src/pmanager/target/release/personal-manager /usr/local/bin/pmanager
CMD ["pmanager"]
