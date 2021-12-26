FROM rust:latest AS builder

WORKDIR /www

COPY ./ .
COPY sources.list /etc/apt/sources.list

RUN update-ca-certificates \
    && cargo build --release


FROM debian:bullseye-slim

WORKDIR /www

COPY --from=builder /www/target/release/rust-jieba ./app
COPY --from=builder /www/log4rs.yml ./
COPY --from=builder /etc/apt/sources.list /etc/apt/sources.list

CMD ["/www/app"]