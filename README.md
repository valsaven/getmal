# Get MyAnimeList! `(*°▽°*)`
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fvalsaven%2Fgetmal.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fvalsaven%2Fgetmal?ref=badge_shield)

Returns the data of a particular anime list and status in JSON.

### Install
`pip install getmal`

### Status codes:
```
7: All Anime
1: Currently Watching
2: Completed
3: On Hold
4: Dropped
6: Plan to Watch
```

### Examples
```bash
# Get USER's dropped anime
getmal USER 4

# Get USER's all anime and export it to json file
getmal USER 7 -o USER.json
```

### Build
```bash
python setup.py sdist bdist_wheel
```

-----------------

## ↓↓↓ Currently blocked by MAL: https://myanimelist.net/forum/?topicid=1844975
## SaaS (with Go):
### Using
`https://getmal.herokuapp.com/LIST/USERNAME/STATUS`

### Lists:
```
anime - for anime list
manga - for manga list
```


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fvalsaven%2Fgetmal.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fvalsaven%2Fgetmal?ref=badge_large)