#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
use std::io::Write;

#[cfg(windows)]
fn main() {
  let mut res = winres::WindowsResource::new();
  res.set_manifest_file("manifest.xml");
  res.set_icon("rx.ico");
  match res.compile() {
    Err(e) => {
      write!(std::io::stderr(), "{}", e).unwrap();
      std::process::exit(1);
    },
    Ok(_) => {}
  }
}

#[cfg(not(windows))]
fn main() {
}
