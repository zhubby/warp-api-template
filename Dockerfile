FROM alpine:3
COPY . .
WORKDIR target/x86_64-unknown-linux-musl/release
ENTRYPOINT ["./warp-api-template"]