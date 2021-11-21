use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult};
// use std::process::Command;

fn main() {
    unsafe {
        let mut pid: Result<ForkResult> = fork();
        fork_child_proc("Host Manager", &pid);
        let pid: Result<ForkResult> = fork();
        fork_child_proc("Lock Manager", &pid);
        fork_child_proc("Server", &pid);
    }
    calculate_x();
}

unsafe fn fork_child_proc(child_proc: &str, pid: &Result<ForkResult, Err>) {
    match pid.expect("Fork failed: unable to create child proc.") {
        ForkResult::Child => {
            println!("Starting PiSQL {}...", child_proc);
        }
        ForkResult::Parent { child } => {
            wait();
            println!("{} has started", child_proc);
        }
    }
}

fn calculate_x() {
    loop {}
}
/*
PiSQL
--> Setup registry/directories/admin work
--> Start HM as a child process and keep checking if the process is still alive
    --> While loop to check if process if alive, if alive, keep looping
    --> If it crashed, try restarting HM
    -->


*/
