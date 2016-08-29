extern crate rss;
extern crate url;
extern crate curl;
extern crate chrono;
extern crate glob;

mod streamrss;

use std::io;
use std::io::prelude::*;
use glob::glob;
use std::path::Path;
use std::fs::File;

fn main() {
  let mut buffer = String::new();

  loop {
    println!("What do you want to do?");
    println!("[1] List currently saved feeds");
    println!("[2] Save a new feed");
    println!("[3] Remove a feed");
    println!("[4] Leave");
    println!("");

    io::stdin().read_line(&mut buffer).ok().expect("Failed to read line");
    buffer = String::from(buffer.trim_right());
    match buffer.as_str() {
      "1" => {
        println!("==========");
        let files = glob("./feeds/*").unwrap().filter_map(std::result::Result::ok);
        for file in files {
          let opened_file = File::open(file).unwrap();
          let line = io::BufReader::new(opened_file).lines().filter_map(std::result::Result::ok).collect::<Vec<String>>();
          println!("{}", line[0]);


        }
        println!("==========");
      }
      "2" => {
        let mut name = String::new();
        io::stdin().read_line(&mut name).ok().expect("Failed to read line");
        name = String::from(name.trim_right());
        let rss_feed = streamrss::StreamRSS::new(&name);
        streamrss::save_feed(&rss_feed);
      }
      "3" => println!("REMOVING"),
      "4" => break,
      _ => println!("WRONG ANSWER JACK")
    }
    buffer.clear();
  }
}
