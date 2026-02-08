# Anid

This tool makes use of ani-cli to download anime episodes to a designated
directory

## Dependencies

- [Just](https://just.systems): build tool
- [ani-cli](https://github.com/pystardust/ani-cli): Download/scrape backend

## Motivation

I wanted something akin to the arr stack but uses direct downloads instead
since
- There aren't many public torrent trackers
- Many torrents tracked are dead torrents (have 0 seeders)
- Having to manually ssh into my Jellyfin server gets old after 2 - 3 times

## Configuration

All the configuration options for this program is contained within the
`config` table, as demonstrated by the example config below

```toml
[config]
# Seconds to sleep after checking for possible downloads
sleep_secs = sleep_secs
```

## Installation

Since there isn't a binary package for this project yet, you'll have to build
it from source. To do so, first install all the dependencies listed in the
dependencies section.

To install, run the command

```sh
just install
```

## Configuration

- Run the program once for it to generate it's configuration directory or
  create the directory (and file) `~/.config/anime-downloader/watchlist.toml`
  yourself
- Make a config file in the following form
```toml
[id]
name = "anime_name"
directory = "target_directory"
select = entry_number
current_episode = episode_number
```
where:
- `id` is a valid TOML table name (and isn't "config" for obvious reasons)
- `anime_name` is the name of the anime you're watching
- `target_directory` is the full raw path (/home/user/...) as environment
  variable substitution ($HOME/...) isn't supported yet nor is ~
- `entry_number` is the entry number to choose in case there are multiple
  matching titles
- `episode_number` is the episode number to begin download (the program will
  automatically increment this number)

While you could run this as you would any other long running process, it is
advisable to start/enable the bundled systemd-unit with

```sh
systemctl start/enable --user anime-downloader
```

## Contribution

All contributions are appreciated. If you have a suggestion/feature request or
bug report, please do so by submitting a new issue or better yet, create a new
pull request. 

## Special thanks

- [ani-cli](https://github.com/pystardust/ani-cli) For the download backend

## Why the name?

Anid is a combination of anime and d (daemon). Very creative, I know
