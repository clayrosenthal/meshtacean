fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "gen")]
    {
        if let Err(e) = generate_protobuf_files() {
            println!("Error generating protobuf files: {}", e);
        }
    }
    Ok(())
}

#[cfg(feature = "gen")]
fn generate_protobuf_files() -> Result<(), Box<dyn std::error::Error>> {
    // Get the manifest directory (where Cargo.toml is located)
    let manifest_dir = std::path::PathBuf::new();
    let proto_dir = manifest_dir.join("protobufs").join("meshtastic");
    let out_dir = manifest_dir.join("src").join("protobufs");

    // Verify the directory exists
    if !proto_dir.exists() {
        panic!(
            "Proto directory not found: {:?}, run `git submodule update --init` to clone the protobufs repo",
            proto_dir
        );
    }

    // Clean the output directory if it exists
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir)?;
    }
    std::fs::create_dir_all(&out_dir)?;

    // Get the list of proto files
    let proto_files = proto_dir
        .read_dir()?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "proto")
                .unwrap_or(false)
        })
        .filter(|entry| entry.path().file_name().unwrap().to_str().unwrap() != "nanopb.proto")
        .map(|entry| entry.path())
        .collect::<Vec<std::path::PathBuf>>();

    let mut config = prost_build::Config::new();
    let mut proto_fd_set = config.load_fds(
        &proto_files,
        &[proto_dir.parent().unwrap().to_path_buf(), proto_dir],
    )?;

    // for file in &mut proto_fd_set.file {
    //     if file.name() == "nanopb.proto" {
    //         file.package = Some("meshtastic".to_string());
    //     }
    // }

    config.out_dir(out_dir);
    config.compile_fds(proto_fd_set).unwrap();

    // Tell Cargo to rerun this script if any proto files change
    println!("cargo:rerun-if-changed=protobufs/meshtastic");
    println!("cargo:rerun-if-changed=protobufs/nanopb.proto");

    Ok(())
}
