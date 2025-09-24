#[cfg(unix)]
use std::process::Child;

#[cfg(target_os = "windows")]
pub fn shutdown(child: &mut Child) {
    use windows_sys::Win32::System::Console::{
        AttachConsole, CTRL_C_EVENT, FreeConsole, GenerateConsoleCtrlEvent,
    };
    let pid = child.pid();
    unsafe {
        // Detach from the current console (if any)
        FreeConsole();

        // Attach to the console of the target process
        if AttachConsole(pid) == 0 {
            eprintln!(
                "Failed to attach to console of PID {pid}. Error: {}",
                std::io::Error::last_os_error()
            );
            // If attaching fails, try to kill the process as a fallback
            child.kill().expect("Failed to kill child process");
            return;
        }

        // Generate the Ctrl+C event
        if GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0) == 0 {
            eprintln!(
                "Failed to send Ctrl+C event to PID {}. Error: {}",
                pid,
                std::io::Error::last_os_error()
            );
            // If attaching fails, try to kill the process as a fallback
            child.kill().expect("Failed to kill child process");
            return;
        }

        // Detach from the process's console
        FreeConsole();
    }
}

#[cfg(unix)]
pub fn shutdown(child: &mut Child) {
    println!("Shutting down service...");

    use nix::sys::signal::{Signal, kill};
    use nix::unistd::Pid;
    let pid = Pid::from_raw(child.id() as i32);
    if let Err(e) = kill(pid, Signal::SIGINT) {
        eprintln!("Failed to send Ctrl+C to PID {pid}. Error: {e:?}");
        child.kill().expect("Failed to kill child process");
    };
}