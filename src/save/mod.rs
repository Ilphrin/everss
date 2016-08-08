use std::str;

use rss::*;
use chrono::*;
use url::{Url};
use curl::easy::Easy;


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
          match transfer.perform() {
            Ok(_) => {},
            Err(e) => println!("Error while performing transfer: {}", e)
          }
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
    Err(_) => {
      panic!("ERROR");
    }
  }
}

pub struct StreamRSS {
  pub url: String,
  pub name: String,
  pub items: Vec<Item>,
  pub object: Channel,
  pub last_update: DateTime<Local>,
  // pub pic: Image,
  // pub description: String,
}

impl StreamRSS {
  pub fn new(url: String) -> StreamRSS {
    let feed = get_feed(&url);
    StreamRSS{url: url, name: feed.title.clone(), items: feed.items.clone(), object: feed, last_update:Local::now()}
  }
}
