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
        if let Some(pid) = self.tx.pid() {
            // Send CTRL+C event to the process group
            let _ = Command::new("taskkill")
                .args(&["/Pid", &pid.to_string(), "/T", "/F"])
                .output();
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
