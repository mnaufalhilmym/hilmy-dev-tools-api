fn main() {
    let proto_files = ["proto/account.proto"];

    tonic_build::configure()
        .build_server(true)
        .compile_protos(&proto_files, &["."])
        .unwrap();

    println!("cargo:rerun-if-changed={}", proto_files.join(", "));
}
