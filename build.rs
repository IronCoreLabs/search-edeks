fn main() {
    println!("cargo:rerun-if-changed=proto/");
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("proto")
        .inputs(["proto/transform.proto"])
        .include("proto")
        .customize(
            protobuf_codegen::Customize::default()
                .tokio_bytes(true)
                .tokio_bytes_for_string(true),
        )
        .pure()
        .run()
        .expect("protobuf codegen failed");
}
