FROM rust as builder

COPY . /app

WORKDIR /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/actix_properties_ms /app/actix_properties_ms
WORKDIR /app

CMD ["./actix_properties_ms"]