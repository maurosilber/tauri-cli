use tauri::{AppHandle, async_runtime::Receiver};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};

pub struct Service {
    _rx: Receiver<CommandEvent>,
    tx: CommandChild,
}

impl Service {
    pub fn new(app: &AppHandle) -> Self {
        use tauri_plugin_shell::ShellExt;

        let (rx, tx) = app
            .shell()
            .sidecar("service")
            .unwrap()
            .spawn()
            .expect("Failed to spawn sidecar");

        Self { _rx: rx, tx }
    }

    pub fn shutdown(mut self) {
        match self.tx.write(b"exit") {
            Ok(_) => (),
            Err(e) => eprintln!("could not close the service: {e:?}"),
        }
    }
}
