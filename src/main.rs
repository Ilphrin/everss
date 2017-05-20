extern crate rss;
extern crate url;
extern crate curl;
extern crate chrono;
extern crate glob;
extern crate termion;

#[macro_use]
pub mod streamrss;
pub mod terminal;

use streamrss::*;

use std::io;

fn main() {
  print!("{clear}{goto}", clear = termion::clear::All , goto = termion::cursor::Goto(1,1));
  let mut buffer = String::new();
  let mut term: terminal::Curses = terminal::Curses::new();
  streamrss::load_feeds(&mut term.streams);

  loop {
    println!("What do you want to do?");
    println!("[1] List currently saved feeds");
    println!("[2] Download a new feed");
    println!("[3] Remove a feed");
    println!("[4] Show news");
    println!("[5] Leave");
    println!("");

    io::stdin().read_line(&mut buffer).ok().expect("Failed to read line");
    buffer = String::from(buffer.trim_right());
    print!("{clear}{goto}", clear = termion::clear::All , goto = termion::cursor::Goto(1,1));
    match buffer.as_str() {
      "1" => term.print_list_feeds(),
      "2" => term.download_feed(),
      "3" => term.remove_feed(),
      "4" => term.output_news(),
      "5" => break,
      _ => println!("WRONG ANSWER :'( ")
    }
    buffer.clear();
  }
}
