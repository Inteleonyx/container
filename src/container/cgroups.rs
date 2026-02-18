use std::fs;
use std::path::PathBuf;
use std::io::Write;
use log::{info, debug};

pub struct CgroupManager {
    base_path: PathBuf,
}

impl CgroupManager {
    pub fn new(container_name: &str) -> Self {
        let base_path = PathBuf::from("/sys/fs/cgroup").join(container_name);
        Self { base_path }
    }

    pub fn apply_limits(&self, pid: nix::unistd::Pid, memory_limit: &str) -> anyhow::Result<()> {
        if !self.base_path.exists() {
            fs::create_dir(&self.base_path)?;
        }

        info!("Applying Cgroup limits at {:?}", self.base_path);

        self.write_value("cgroup.procs", &pid.to_string())?;

        self.write_value("memory.max", memory_limit)?;

        self.write_value("pids.max", "50")?;

        Ok(())
    }

    fn write_value(&self, file: &str, value: &str) -> anyhow::Result<()> {
        let path = self.base_path.join(file);
        let mut f = fs::File::create(path)?;
        f.write_all(value.as_bytes())?;
        Ok(())
    }

    pub fn remove(&self) -> anyhow::Result<()> {
        debug!("Removing cgroup {:?}", self.base_path);
        fs::remove_dir(&self.base_path)?;
        Ok(())
    }
}