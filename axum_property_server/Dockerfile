FROM rust as builder

COPY . /app

WORKDIR /app

RUN cargo build --profile release-lto

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release-lto/axum_property_server /app/axum_property_server
WORKDIR /app

CMD ["./axum_property_server"]