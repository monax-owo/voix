use crate::vv;

#[tauri::command]
#[specta::specta]
pub fn echo(text: String) -> Result<(), String> {
  vv::echo(text).map_err(|v| v.to_string())?;
  Ok(())
}
