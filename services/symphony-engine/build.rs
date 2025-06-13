// services/symphony-engine/build.rs
fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile_protos(
            "../../proto/audio.proto",
            "../../proto",
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
