const BRAIINS_PROTOBUF_REPO: &str = "https://github.com/braiins/bos-plus-api";

use std::{env, fs, path::Path};

fn main() {
    env::set_var("PROTOC", protobuf_src::protoc());

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let repo_dir = out_dir.join("bos-plus-api");

    if repo_dir.exists() {
        fs::remove_dir_all(&repo_dir).unwrap();
    }

    git2::Repository::clone(BRAIINS_PROTOBUF_REPO, &repo_dir).unwrap();

    let proto_files: Vec<_> = walkdir::WalkDir::new(&repo_dir)
        .into_iter()
        .flatten()
        .map(|entry| entry.into_path())
        .filter(|path| path.extension().map(|p| p.to_str()).flatten() == Some("proto"))
        .collect();

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(&proto_files, &[repo_dir.join("proto")])
        .unwrap();
}
