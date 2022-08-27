use nix::sched::{unshare, CloneFlags};
use std::os::unix::io::RawFd;

use crate::errors::Errcode;
use crate::ipc::{recv_boolean, send_boolean};

pub fn userns(fd: RawFd, uid: u32) -> Result<(), Errcode> {
    log::debug!("Setting up user namespace with UID {}", uid);
    let has_userns = match unshare(CloneFlags::CLONE_NEWUSER) {
        Ok(_) => true,
        Err(_) => false,
    };
    send_boolean(fd, has_userns)?;

    if recv_boolean(fd)? {
        return Err(Errcode::NamespacesError(0));
    }

    if has_userns {
        log::info!("User namespaces set up");
    } else {
        log::info!("User namespaces not supported, continuing...");
    }

    // Switch UID / GID with the one provided by the user

    Ok(())
}

use nix::unistd::Pid;
pub fn handle_child_uid_map(pid: Pid, fd: RawFd) -> Result<(), Errcode> {
    if recv_boolean(fd)? {
        // Perform UID / GID map here
    } else {
        log::info!("No user namespace set up from child process");
    }

    log::debug!("Child UID/GID map done, sending signal to child to continue...");
    send_boolean(fd, false)
}
