// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use specta::Type;
use tauri::{
  generate_context, generate_handler, App, Builder, CustomMenuItem, Manager, SystemTray,
  SystemTrayMenu, WindowEvent,
};

mod command;
mod vv;

use command::*;

#[derive(Debug, Type)]
pub struct AppState {}

// #[derive(Debug, Type)]
// pub struct SerializeAppState {
//   windows: Vec<SerializeWindowData>,
//   pub overlay: AtomicBool,
// }

impl AppState {}

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  tauri_specta::ts::export(
    specta::collect_types![echo],
    "../src/lib/generated/specta/bindings.ts",
  )
  .expect("failed to generate types");

  let builder = Builder::default();
  let state = AppState {};

  builder
    .setup(move |app: &mut App| {
      let _handle = app.handle();
      let main_window = Arc::new(app.get_window("main").expect("Failed to get main window"));

      //
      #[cfg(not(debug_assertions))]
      {
        _window_focus(&main_window)?;
      }
      //

      //
      main_window.on_window_event({
        let main_window = Arc::clone(&main_window);
        move |e| {
          if let WindowEvent::CloseRequested { api, .. } = e {
            api.prevent_close();
            main_window.hide().unwrap();
          }
        }
      });
      //

      //
      const MENU_SHOW: &str = "show";
      const MENU_QUIT: &str = "quit";

      let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(MENU_SHOW, "Show"))
        .add_item(CustomMenuItem::new(MENU_QUIT, "Quit"));

      let _tray_handle = SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("Relais")
        .on_event(move |e| match &e {
          // SystemTrayEvent::LeftClick { .. } => _window_focus(&main_window).unwrap(),
          // SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
          //   MENU_SHOW => _window_focus(&main_window).unwrap(),
          //   MENU_QUIT => exit_0(&handle).expect("Failed to remove tasktray icon"),
          //   _ => (),
          // },
          _ => (),
        })
        .build(app)?;
      //

      Ok(())
    })
    .on_window_event(move |e| match e.event() {
      WindowEvent::Destroyed => println!("destroy!"),
      WindowEvent::ThemeChanged(theme) => println!("theme = {:?}", theme),
      _ => (),
    })
    .manage(state)
    .invoke_handler(generate_handler![echo])
    .run(generate_context!())
    .expect("error while running tauri application");
}
