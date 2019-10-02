fn main() {
    let proto = "proto/grpc/testing/worker_service.proto";

    tonic_build::configure()
        .compile(
            &["proto/grpc/testing/service.proto"],
            &["proto/grpc/testing", "proto/grpc/core"],
        )
        .unwrap();

    // prevent needing to rebuild if files (or deps) haven't changed
    println!("cargo:rerun-if-changed={}", proto);
}
