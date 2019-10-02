fn main() {
    println!("Hello, world!");
}

pub mod pb {
    tonic::include_proto!("grpc.testing");
}
