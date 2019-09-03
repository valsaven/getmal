import os
import json
import textwrap

from pathlib import Path
from argparse import ArgumentParser, RawDescriptionHelpFormatter

from bs4 import BeautifulSoup
from urllib.request import urlopen


def set_arguments():
    parser = ArgumentParser(
        description=textwrap.dedent('''\
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
            -------------------------
        '''),
        formatter_class=RawDescriptionHelpFormatter,
    )

    # username
    parser.add_argument(
        'username',
        metavar='username',
        type=str,
        help='Username'
    )

    # status
    parser.add_argument(
        'status',
        metavar='status',
        type=int,
        help='Status code'
    )

    # filename
    parser.add_argument(
        '-o',
        metavar='filename',
        type=str,
        help='Save data to file (Example: -o animelist.json)'
    )

    return parser


def save_to_file(filename, data):
    data_folder = Path(os.getcwd())

    file_to_open = data_folder / filename

    with open(file_to_open, 'a') as out:
        out.write(data)

if __name__ == '__main__':
    parser = set_arguments()
    args = parser.parse_args()

    url = 'https://myanimelist.net/animelist/{}?status={}'.format(args.username, args.status)

    soup = BeautifulSoup(urlopen(url), 'lxml')
    data = soup.select('.list-table')[0]['data-items']

    encoded_data = json.loads(data)

    formatted_data = json.dumps(encoded_data, sort_keys=True, indent=4)
    print(formatted_data)

    if (args.o):
        save_to_file(args.o, formatted_data)
