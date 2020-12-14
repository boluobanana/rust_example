fn main() {
  // let target_path = std::env::var("OUT_DIR").expect("get env fail");
  // let mut target_path = std::path::PathBuf::from(target_path);
  // target_path.pop();
  // target_path.pop();
  // target_path.pop();
  // println!("cargo:rustc-link-search=all={}", target_path.to_str().unwrap());
  println!("cargo:rustc-link-lib=dylib=dylib_window_example");
}