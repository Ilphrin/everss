use std::str;

use rss::*;
use chrono::*;
use url::{Url};
use curl::easy::Easy;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;


struct FeedGetter<'data> {
  feed: &'data mut String
}

pub fn get_feed(buffer: &String) -> Channel {
  let mut feed_str = String::new();
  {
    let getter = FeedGetter{feed: &mut feed_str};
    match Url::parse(buffer.as_str()) {
        Ok(_) => {
          let mut easy = Easy::new();
          easy.url(buffer.as_str()).unwrap();
          let mut transfer = easy.transfer();
          transfer.write_function(|data| {
            match str::from_utf8(data) {
              Ok(elem) => getter.feed.push_str(elem),
              Err(_) => {}
            }
            Ok(data.len())
          }).unwrap();
          transfer.perform().unwrap_or(());
        },
        Err(e) => {
          getter.feed.push_str("ERROR");
          println!("Invalid address: {}", e);
        }
    }
  }
  match feed_str.parse::<Rss>() {
    Ok(value) => {
      let Rss(feed) = value;
      return feed
    },
    Err(why) => {
      panic!("{}", why);
    }
  }
}

pub struct StreamRSS {
  pub url: String,
  pub name: String,
  pub items: Vec<Item>,
  pub object: Channel,
  pub last_update: DateTime<Local>,
  // pub favicon: Image,
  // pub description: String,
}

impl StreamRSS {
  pub fn new(url: &String) -> StreamRSS {
    let feed = get_feed(&url);
    StreamRSS{url: url.clone(), name: feed.title.clone(), items: feed.items.clone(), object: feed, last_update:Local::now()}
  }
}

pub fn save_feed(feed: &StreamRSS) {
  let mut buffer = String::new();
  let mut path = String::new();
  path.push_str("feeds/");

  println!("Give him a name");
  io::stdin().read_line(&mut buffer).ok().expect("Failed to read line");
  buffer = String::from(buffer.trim_right());
  path.push_str(buffer.as_str());
  path.push_str(".rss");

  let path = Path::new(path.as_str());
  let display = path.display();
  let mut file;

  match File::open(&path) {
    Err(why) => {
      println!("{}", why);
      file = match File::create(path) {
        Err(_) => panic!("couldn't create {}", display),
        Ok(file) => file
      };
    }
    Ok(_) => {
      println!("File {} already exists", display);
      return;
    }
  }

  let mut all_file = String::new();
  all_file.push_str(feed.name.as_str());
  all_file.push('\n');
  all_file.push_str(feed.url.as_str());
  all_file.push('\n');
  all_file.push_str(feed.last_update.to_string().as_str());

  match file.write_all(all_file.as_bytes()) {
    Err(_) => println!("couldn't write to {}", display),
    Ok(_) => println!("Successfully wrote to {}", display)
  }
  println!("");
}

