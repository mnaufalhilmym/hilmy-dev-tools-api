fn main() {
    let proto_files = ["proto/gateway.proto"];

    tonic_build::configure()
        .build_server(true)
        .compile(&proto_files, &["."])
        .unwrap();

    println!(
        "cargo:rerun-if-changed={}, ./migrations",
        proto_files.join(", ")
    );
}
