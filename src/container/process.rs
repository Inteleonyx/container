use crate::container::rootfs;
use crate::container::cgroups::CgroupManager; // Se já criou o cgroups.rs
use nix::sched::{clone, CloneFlags};
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execvp, sethostname};
use std::ffi::CString;
use log::{info, debug, error, warn};

pub struct ContainerProcess {
    pub rootfs: String,
    pub command: Vec<String>,
    pub hostname: String,
}

impl ContainerProcess {
    pub fn new(rootfs: String, command: Vec<String>, hostname: String) -> Self {
        Self { rootfs, command, hostname }
    }

    pub fn spawn(&self) -> anyhow::Result<()> {
        info!("Spawning container process...");

        let mut flags = CloneFlags::empty();
        flags.insert(CloneFlags::CLONE_NEWNS);
        flags.insert(CloneFlags::CLONE_NEWUTS);
        flags.insert(CloneFlags::CLONE_NEWPID);
        flags.insert(CloneFlags::CLONE_NEWIPC);

        const STACK_SIZE: usize = 1024 * 1024;
        let mut stack = [0u8; STACK_SIZE];

        // O callback agora captura o 'self' para usar dentro do filho
        let callback = Box::new(|| {
            match self.child_init() {
                Ok(_) => 0,
                Err(e) => {
                    // Logs dentro do filho precisam ser claros
                    eprintln!("[Child Error] {}", e);
                    1
                }
            }
        });

        let pid = unsafe {
            clone(
                callback,
                &mut stack[..],
                flags,
                Some(Signal::SIGCHLD as i32),
            )?
        };

        // --- FASE 3: CGROUPS (Pai configura o limite para o filho) ---
        let cg = CgroupManager::new("my-container");
        if let Err(e) = cg.apply_limits(pid, "256M") {
            warn!("Failed to apply cgroup limits: {}", e);
        }

        debug!("Container process cloned with PID: {}", pid);

        match waitpid(pid, None)? {
            WaitStatus::Exited(p, status) => {
                info!("Container process {} exited with status {}", p, status);
            }
            _ => warn!("Container process exited unexpectedly"),
        }

        // Limpeza
        let _ = cg.remove();
        Ok(())
    }

    fn child_init(&self) -> anyhow::Result<()> {
        // 1. Hostname
        sethostname(&self.hostname)?;

        // 2. FASE 2: ROOTFS (Onde o isolamento real acontece)
        // Isso vai fazer o pivot_root para a pasta do Alpine
        rootfs::prepare_rootfs(&self.rootfs)?;

        // 3. Execução
        let cmd_path = CString::new(self.command[0].as_str())?;
        let args: Vec<CString> = self.command
            .iter()
            .map(|s| CString::new(s.as_str()).unwrap())
            .collect();

        info!("Executing: {:?}", self.command);
        execvp(&cmd_path, &args)?;

        Ok(())
    }
}