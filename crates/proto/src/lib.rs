// crates/proto/src/lib.rs
pub mod common {
    tonic::include_proto!("finalverse.common");
}

pub mod world {
    tonic::include_proto!("finalverse.world");
}

pub mod story {
    tonic::include_proto!("finalverse.story");
}