FROM rust as builder

COPY . /app

WORKDIR /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/actix_user_ms /app/actix_user_ms
WORKDIR /app

CMD ["./actix_user_ms"]