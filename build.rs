extern crate protoc_rust_grpc;

fn main() {
    use std::env;
    if env::consts::OS != "windows" {
        build_protos();
    }
}

fn build_protos() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/proto",
        includes: &["proto"],
        input: &[
            "proto/orm.proto",
        ],
        rust_protobuf: true,
        ..Default::default()
    }).expect("protoc-rust-grpc");
}