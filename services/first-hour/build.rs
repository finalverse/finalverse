// services/first-hour/build.rs
fn main() {
    // If you have proto files for first-hour service
    // tonic_build::compile_protos("../../proto/first_hour.proto")
    //     .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    println!("cargo:rerun-if-changed=../../proto");
}