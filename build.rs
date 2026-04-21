fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        std::env::set_var("PROTOC", protobuf_src::protoc());
    }
    tonic_build::compile_protos("proto/ai_prop.proto")?;
    Ok(())
}
