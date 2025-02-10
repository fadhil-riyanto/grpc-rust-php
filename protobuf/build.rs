use std::io::Result;

fn main() -> Result<()> {
        prost_build::compile_protos(
                &[
                        "./proto/se_1.proto"
                ], &[
                        "./proto"
                ]
        );

        Ok(())
}