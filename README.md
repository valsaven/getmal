# Get MyAnimeList! `(*°▽°*)`
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

## SaaS (with Go):
### Using
`https://enigmatic-island-24693.herokuapp.com/LIST/USERNAME/STATUS`

### Lists:
```
anime - for anime list
manga - for manga list
```
