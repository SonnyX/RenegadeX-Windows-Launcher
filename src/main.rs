#![windows_subsystem="windows"]

extern crate reqwest;
extern crate json;
#[macro_use]
extern crate sciter;
extern crate renegadex_patcher;
extern crate xml;
extern crate ini;

mod traits;

use std::env::args;

use std::{process,io};
use std::fs::File;
use std::sync::{Arc,Mutex};
use std::rc::{Rc, Weak};
use std::ops::Deref;

use sciter::{Value, Element, HELEMENT, host, utf};
use sciter::dom::event::*;

use renegadex_patcher::{Downloader,Progress};
use traits::JsonExtend;
use xml::reader::{EventReader, XmlEvent};
use ini::Ini;

struct Handler {
  root: Option<sciter::Element>,
  patcher: Arc<Mutex<Downloader>>,
  update_available: Arc<Mutex<Option<bool>>>,
}

impl Handler {
  fn check_update(&self, done: sciter::Value) -> bool {
    let mut patcher = self.patcher.clone();
		std::thread::spawn(move || {
      let mut patcher = patcher.lock().unwrap();
      patcher.retrieve_mirrors();
      let mut update_available = patcher.update_available();
      if update_available {
        println!("Update available");
        done.call(None, &make_args!("update.htm"), None).unwrap();
      } else {
        println!("No update available");
        done.call(None, &make_args!("frontpage.htm"), None).unwrap();
      }
		});
		true
  }
  fn poll_progress(&self, callback: sciter::Value) -> bool {
    let progress : Arc<Mutex<Progress>> = self.patcher.lock().unwrap().get_progress();
		std::thread::spawn(move || {
      callback.call(None, &make_args!("update.htm"), None).unwrap();
		});
		true
  }
}

impl sciter::EventHandler for Handler {
	dispatch_script_call! {
		fn check_update(Value);
    fn poll_progress(Value);
  }
}

fn main() {
  let conf = match Ini::load_from_file("RenegadeX-Launcher.ini") {
    Ok(conf) => conf,
    Err(_e) => {
      let mut conf = Ini::new();
      conf.with_section(Some("RenX_Launcher"))
        .set("GameLocation", "C:/Program Files (x86)/Renegade X/")
        .set("VersionUrl", "https://static.renegade-x.com/launcher_data/version/release.json")
        .set("PlayerName", "")
        .set("LauncherTheme", "dom")
        .set("LastNewsGUID", "")
        .set("64-bit-version", "false");
      conf.write_to_file("RenegadeX-Launcher.ini").unwrap();
      conf
    }
  };

  let section = conf.section(Some("RenX_Launcher".to_owned())).unwrap();
  let game_location = section.get("GameLocation").unwrap();
  let version_url = section.get("VersionUrl").unwrap();

  let mut current_path = std::env::current_exe().unwrap();
  current_path.pop();
  sciter::set_options(
    sciter::RuntimeOptions::ScriptFeatures(
      sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8 | // Enables Sciter.machineName()
      sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8 | // Enables opening file dialog (view.selectFile())
      sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SOCKET_IO as u8
    )
  ).unwrap(); // Enables connecting to the inspector via Ctrl+Shift+I
  let mut frame = sciter::Window::new();
  let mut downloader = Downloader::new();
  downloader.set_location(game_location.to_string());
  downloader.set_version_url(version_url.to_string());
  let patcher : Arc<Mutex<Downloader>> = Arc::new(Mutex::new(downloader));
  let update_available : Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(None));
  frame.event_handler(Handler{root:None, patcher: patcher.clone(), update_available: update_available.clone()});
  current_path.push("dom/load-page.htm");
  frame.load_file(current_path.to_str().unwrap());
  frame.run_app();
}


pub struct Launcher {
  //for example: ~/RenegadeX/
  RenegadeX_location: Option<String>,
  //For example: DRI_PRIME=1
  env_arguments: Option<String>,
  player_name: Option<String>,
  servers: Option<json::JsonValue>,
  ping: Option<json::JsonValue>,
  x64_bit: bool
}

impl Launcher {
  pub fn new(game_folder: String) -> Launcher {
    Launcher {
      RenegadeX_location: Some(game_folder),
      env_arguments: None,
      player_name: None,
      servers: None,
      ping: None,
      x64_bit: true
    }
  }

  pub fn refresh_server_list(&mut self) {
    
  }

  pub fn launch_game(&mut self, server_index: Option<u16>) -> std::process::Child {
    if server_index == None {
      let mut wine_location = self.RenegadeX_location.clone().unwrap();
      wine_location.push_str("libs/wine/bin/wine64");
      let mut game_location = self.RenegadeX_location.clone().unwrap();
      game_location.push_str("game_files/Binaries/Win64/UDK.exe");

      let mut wine_prefix = self.RenegadeX_location.clone().unwrap();
      wine_prefix.push_str("wine_instance/");
      return process::Command::new(wine_location)
        .arg(game_location)
        //.arg("5.39.74.177:7777")
        .arg("-nomoviestartup")
        .arg("-ini:UDKGame:DefaultPlayer.Name=SonnyX")	
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn().expect("failed to execute child");

    } else {
      let mut game_location = self.RenegadeX_location.clone().unwrap();
      game_location.push_str("C:/Program Files (x86)/Renegade X/Binaries/Win64/UDK.exe");
      return process::Command::new(game_location)
        .arg("some server")
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn().expect("failed to execute child");
    }
  }
}
