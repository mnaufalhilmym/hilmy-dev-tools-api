fn main() {
    let proto_files = ["proto/link.proto"];

    tonic_build::configure()
        .build_server(true)
        .compile(&proto_files, &["."])
        .unwrap();

    println!("cargo:rerun-if-changed={}", proto_files.join(", "));
}
