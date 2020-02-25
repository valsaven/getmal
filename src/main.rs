mod lib;

fn main() {
  lib::help();
  let user: lib::User = lib::parse_args();
  lib::get_mal_link(user.username, user.status);
}
