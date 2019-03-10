#[cfg(windows)]
extern crate winres;

fn main() {
  embed_resources();
}

#[cfg(windows)]
fn embed_resources() {
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
fn embed_resources() {

}
