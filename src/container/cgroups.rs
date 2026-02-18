/* src/container/cgroups.rs */

use std::fs;
use std::path::PathBuf;
use std::io::Write;
use log::{info, debug};

pub struct CgroupManager {
    base_path: PathBuf,
}

impl CgroupManager {
    pub fn new(container_name: &str) -> Self {
        // No Cgroup v2, criamos subdiretórios em /sys/fs/cgroup
        let base_path = PathBuf::from("/sys/fs/cgroup").join(container_name);
        Self { base_path }
    }

    pub fn apply_limits(&self, pid: nix::unistd::Pid, memory_limit: &str) -> anyhow::Result<()> {
        if !self.base_path.exists() {
            fs::create_dir(&self.base_path)?;
        }

        info!("Applying Cgroup limits at {:?}", self.base_path);

        // 1. Adicionar o processo ao cgroup
        self.write_value("cgroup.procs", &pid.to_string())?;

        // 2. Limitar memória (ex: "512M")
        // O arquivo memory.max define o teto rígido
        self.write_value("memory.max", memory_limit)?;

        // 3. Limitar PIDs (proteção contra Fork Bomb)
        self.write_value("pids.max", "50")?;

        Ok(())
    }

    fn write_value(&self, file: &str, value: &str) -> anyhow::Result<()> {
        let path = self.base_path.join(file);
        let mut f = fs::File::create(path)?;
        f.write_all(value.as_bytes())?;
        Ok(())
    }

    // Limpeza ao encerrar
    pub fn remove(&self) -> anyhow::Result<()> {
        debug!("Removing cgroup {:?}", self.base_path);
        fs::remove_dir(&self.base_path)?;
        Ok(())
    }
}