extern crate reqwest;
extern crate select;

mod lib;

use select::document::Document;
use select::predicate::Name;

fn main() {
  lib::help();
  let user: lib::User = lib::parse_args();
  let url: String = lib::get_mal_link(user.username, user.status);

  // TODO: Remove
  println!("{}", url);

  let res = reqwest::get(&url);

  // Document::from_read(res)?
  //     .find(Name("a"))
  //     .filter_map(|n| n.attr("href"))
  //     .for_each(|x| println!("{}", x));


  let doc = roxmltree::Document::parse(Document::from_read(res)).unwrap();
  println!("{}", doc);



  // soup = BeautifulSoup(urlopen(url), 'lxml')
  // data = soup.select('.list-table')[0]['data-items']

  // encoded_data = json.loads(data)

  // formatted_data = json.dumps(encoded_data, sort_keys=True, indent=4)
  // print(formatted_data)

  // if (args.o):
  //     save_to_file(args.o, formatted_data)
}
