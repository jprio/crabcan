use crate::config::ContainerOpts;
use crate::errors::Errcode;

use nix::sched::clone;
use nix::sched::CloneFlags;
use nix::sys::signal::Signal;
use nix::unistd::Pid;

const STACK_SIZE: usize = 1024 * 1024;

pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();
    flags.insert(CloneFlags::CLONE_NEWNS);
    flags.insert(CloneFlags::CLONE_NEWCGROUP);
    flags.insert(CloneFlags::CLONE_NEWPID);
    flags.insert(CloneFlags::CLONE_NEWIPC);
    flags.insert(CloneFlags::CLONE_NEWNET);
    flags.insert(CloneFlags::CLONE_NEWUTS);
    // Flags definition
    match clone(
        Box::new(|| child(config.clone())),
        &mut tmp_stack,
        flags,
        Some(Signal::SIGCHLD as i32),
    ) {
        Ok(pid) => {
            log::info!("Pid : {}", pid);
            Ok(pid)
        }
        Err(_) => Err(Errcode::ChildProcessError(0)),
    }
}
fn child(config: ContainerOpts) -> isize {
    log::info!(
        "Starting container with command {} and args {:?}",
        config.path.to_str().unwrap(),
        config.argv
    );
    match setup_container_configurations(&config) {
        Ok(_) => log::info!("Container set up successfully"),
        Err(e) => {
            log::error!("Error while configuring container: {:?}", e);
            return -1;
        }
    }
    0
}

use crate::hostname::set_container_hostname;

fn setup_container_configurations(config: &ContainerOpts) -> Result<(), Errcode> {
    set_container_hostname(&config.hostname)?;
    Ok(())
}
