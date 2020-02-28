use std::env;

pub struct User {
  pub username: String,
  pub status: u8,
}

pub fn help() {
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

  Examples:
  # Get USER's dropped anime
  getmal USER 4

  # Get USER's all anime and export it to json file
  getmal USER 7 -o USER.json
  -------------------------";
  println!("{}", title);
}

pub fn parse_args() -> User {
  let args: Vec<String> = env::args().collect();

  match args.len() {
      // one command and one argument passed
      3 => {
          let username: String = String::from(&args[1]);
          // let status: u8 = String::from(&args[2]).parse().unwrap();
          let s = &args[2];

          // parse the number of status
          let status: u8 = match s.parse() {
            Ok(n) => {
              n
            },
            Err(_) => {
              panic!("error: second argument not an integer");
            },
          };

          let statuses = [1, 2, 3, 4, 6, 7];

          if !statuses.contains(&status) {
            panic!("error: status doesn't exist (existing: 1, 2, 3, 4, 6, 7)");
          }

          let user: User = User {
            username,
            status
          };

          return user;
      },
      _ => {
          panic!("error:  invalid number of arguments.");
      }
  }
}


pub fn get_mal_link(username: String, status: u8) -> String {
  let url = format!("https://myanimelist.net/animelist/{}?status={}", username, status);
  return url;
}
