# build stage
FROM rust:1.54-alpine as builder
WORKDIR /app
COPY . /app
RUN rustup default stable
RUN apk update && apk add --no-cache libpq musl-dev pkgconfig openssl-dev postgresql-dev
RUN cargo build --release
# RUN cargo build

# Prod stage
FROM alpine:3.15
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
RUN apk update && apk add --no-cache libpq curl
COPY --from=builder /app/.env /app
COPY --from=builder /app/settings.toml /app
# COPY --from=builder /app/target/debug/* /app/
# 
# only copy the execute file to minimal the image size
# do not copy the release folder
COPY --from=builder /app/target/release/rust-learn /app/
CMD ["./rust-learn"]



