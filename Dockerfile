
FROM rust:1.59 AS builder

WORKDIR /usr/src/pmanager
COPY . .
RUN cargo install --path .

FROM rust:1.59-alpine

COPY --from=builder /usr/src/pmanager/target/release/personal-manager /usr/local/bin/pmanager
CMD ["pmanager"]
