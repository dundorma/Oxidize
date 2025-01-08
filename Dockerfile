FROM rust:1.83.0

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

EXPOSE 8008

ENTRYPOINT ["./target/release/oxidize"]
