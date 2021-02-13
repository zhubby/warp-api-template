FROM alpine:3
COPY target/x86_64-unknown-linux-musl/release/warp-api-template ./warp-api-template
ENTRYPOINT ["./warp-api-template"]