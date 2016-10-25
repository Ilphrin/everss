use std::io;
use std::io::prelude::*;
use std::result;
use glob::glob;
use std::fs;
use std::fs::File;

use streamrss::*;


pub struct Curses {
  streams: Vec<StreamRSS>,
}

impl Curses {
  pub fn new() -> Curses {
    Curses { streams: Vec::new() }
  }
}

impl Irss for Curses {
  fn print_list_feeds(&mut self) {
    let mut i = 0;

    println!("");
    println!("=============");
    for feed in self.streams.iter() {
      println!("[{}] {}", i, feed);
      i += 1;
    }
    println!("=============");
    println!("");
  }

  fn download_feed(&mut self) {
    let mut url = String::new();
    io::stdin().read_line(&mut url).ok().expect("Failed to read line");
    url = String::from(url.trim_right());
    match StreamRSS::new(&url) {
      Ok(feed) => {
        save_feed(&feed);
        self.streams.push(feed);
      }
      Err(why) => println!("Error while loading web feed: {}", why),
    }
  }

  fn load_feeds(&mut self) {
    let files = glob("./feeds/*").unwrap().filter_map(result::Result::ok);

    for file in files {
      let opened_file = File::open(file).unwrap();
      let lines = io::BufReader::new(opened_file).lines().filter_map(result::Result::ok).collect::<Vec<String>>();
      match StreamRSS::import(&lines) {
        Ok(value) => {
          self.streams.push(value);
        }
        Err(why) => println!("[ERR] While loading file feed: {}", why),
      }
    }
  }

  fn remove_feed(&mut self, index: usize) {
    if index >= self.streams.len() {
      println!("Too high value");
      return;
    }
    let mut file = String::new();
    file.push_str("feeds/");
    file.push_str(self.streams[index].name.as_str());
    file.push_str(".rss");
    match fs::remove_file(file.as_str()) {
      Ok(_) => {
        println!("Removed {}", self.streams[index].name);
        self.streams.remove(index);
      },
      Err(why) => println!("Can't remove {}: {}", file, why),
    }
  }
}

