fn main() {
  // TODO
  println!("cargo:rustc-link-search=native=voicevox_core");
  println!("cargo:rustc-link-lib=voicevox_core");

  tauri_build::build();
}
