FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:latest AS release
WORKDIR /app
COPY --from=builder /app/target/release/web-with-graphql /app/web-with-graphql

CMD ["/app/web-with-graphql"]
