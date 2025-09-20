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

    pub fn shutdown(mut self) {
        println!("closing service...");
        match self.tx.write(b"\x03") {
            Ok(_) => println!("closed service"),
            Err(e) => eprintln!("could not close the service: {e:?}"),
        }
    }
}
