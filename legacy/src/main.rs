extern crate roxmltree;
mod lib;

use std::env;

fn main() {
  print_title();

  let args: Vec<String> = env::args().collect();

  let username = &args[1];
  let status = &args[2];

  let url = format!("https://myanimelist.net/animelist/{}?status={}", username, status);
  println!("{}", url);

  // soup = BeautifulSoup(urlopen(url), 'lxml')
  // data = soup.select('.list-table')[0]['data-items']

  // encoded_data = json.loads(data)
  let doc = roxmltree::Document::parse("<rect id='rect1'/>").unwrap();
  let elem = doc.descendants().find(|n| n.attribute("id") == Some("rect1")).unwrap();
  print!("{}", elem.has_tag_name("rect"));

  // formatted_data = json.dumps(encoded_data, sort_keys=True, indent=4)
  // print(formatted_data)
}
