# Anid

This tool makes use of ani-cli to download anime episodes to a designated
directory

## Dependencies

- [Just](https://just.systems): Build tool
- [ani-cli](https://github.com/pystardust/ani-cli): Download/scrape backend

## Motivation

I wanted something akin to the arr stack but uses direct downloads instead of
torrent/usenet since
- There aren't many public torrent trackers.
- Many torrents tracked are dead torrents (have 0 seeders).
- The ones that are active often only have 2 - 3 seeders and the
  download/upload rate is glacial 
- Having to manually ssh into my Jellyfin server gets old after 2 - 3 times.
- Do you think I can/would shell out 10 bucks a month or so for a usenet
  provider?


## Installation

Since there isn't a binary package for this project yet, you'll have to build
it from source. To do so, first install all the dependencies listed in the
dependencies section.

To install, run the command

```sh
just install
```

## Configuration

All the configurations for the program is contained within the file located at
`~/.config/anime-downloader/watchlist.toml`. Run the program once to generate
it or write one yourself using the one below as reference.

```toml
[config]
# Seconds to sleep after checking for possible downloads
sleep_secs = sleep_secs

[id]
name = "anime_name"
directory = "~/Videos/target_directory"
select = 1
current_episode = 1 # Optional
```

Where:
- `id` is a valid TOML table name (and isn't "config" for obvious reasons)
- `anime_name` is the name of the anime you're watching
- `target_directory` is the full raw path (/home/user/...) as environment
  variable substitution ($HOME/...) isn't supported yet. (`~` could be used in
  $HOME's place however)
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
