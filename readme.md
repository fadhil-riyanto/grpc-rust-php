## my gRPC leaning
compile PHP grpc
```sh
protoc --proto_path="./proto"  \
      --php_out="./compiled" 
      --grpc_out="generate_server:compiled" \ 
      --plugin=protoc-gen-grpc=/usr/bin/grpc_php_plugin "./proto/helloworld/hello.proto"
```

for rust stuff (A server)
```sh
cargo run --bin helloworld
```1