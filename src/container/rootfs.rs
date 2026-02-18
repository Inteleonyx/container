use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::unistd::{chdir, pivot_root};
use std::fs;
use std::path::Path;
use log::{debug, info};

pub fn prepare_rootfs(new_root: &str) -> anyhow::Result<()> {
    let root = Path::new(new_root);

    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_REC | MsFlags::MS_PRIVATE,
        None::<&str>,
    )?;

    debug!("Bind mounting rootfs to itself...");
    mount(
        Some(root),
        root,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    )?;

    let old_root_dir = root.join(".old_root");
    if !old_root_dir.exists() {
        fs::create_dir_all(&old_root_dir)?;
    }

    debug!("Pivoting root to {:?}", root);
    pivot_root(root, &old_root_dir)?;

    chdir("/")?;

    umount2("/.old_root", MntFlags::MNT_DETACH)?;
    fs::remove_dir("/.old_root")?;

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