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

use renegadex_patcher::{Downloader,Progress,Update, traits::Error};
use traits::JsonExtend;
use xml::reader::{EventReader, XmlEvent};
use ini::Ini;

struct Handler {
  root: Option<sciter::Element>,
  patcher: Arc<Mutex<Downloader>>,
  progress: Arc<Mutex<Progress>>,
  update_available: Arc<Mutex<Option<bool>>>,
}

impl Handler {
  fn check_update(&self, done: sciter::Value, error: sciter::Value) -> bool {
    let mut patcher = self.patcher.clone();
		std::thread::spawn(move || {
      let check_update = || -> Result<(), Error> {
        let update_available : Update;
        {
          let mut patcher = patcher.lock().unwrap();
          patcher.retrieve_mirrors()?;
          update_available = patcher.update_available()?;
        }
        match update_available {
          Update::UpToDate => {
            println!("No update available");
            done.call(None, &make_args!(false, false), None).unwrap();
          },
          Update::Resume | Update::Full => {
            println!("Resuming download!");
            done.call(None, &make_args!(true, true), None).unwrap();
          },
          Update::Delta => {
            println!("Update available");
            done.call(None, &make_args!(true, false), None).unwrap();
          }
        };
        Ok(())
		  };
      let result : Result<(),Error> = check_update();
      if result.is_err() {
        println!("{:#?}", result);
        //error.call(None, &make_args!(result.unwrap_err()), None);
      }
    });
		true
  }

  fn start_download(&self, callback: sciter::Value, callback_done: sciter::Value) -> bool {
    let mut progress = self.patcher.clone().lock().unwrap().get_progress();
		std::thread::spawn(move || {
      let mut notFinished = true;
      while notFinished {
        std::thread::sleep(std::time::Duration::from_millis(500));
        {
          let progress_locked = progress.lock().unwrap();
          let mut me : Value = format!(
            "{{\"hash\": [{},{}],\"download\": [{},{}],\"patch\": [{},{}]}}",
            progress_locked.hashes_checked.0.clone(),
            progress_locked.hashes_checked.1.clone(),
            progress_locked.download_size.0.clone()/10000,
            progress_locked.download_size.1.clone()/10000,
            progress_locked.patch_files.0.clone(),
            progress_locked.patch_files.1.clone()
          ).parse().unwrap();
          notFinished = !progress_locked.finished_patching;
          callback.call(None, &make_args!(me), None).unwrap();
        }
      }
		});
    let mut patcher = self.patcher.clone();
    std::thread::spawn(move || {
      patcher.lock().unwrap().download();
      callback_done.call(None, &make_args!(false,false), None).unwrap();
    });
    true
  }
}

impl sciter::EventHandler for Handler {
	dispatch_script_call! {
		fn check_update(Value, Value);
    fn start_download(Value, Value);
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
  let progress = downloader.get_progress();
  let patcher : Arc<Mutex<Downloader>> = Arc::new(Mutex::new(downloader));
  let update_available : Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(None));
  frame.event_handler(Handler{root:None, patcher: patcher.clone(), progress: progress, update_available: update_available.clone()});
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
