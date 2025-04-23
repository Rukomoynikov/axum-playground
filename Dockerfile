FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM node:alpine AS frontend
WORKDIR /app
COPY ./client .
RUN npm install && npm run build

FROM rust:latest AS release
WORKDIR /app
COPY --from=builder /app/target/release/web-with-graphql /app/web-with-graphql
COPY --from=frontend /app/dist /app/assets

CMD ["/app/web-with-graphql"]
