fn main() {
    let protoc_bin_path =
        protoc_bin_vendored::protoc_bin_path().expect("Failed to find protoc binary");
    std::env::set_var("PROTOC", protoc_bin_path);

    let proto_files = ["src/protos/blocks.proto"];
    let includes = ["src/protos"];

    prost_build::Config::new()
        .out_dir("src/protos")
        .compile_protos(&proto_files, &includes)
        .expect("Failed to compile Protobuf files");
}
