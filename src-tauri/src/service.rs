use tauri::{AppHandle, async_runtime::Receiver};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};

pub struct Service {
    _rx: Receiver<CommandEvent>,
    tx: CommandChild,
}

impl Service {
    pub fn new(app: &AppHandle) -> Self {
        use tauri_plugin_shell::ShellExt;

        println!("opening service...");

        let (rx, tx) = app
            .shell()
            .sidecar("service")
            .unwrap()
            .spawn()
            .expect("Failed to spawn sidecar");

        println!("opened service");

        Self { _rx: rx, tx }
    }

    #[cfg(target_os = "windows")]
    pub fn shutdown(mut self) {
        use windows_sys::Win32::System::Console::{
            AttachConsole, CTRL_C_EVENT, FreeConsole, GenerateConsoleCtrlEvent,
        };
        println!("Shutting down service...");

        let pid = self.tx.pid();

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
                self.tx.kill().expect("Failed to kill child process");
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
                self.tx.kill().expect("Failed to kill child process");
                return;
            }

            // Detach from the process's console
            FreeConsole();
        }
    }

    // #[cfg(not(target_os = "windows"))]
    // pub fn shutdown(self) {
    //     println!("Shutting down service...");

    //     use nix::sys::signal::{Signal, kill};
    //     use nix::unistd::Pid;
    //     match kill(Pid::from_raw(self.tx.pid() as i32), Signal::SIGINT) {
    //         Ok(_) => println!("Service shutdown."),
    //         Err(e) => eprintln!("Error shutting down service: {e:?}"),
    //     };
    // }
}
