use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn print_title() {
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

// pub fn save_to_file(filename, data) {
//   fn main() -> std::io::Result<()> {
//     let mut file = File::create("foo.txt")?;
//     file.write_all(b"Hello, world!")?;
//     Ok(())
// }

// }

pub fn save_to_file(filename: &str, data: &str) -> std::io::Result<()> {
  let mut file = File::create(filename)?;

  fs::write(file, data.as_bytes())?;

  Ok(())
}
    // data_folder = Path(os.getcwd())

    // file_to_open = data_folder / filename

    // with open(file_to_open, 'a') as out:
    //     out.write(data)


// def set_arguments():
//     parser = ArgumentParser(
//         description=textwrap.dedent('''\
//             ~~~~~~~~~~~~~~~~~~~~~~~~~~
//              Get MyAnimeList! (*°▽°*)
//             ~~~~~~~~~~~~~~~~~~~~~~~~~~
//             Status codes:
//               7: All Anime
//               1: Currently Watching
//               2: Completed
//               3: On Hold
//               4: Dropped
//               6: Plan to Watch
//             -------------------------
//         '''),
//         formatter_class=RawDescriptionHelpFormatter,
//     )

//     # username
//     parser.add_argument(
//         'username',
//         metavar='username',
//         type=str,
//         help='Username'
//     )

//     # status
//     parser.add_argument(
//         'status',
//         metavar='status',
//         type=int,
//         help='Status code'
//     )

//     # filename
//     parser.add_argument(
//         '-o',
//         metavar='filename',
//         type=str,
//         help='Save data to file (Example: -o animelist.json)'
//     )

//     return parser


