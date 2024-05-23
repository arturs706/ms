use std::path::PathBuf;

fn get_proto_files(proto_path: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let proto_files = std::fs::read_dir(proto_path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "proto"))
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    Ok(proto_files)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let include_path = "./include";
    let proto_path = "./protos";
    let proto_files = get_proto_files(proto_path)?;
    tonic_build::configure()
        .build_server(true)
        .generate_default_stubs(true)
        .compile(&proto_files, &[include_path, proto_path])?;

    Ok(())
}