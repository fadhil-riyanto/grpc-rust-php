

fn main() -> () {
    tonic_build::configure()
        .file_descriptor_set_path(
            "build-proto/hello.pb.bin",
        )
        .compile_protos(&["proto/hello.proto"], &["proto"])
        .unwrap();

    // tonic_build::compile_protos("proto/hello.proto").unwrap();
}
