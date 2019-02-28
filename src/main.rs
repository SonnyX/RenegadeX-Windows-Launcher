//#![windows_subsystem="windows"]

extern crate reqwest;
extern crate json;
extern crate sciter;
extern crate renegadex_patcher;

mod traits;

use std::env::args;

use std::process;
use std::io;
use std::fs::File;

use sciter::{Value, Element, HELEMENT};
use sciter::dom::event::*;

use renegadex_patcher::Downloader;
use traits::JsonExtend;

struct Handler;

impl Handler {
}

impl sciter::EventHandler for Handler {
  fn get_subscription(&mut self) -> Option<EVENT_GROUPS> {
    Some(default_events() | EVENT_GROUPS::HANDLE_TIMER)
  }

  fn attached(&mut self, root: sciter::HELEMENT) {
  }

  fn detached(&mut self, root: sciter::HELEMENT) {
  }

  fn on_event(&mut self, root: HELEMENT, source: HELEMENT, target: HELEMENT, code: BEHAVIOR_EVENTS, phase: PHASE_MASK, reason: EventReason) -> bool {
    false
  }

  fn on_timer(&mut self, root: HELEMENT, timer_id: u64) -> bool {
    false
  }

  fn document_complete(&mut self, root: sciter::HELEMENT, source: sciter::HELEMENT) {
    println!("Loaded page!");
    /*
    let root = Element::from(root);
    let mut smth = match root.find_first("tbody").unwrap() {
      Some(mut tbody) => {
        let servers_url = "http://serverlist.renegade-x.com/servers.jsp";
        match reqwest::get(servers_url) {
          Ok(mut servers_response) => {
            let servers_text = servers_response.text().unwrap();
            let servers_data = match json::parse(&servers_text) {
              Ok(result) => result,
              Err(e) => panic!("Invalid JSON: {}", e)
            };
            for server_entry in servers_data.into_inner() {
              //let mut tbody = tbody_wrapped.unwrap();
              let mut entry = Element::with_parent("tr", &mut tbody).unwrap();
              entry.set_attribute("class", "server-entry");
              let mut server = Element::with_text("td", &server_entry["Name"].as_string()).unwrap();
              server.set_attribute("name","name");
              entry.append(&server).expect("wtf?");
              let mut map = Element::with_text("td", &server_entry["Current Map"].as_string()).unwrap();
              server.set_attribute("name","map");
              entry.append(&map).expect("wtf?");
              let mut players = Element::with_text("td", &format!("{}/{}", server_entry["Players"].as_u64().unwrap(), server_entry["Variables"]["Player Limit"].as_u64().unwrap())).unwrap();
              server.set_attribute("name","players");
              entry.append(&players).expect("wtf?");
              let mut ping = Element::with_text("td", "18ms").unwrap();
              server.set_attribute("name","latency");
              entry.append(&ping).expect("wtf?");
            }
          },
          Err(e) => println!("Is your internet down? {}", e)
        };

      },
      None => {}
    };
    //let smth = body.select("table");
    //println!("{:?}", smth);
    */
  }
}

fn main() {
  // Step 1: Include the 'minimal.html' file as a byte array.
  // Hint: Take a look into 'minimal.html' which contains some tiscript code.
  //let html = include_bytes!("minimal.htm");
  let mut patcher : Downloader = Downloader::new();
  sciter::set_options(
    sciter::RuntimeOptions::ScriptFeatures(
      sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8 | // Enables Sciter.machineName()
      sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8 | // Enables opening file dialog (view.selectFile())
      sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SOCKET_IO as u8
    )
  ).unwrap(); // Enables connecting to the inspector via Ctrl+Shift+I
  let mut frame = sciter::Window::new();
  frame.event_handler(Handler);
  let mut path = std::env::current_exe().unwrap();
  path.pop();
  path.push("dom/frontpage.htm");
  frame.load_file(path.to_str().unwrap());
  //frame.load_html(html, Some("example://minimal.htm"));
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

  fn download_wine(&mut self, version: String) {
    //grab wine version from play-on-linux, wine itself, or from lutris.
    //...
    self.instantiate_wine_prefix();
  }

  //Checks if the (paranoid) kernel blocks ICMP by programs such as wine, otherwise prompt the user to enter password to execute the followiwng commands
  fn ping_test(&mut self) {
    let successful = false;
    
    if !successful {
      let mut wine_location = self.RenegadeX_location.clone().unwrap();
      wine_location.push_str("libs/wine/bin/wine64");
      let mut pkexec = process::Command::new("pkexec")
        .args(&["--user","root","setcap","cap_net_raw+epi",wine_location.as_str()])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn().expect("failed to execute child");
      pkexec.wait();
      /*
      Need to use pkexec to show the user a dialog questioning to allow executing setcap in order to allow the launcher (and wine?) to ping.
      https://wiki.archlinux.org/index.php/Polkit

      sudo setcap cap_net_raw+epi /usr/bin/wine-preloader
      sudo setcap cap_net_raw+epi /usr/bin/wine
      sudo setcap cap_net_raw+epi /usr/bin/wine64-preloader
      sudo setcap cap_net_raw+epi /usr/bin/wine64
      */
    }
  }

  //Checks if wine prefix exists, if not create it, install necessary libraries.
  fn instantiate_wine_prefix(&mut self) {
    //at the very least we need vcrun2005 and dotnet40 (or perhaps mono works)
    //what else? corefonts, vcrun2008 and vcrun2010 probs? xact,
    //overides?
    //At some point we may be able to use VK9 to improve performance.
    let mut wine_location = self.RenegadeX_location.clone().unwrap();
    wine_location.push_str("libs/wine/bin/wine64");
    //process::Command::new(wine_location)
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
/*
      return process::Command::new("strace")
        .arg("-e")
        .arg("openat")
        .arg("-f")
        .arg(wine_location)
*/
      return process::Command::new(wine_location)
        //.env("WINEDEBUG","fixme-all,warn-dll,-heap")
        .env("WINEARCH","win64")
        .env("WINEPREFIX",wine_prefix)
        .env("DRI_PRIME", "1")
        .arg(game_location)
        //.arg("5.39.74.177:7777")
        .arg("-nomoviestartup")
        .arg("-ini:UDKGame:DefaultPlayer.Name=SonnyX")	
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn().expect("failed to execute child");

    } else {

      let mut wine_location = self.RenegadeX_location.clone().unwrap();
      wine_location.push_str("libs/wine/bin/wine");
      let mut game_location = self.RenegadeX_location.clone().unwrap();
      game_location.push_str("game_files/Binaries/Win64/UDK.exe");

      return process::Command::new(wine_location)
        .arg(game_location)
        .arg("some server")
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn().expect("failed to execute child");
    }
  }

  /* Check if there are wine processes, if so prompt the user if these should be killed */
  pub fn kill_wine_instances() {
    let mut killall = process::Command::new("pkexec")
      .arg("killall")
      .arg("-9 wineserver winedevice.exe UDK.exe plugplay.exe services.exe explorer.exe mscorsvw.exe")
      .stdout(process::Stdio::piped())
      .stderr(process::Stdio::inherit())
      .spawn().expect("failed to execute child");
     killall.wait();
  }

}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn launcher() {
    let mut launcher_instance : Launcher = Launcher::new("/home/sonny/RenegadeX/".to_string());
    let mut child = launcher_instance.launch_game(None);
    if child.wait().expect("failed to wait on child").success() {
      println!("Succesfully terminated wine");
      assert!(false);
    } else {
      println!("Child exited with exit code:");
      //Launcher::kill_wine_instances();
      assert!(false);
    }
  }
}
