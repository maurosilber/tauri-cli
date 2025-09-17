#[cfg(desktop)]
use std::error::Error;
use std::sync::Mutex;

use tauri::{
    App, AppHandle, Manager,
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder},
};

mod service;

type SetupResult = Result<(), Box<dyn Error>>;

#[derive(Default)]
struct AppState {
    service: Option<service::Service>,
}

impl AppState {
    fn start_service(&mut self, app: &AppHandle) {
        if self.service.is_none() {
            self.service = Some(service::Service::new(app))
        }
    }

    fn stop_service(&mut self) {
        if let Some(x) = self.service.take() {
            x.shutdown();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(gui: bool) {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init()) // to launch sidecar binaries
        .setup(move |app| {
            if gui {
                open_window(app.handle(), "main");
            } else {
                set_dock_visibility(app.handle(), false);
            }
            state(app);
            tray(app)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app, event| {
            if let tauri::RunEvent::ExitRequested {
                code: None, api, ..
            } = event
            {
                api.prevent_exit()
            }
        })
}

fn state(app: &mut App) {
    app.manage(Mutex::new(AppState::default()));
}

fn tray(app: &mut App) -> SetupResult {
    let service = CheckMenuItemBuilder::new("Service")
        .id("service")
        .checked(false)
        .build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&MenuItemBuilder::new("Open GUI").id("open_gui").build(app)?)
        .item(
            &MenuItemBuilder::new("Open logs")
                .id("open_logs")
                .build(app)?,
        )
        .separator()
        .item(&service)
        .separator()
        .item(&MenuItemBuilder::new("Quit").id("quit").build(app)?)
        .build()?;

    tauri::tray::TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .show_menu_on_left_click(true)
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            "open_gui" => open_window(app, "main"),
            "open_logs" => open_window(app, "perfetto"),
            "service" => {
                let state = app.state::<Mutex<AppState>>();
                let mut state = state.lock().unwrap();
                if service
                    .is_checked()
                    .expect("could not access service menu checked status")
                {
                    state.start_service(app);
                } else {
                    state.stop_service();
                }
            }
        })
        .build(app)?;
    Ok(())
}

fn set_dock_visibility(app: &AppHandle, visible: bool) {
    #[cfg(target_os = "macos")]
    let _ = app.set_dock_visibility(visible);
}

fn open_window(app: &tauri::AppHandle, label: &'static str) {
    set_dock_visibility(app, true);
    match app.get_webview_window(label) {
        Some(w) => {
            if let Err(e) = w.set_focus() {
                eprint!("could not focus {label} window: {e:?}");
            }
        }
        None => {
            let app = app.to_owned();
            std::thread::spawn(move || {
                let conf = app
                    .config()
                    .app
                    .windows
                    .iter()
                    .find(|c| c.label == label)
                    .expect("could not find window");
                tauri::WebviewWindowBuilder::from_config(&app, conf)
                    .expect("could not create window builder")
                    .build()
                    .expect("could not build window");
            });
        }
    };
}
