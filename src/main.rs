extern crate rss;
extern crate url;
extern crate curl;
extern crate chrono;

mod save;

use std::str;
use std::io;


fn main() {
  let mut buffer = String::new();

  io::stdin().read_line(&mut buffer).ok().expect("Failed to read line");
  buffer = String::from(buffer.trim_right());
  let rss_feed = save::StreamRSS::new(buffer);
  for elem in rss_feed.items {
    println!("{}", elem.title.unwrap());
  }
  println!("{}", rss_feed.name);
}
