fn main() {
    let proto_files = ["proto/apprepo.proto"];

    let mut prost_config = prost_build::Config::new();
    prost_config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        .build_server(true)
        .compile_with_config(prost_config, &proto_files, &["."])
        .unwrap();

    println!("cargo:rerun-if-changed={}", proto_files.join(", "));
}
