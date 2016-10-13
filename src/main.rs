extern crate rss;
extern crate url;
extern crate curl;
extern crate chrono;
extern crate glob;

mod streamrss;

use std::io;
use std::io::prelude::*;
use glob::glob;
use std::fs::File;

fn print_feeds(ref feeds: &Vec<streamrss::StreamRSS>) {
  println!("==========");
  for feed in feeds.iter() {
    println!("{}: X elements. From {}", feed.name, feed.last_update);
  }
  println!("==========");
}

fn download_feed() {
  let mut name = String::new();
  io::stdin().read_line(&mut name).ok().expect("Failed to read line");
  name = String::from(name.trim_right());
  let rss_feed = streamrss::StreamRSS::new(&name);
  streamrss::save_feed(&rss_feed);
}

fn load_feeds() -> Vec<streamrss::StreamRSS> {
  let mut feeds: Vec<streamrss::StreamRSS> = vec![];
  let mut feed: streamrss::StreamRSS;
  let files = glob("./feeds/*").unwrap().filter_map(std::result::Result::ok);

  for file in files {
    let opened_file = File::open(file).unwrap();
    let lines = io::BufReader::new(opened_file).lines().filter_map(std::result::Result::ok).collect::<Vec<String>>();
    feed = streamrss::StreamRSS::new(&lines[1]);
    feeds.push(feed);
  }
  return feeds
}

fn main() {
  let mut buffer = String::new();
  let feeds = load_feeds();

  loop {
    println!("What do you want to do?");
    println!("[1] List currently saved feeds");
    println!("[2] Download a new feed");
    println!("[3] Remove a feed");
    println!("[4] Import a feed from XML");
    println!("[5] Leave");
    println!("");

    io::stdin().read_line(&mut buffer).ok().expect("Failed to read line");
    buffer = String::from(buffer.trim_right());
    match buffer.as_str() {
      "1" => print_feeds(&feeds),
      "2" => download_feed(),
      "3" => println!("REMOVING"),
      "4" => println!("IMPORTING"),
      "5" => break,
      _ => println!("WRONG ANSWER JACK")
    }
    buffer.clear();
  }
}
