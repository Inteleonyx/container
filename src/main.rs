mod container;

use crate::container::process::ContainerProcess;
use log::{info, error};
use std::fs;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 1. Caminho absoluto garantido
    let rootfs_path = fs::canonicalize("./alpine-rootfs")
        .expect("FOLDER STILL EMPTY! Extract the alpine-minirootfs.tar.gz into it first.")
        .to_str()
        .unwrap()
        .to_string();

    // 2. No Alpine, usamos "/bin/sh". 
    // Se quiser bash futuramente, terá que rodar 'apk add bash' lá dentro.
    let container = ContainerProcess::new(
        rootfs_path,
        vec!["/bin/sh".to_string()], 
        "isolated-box".to_string(),
    );

    if let Err(e) = container.spawn() {
        eprintln!("Failed to start container: {}", e);
        std::process::exit(1);
    }
}