FROM rust as builder

COPY . /app

WORKDIR /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/actix_gw /app/actix_gw
WORKDIR /app

CMD ["./actix_gw"]