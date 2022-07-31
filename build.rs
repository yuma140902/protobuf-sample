fn main() {
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .includes(&["src/protos"])
        .inputs(&["src/protos/person.proto"])
        .cargo_out_dir("protos")
        .run_from_script();
}
