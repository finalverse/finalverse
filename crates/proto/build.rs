// crates/proto/build.rs
use std::path::PathBuf;

fn main() {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                proto_root.join("common.proto").to_str().unwrap(),
                proto_root.join("world.proto").to_str().unwrap(),
                proto_root.join("story.proto").to_str().unwrap(),
            ],
            &[proto_root.to_str().unwrap()],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
}