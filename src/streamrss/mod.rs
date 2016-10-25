use std::str;
use std::io::prelude::*;
use std::io;
use std::fs::{File, DirBuilder};
use std::path::Path;
use std::fmt;

use rss::*;
use chrono::*;
use url::{Url};
use curl::easy::Easy;

#[warn(missing_docs)]
pub fn get_feed(buffer: &String) -> Result<Rss, ReadError> {
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
  feed_str.parse::<Rss>()
}

#[warn(missing_docs)]
pub fn save_feed(feed: &StreamRSS) {
  let mut path = String::new();
  path.push_str("feeds/");

  match DirBuilder::new().create("feeds/") {
    Ok(_) => println!("[INFO] No feeds/ folder, creating one"),
    Err(_) => {},
  }

  path.push_str(feed.name.as_str());
  path.push_str(".rss");

  let path = Path::new(path.as_str());
  let display = path.display();
  let mut file;

  match File::open(&path) {
    Err(_) => {
      file = match File::create(path) {
        Err(_) => panic!("couldn't create {}", display),
        Ok(file) => file
      };
    }
    Ok(_) => {
      println!("[WARN] File {} already exists", display);
      return;
    }
  }

  let mut all_file = String::new();
  all_file.push_str(feed.name.as_str());
  all_file.push('\n');
  all_file.push_str(feed.url.as_str());
  all_file.push('\n');
  all_file.push_str(feed.last_update.format("%Y-%m-%d %H:%M:%S %:z").to_string().as_str());

  match file.write_all(all_file.as_bytes()) {
    Err(_) => println!("[ERROR] Couldn't write to {}", display),
    Ok(_) => println!("[SUCCESS] Successfully wrote to {}", display)
  }
  println!("");
}

struct FeedGetter<'data> {
  feed: &'data mut String
}

/// Main class for managing RSS streams, loaded from and XML file
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
  /// Download a new XML file from an url and create a StreamRSS from it
  pub fn new(url: &String) -> Result<StreamRSS, &str> {
    //! We get a new StreamRSS from an XML file downloaded on the web with curl
    //! # Example
    //!
    //! ```
    //! use std::io;
    //! let mut url = String::new();
    //! io::stdin().read_line(&mut url).ok().expect("Failed to read line");
    //! url = String::from(url.trim_right());
    //! match streamrss::StreamRSS::new(&url) {
    //!   Ok(_) => println!("Successfully loaded"),
    //!   Err(why) => println!("Error while loading web feed: {}", why),
    //! }
    //! ```

    match get_feed(&url) {
      Ok(value) => {
        let Rss(feed) = value;
        Ok(StreamRSS{url: url.clone(), name: feed.title.clone(), items: feed.items.clone(), object: feed, last_update:Local::now()})
      }
      Err(_) => Err("Failed to get feed")
    }
  }

  pub fn import<'a>(data: &'a Vec<String>) -> Result<StreamRSS, &'a str> {
    match StreamRSS::new(&data[1]) {
      Ok(mut feed) => {
        feed.update(&data[2]);
        feed.new_name(&data[0]);
        Ok(feed)
      }
      Err(error) => Err(error)
    }
  }

  /// Change the last_update value from the StreamRSS feed
  pub fn update(&mut self, data: &str) {
    self.last_update = Local.datetime_from_str(data, "%Y-%m-%d %H:%M:%S %:z").unwrap();
  }

  /// Change the name of the StreamRSS feed
  pub fn new_name(&mut self, data: &String) {
    self.name = data.clone();
  }
}

impl fmt::Display for StreamRSS {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {} elements. From {}", self.name, self.items.len(), self.last_update)
  }
}

pub trait Irss {
  /// This method should be implemented to print the information about each
  /// Stream of a Vec
  fn print_list_feeds(&mut self);

  /// This method ask for a feed to download, and add it to a Vec of StreamRSS
  /// passed as argument
  fn download_feed(&mut self);

  /// This method load the feed from the feeds/ folder
  fn load_feeds(&mut self);

  /// This method remove the feed from the feeds/ folder
  fn remove_feed(&mut self, index: usize);
}

