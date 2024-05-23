FROM rust as builder

COPY . /app
# Copy the protobuf files into the image
COPY protos ./protos
COPY build.rs ./build.rs
RUN apt-get update && \
    apt-get install -y protobuf-compiler


WORKDIR /app

RUN cargo build --profile release-lto

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release-lto/grpc_apigw /app/grpc_apigw
COPY --from=builder /app/protos /app/protos
COPY --from=builder /app/build.rs /app/build.rs
# Install protoc compiler

WORKDIR /app

CMD ["./grpc_apigw"]