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
- The few ones that aren't dead usually have only 2 - 3 seeders, making
  download/upload speed glacial
- Having to manually ssh into my Jellyfin server gets old after 2 - 3 times
- Do you think I can/would shell out 10 bucks or so per month for a usenet
  provider? 


## Installation

Since there isn't a binary package for this project yet, you'll have to build
it from source. To do so, first install all the dependencies listed in the
dependencies section then run.

```sh
just install
```

To build and install the program along with in bundled systemd unit.

## Configuration

All the configuration options for this program is contained within the file
located at `~/.config/anid/watchlist.toml`. To get started, run the program
once or write one yourself using the following example as reference.

```toml
[config]
# Seconds to sleep after checking for possible downloads
sleep_secs = sleep_secs

[id]
name = "anime_name"
directory = "target_directory"
select = entry_number
current_episode = episode_number
rename = "Sousou no Frieren Season 2 Episode {episode+x}"
```
where:
- `id` is a valid TOML table name (and isn't "config" for obvious reasons)
- `anime_name` is the name of the anime you're watching
- `target_directory` is the full raw path (/home/user/...) as environment
  variable substitution ($HOME/...) isn't supported yet nor is ~
- `entry_number` is the entry to choose in case there are multiple matching
  titles
- `episode_number` is the episode number to begin download (the program will
  automatically increment this number)

For more info on `rename`, please read the config example located in
`examples/watchlist.toml`

## Usage

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
