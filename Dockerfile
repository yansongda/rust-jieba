FROM rust:latest AS builder

WORKDIR /www

COPY ./ .
COPY sources.list /etc/apt/sources.list

RUN update-ca-certificates \
    && cargo build --release


FROM debian:bullseye-slim

WORKDIR /www
ENV TZ=Asia/Shanghai

COPY --from=builder /www/target/release/rust-jieba ./app
COPY --from=builder /etc/apt/sources.list /etc/apt/sources.list

RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

CMD ["/www/app"]