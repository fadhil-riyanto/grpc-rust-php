fn main() -> () {
    tonic_build::configure()
        .file_descriptor_set_path("/home/fadhil_riyanto/BALI64/protobuf-php-rust/tonic-builder/out/a.bin")
        .compile_protos(&["../proto/proto_1.proto"], &["../proto"])
        .unwrap();


    tonic_build::compile_protos("../proto/proto_1.proto").unwrap();
}