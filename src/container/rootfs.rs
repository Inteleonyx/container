use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::unistd::{chdir, pivot_root};
use std::fs;
use std::path::Path;
use log::{debug, info};

pub fn prepare_rootfs(new_root: &str) -> anyhow::Result<()> {
    let root = Path::new(new_root);

    // 1. Tornar a visão de mounts do container PRIVADA
    // Sem isso, o pivot_root retorna EBUSY
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_REC | MsFlags::MS_PRIVATE,
        None::<&str>,
    )?;

    // 2. Garantir que o new_root seja um mount point (Bind Mount)
    debug!("Bind mounting rootfs to itself...");
    mount(
        Some(root),
        root,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    )?;

    // 3. Criar o diretório temporário para o sistema antigo
    let old_root_dir = root.join(".old_root");
    if !old_root_dir.exists() {
        fs::create_dir_all(&old_root_dir)?;
    }

    // 4. Pivot Root
    debug!("Pivoting root to {:?}", root);
    pivot_root(root, &old_root_dir)?;

    // 5. Mudar o diretório de trabalho para a nova raiz
    chdir("/")?;

    // 6. Desmontar o sistema antigo (que agora está em /.old_root)
    // MNT_DETACH garante que o unmount aconteça mesmo com arquivos abertos
    umount2("/.old_root", MntFlags::MNT_DETACH)?;
    fs::remove_dir("/.old_root")?;

    // 7. Montar o /proc (para comandos como ps, top, htop funcionarem)
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        None::<&str>,
    )?;

    info!("Rootfs successfully isolated at {}", new_root);
    Ok(())
}