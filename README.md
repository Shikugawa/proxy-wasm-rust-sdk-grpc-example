# proxy-wasm-rust-sdk-grpc-example

The examples of gRPC operations with proxy-wasm-rust-sdk. This example contains

1. gRPC callout (Unary gRPC)
2. gRPC stream management (Client/Server/Bidi gRPC streaming)

## Requirements
- Envoy 1.19 (It is not released on 2021/4/13)
- [grpc-go](https://github.com/grpc/grpc-go)

## How to play?

Please start valid service you want to try belongs to [here](https://github.com/grpc/grpc-go/tree/master/examples)

```
cargo build -p stream --target wasm32-unknown-unknown
envoy --config-path run/config-grpc-stream.yaml
```
