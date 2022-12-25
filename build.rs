use std::path::{Path, PathBuf};

use walkdir::WalkDir;

/// Find all *.proto files in the `in_dir` and add them to the list of files
fn get_all_protos(in_dir: PathBuf) -> Vec<PathBuf> {
    let mut protos = Vec::new();
    let proto_ext = Some(Path::new("proto").as_os_str());
    for entry in WalkDir::new(&in_dir) {
        let path = entry.unwrap().into_path();
        if path.extension() == proto_ext {
            // Re-run this build.rs if any of the files in the protos dir change
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
            protos.push(path);
        }
    }

    protos
}

fn generate_protos(in_dir: PathBuf, out_dir: &PathBuf, protos: &[PathBuf]) {
    use pb_rs::types::FileDescriptor;
    use pb_rs::ConfigBuilder;

    // Delete all old generated files before re-generating new ones
    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir).unwrap();
    }

    std::fs::DirBuilder::new().create(out_dir).unwrap();
    let config_builder = ConfigBuilder::new(protos, None, Some(out_dir), &[in_dir])
        .unwrap()
        .build();
    FileDescriptor::run(&config_builder).unwrap();
}

fn main() {
    let base_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let in_dir = PathBuf::from(&base_dir).join("protobufs");
    let out_dir = PathBuf::from(&base_dir).join("src/protos");

    // Re-run this build.rs if the protos dir changes (i.e. a new file is added)
    println!("cargo:rerun-if-changed={}", in_dir.to_str().unwrap());
    let protos = get_all_protos(in_dir.clone());

    generate_protos(in_dir, &out_dir, &protos);
}
