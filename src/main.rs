extern crate roxmltree;

use std::env;

fn print_title() {
  let title = "
  ~~~~~~~~~~~~~~~~~~~~~~~~~~
  Get MyAnimeList! (*°▽°*)
  ~~~~~~~~~~~~~~~~~~~~~~~~~~
  Status codes:
  7: All Anime
  1: Currently Watching
  2: Completed
  3: On Hold
  4: Dropped
  6: Plan to Watch
  -------------------------";
  println!("{}", title);
}

fn main() {
  print_title();

  let args: Vec<String> = env::args().collect();

  let username = &args[1];
  let status = &args[2];

  let url = format!("https://myanimelist.net/animelist/{}?status={}", username, status);
  println!("{}", url);
}
