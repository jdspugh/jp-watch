#[cfg(not(target_os="macos"))] extern crate inotify;
#[cfg(not(target_os="macos"))] use inotify::{Inotify, WatchMask};

#[cfg(target_os="macos")] extern crate fsevent;
#[cfg(target_os="macos")] use std::sync::mpsc::channel;
#[cfg(target_os="macos")] use std::thread;

use std::env;

fn main(){
  let path=env::args().nth(1).expect("Missing directory argument");

  #[cfg(not(target_os="macos"))]
  {
    // init
    let mut inotify=Inotify::init().expect("Error initializing inotify");
    inotify.add_watch(&path,WatchMask::MODIFY|WatchMask::CREATE|WatchMask::DELETE|WatchMask::ATTRIB).expect("Error adding inotify watch");

    // loop
    let mut buffer=[0;4096];
    loop {
      let events=inotify.read_events_blocking(&mut buffer).expect("Error reading inotify events");
      for event in events {
        print!("{}",event.name.unwrap().to_string_lossy());
        if event.mask.contains(inotify::EventMask::ISDIR){print!("/")}
        println!();
      }
    }
  }

  #[cfg(target_os="macos")]
  {
    // init
    let (sender,receiver)=channel();
    thread::spawn(move || {
      fsevent::FsEvent::new(vec![path.to_string()]).observe(sender);
    });

    // loop
    loop {
      let r=receiver.recv().unwrap();
      print!("{}",r.path);
      if r.flag.intersects(fsevent::StreamFlags::IS_DIR){print!("/")}
      println!();
    }
  }
}