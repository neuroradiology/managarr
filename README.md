# managarr - A TUI and CLI to manage your Servarrs

![check](https://github.com/Dark-Alex-17/managarr/actions/workflows/check.yml/badge.svg)
![test](https://github.com/Dark-Alex-17/managarr/actions/workflows/test.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blueviolet.svg)
![LOC](https://tokei.rs/b1/github/Dark-Alex-17/managarr?category=code)
[![crates.io link](https://img.shields.io/crates/v/managarr.svg)](https://crates.io/crates/managarr)
![Release](https://img.shields.io/github/v/release/Dark-Alex-17/managarr?color=%23c694ff)
[![codecov](https://codecov.io/gh/Dark-Alex-17/managarr/graph/badge.svg?token=33G179TW67)](https://codecov.io/gh/Dark-Alex-17/managarr)
![Crate.io downloads](https://img.shields.io/crates/d/managarr?label=Crate%20downloads)

Managarr is a TUI and CLI to help you manage your HTPC (Home Theater PC). Built with 🤎 in Rust!

![library](screenshots/library.png)

## What Servarrs are supported?

- [x] ![radarr_logo](logos/radarr.png) [Radarr](https://wiki.servarr.com/radarr)
- [ ] ![sonarr_logo](logos/sonarr.png) [Sonarr](https://wiki.servarr.com/en/sonarr)
- [ ] ![readarr_logo](logos/readarr.png) [Readarr](https://wiki.servarr.com/en/readarr)
- [ ] ![lidarr_logo](logos/lidarr.png) [Lidarr](https://wiki.servarr.com/en/lidarr)
- [ ] ![prowlarr_logo](logos/prowlarr.png) [Prowlarr](https://wiki.servarr.com/en/prowlarr)
- [ ] ![whisparr_logo](logos/whisparr.png) [Whisparr](https://wiki.servarr.com/whisparr)
- [ ] ![bazarr_logo](logos/bazarr.png) [Bazarr](https://www.bazarr.media/)
- [ ] ![tautulli_logo](logos/tautulli.png) [Tautulli](https://tautulli.com/)

## Try Before You Buy
To try out Managarr before linking it to your HTPC, you can use the purpose built [managarr-demo](https://github.com/Dark-Alex-17/managarr-demo) repository.
Simply run the following command to start a demo:

```shell
curl https://raw.githubusercontent.com/Dark-Alex-17/managarr-demo/main/managarr-demo.sh > /tmp/managarr-demo.sh && bash /tmp/managarr-demo.sh
```

## Installation
### Cargo
If you have Cargo installed, then you can install Managarr from Crates.io:

```shell
cargo install managarr

# If you encounter issues installing, try installing with '--locked'
cargo install --locked managarr
```

### Docker
Run Managarr as a docker container by mounting your `config.yml` file to `/root/.config/managarr/config.yml`. For example:
```shell
docker run --rm -it -v ~/.config/managarr/config.yml:/root/.config/managarr/config.yml darkalex17/managarr
```

You can also clone this repo and run `make docker` to build a docker image locally and run it using the above command.

Please note that you will need to create and popular your configuration file first before starting the container. Otherwise the container will fail to start.

## Features
Key:

| Symbol             | Status    |
|--------------------|-----------|
| :white_check_mark: | Supported |
| :x:                | Missing   |
| :no_entry_sign:    | Won't Add |

### Radarr

| TUI                | CLI                | Feature                                                                                                        |
|--------------------|--------------------|----------------------------------------------------------------------------------------------------------------|
| :white_check_mark: | :white_check_mark: | View your library, downloads, collections, and blocklist                                                       |
| :white_check_mark: | :white_check_mark: | View details of a specific movie including description, history, downloaded file info, or the credits          |
| :white_check_mark: | :white_check_mark: | View details of any collection and the movies in them                                                          |
| :no_entry_sign:    | :white_check_mark: | View your host and security configs from the CLI to programmatically fetch the API token, among other settings |
| :white_check_mark: | :white_check_mark: | Search your library or collections                                                                             |
| :white_check_mark: | :white_check_mark: | Add movies to your library                                                                                     |
| :white_check_mark: | :white_check_mark: | Delete movies, downloads, and indexers                                                                         |
| :white_check_mark: | :white_check_mark: | Trigger automatic searches for movies                                                                          |
| :white_check_mark: | :white_check_mark: | Trigger refresh and disk scan for movies, downloads, and collections                                           |
| :white_check_mark: | :white_check_mark: | Manually search for movies                                                                                     |
| :white_check_mark: | :white_check_mark: | Edit your movies, collections, and indexers                                                                    |
| :white_check_mark: | :white_check_mark: | Manage your tags                                                                                               |
| :white_check_mark: | :white_check_mark: | Manage your root folders                                                                                       |
| :white_check_mark: | :white_check_mark: | Manage your blocklist                                                                                          |
| :white_check_mark: | :white_check_mark: | View and browse logs, tasks, events queues, and updates                                                        |
| :white_check_mark: | :white_check_mark: | Manually trigger scheduled tasks                                                                               |

### Sonarr
- [ ] Support for Sonarr

### Readarr

- [ ] Support for Readarr

### Lidarr

- [ ] Support for Lidarr

### Whisparr

- [ ] Support for Whisparr

### Bazarr

- [ ] Support for Bazarr

### Prowlarr

- [ ] Support for Prowlarr

### Tautulli

- [ ] Support for Tautulli

### The Managarr CLI
Managarr can be used in one of two ways: As a TUI, or as a CLI for managing your Servarrs. 

All management features available in the TUI are also available in the CLI. However, the CLI is
equipped with additional features to allow for more advanced usage and automation.

The CLI can be helpful for automating tasks or for use in scripts. For example, you can use the CLI to trigger a search for a movie, or to add a movie to your library.

To see all available commands, simply run `managarr --help`:

```shell
$ managarr --help
managarr 0.2.1
Alex Clarke <alex.j.tusa@gmail.com>

A TUI and CLI to manage your Servarrs

Usage: managarr [OPTIONS] [COMMAND]

Commands:
  radarr       Commands for manging your Radarr instance
  completions  Generate shell completions for the Managarr CLI
  help         Print this message or the help of the given subcommand(s)

Options:
      --config <CONFIG>               The Managarr configuration file to use
  -h, --help                          Print help
  -V, --version                       Print version
```

All subcommands also have detailed help menus to show you how to use them. For example, to see all available commands for Radarr, you would run:

```shell
$ managarr radarr --help
Commands for manging your Radarr instance

Usage: managarr radarr [OPTIONS] <COMMAND>

Commands:
  add                       Commands to add or create new resources within your Radarr instance
  delete                    Commands to delete resources from your Radarr instance
  edit                      Commands to edit resources in your Radarr instance
  get                       Commands to fetch details of the resources in your Radarr instance
  list                      Commands to list attributes from your Radarr instance
  refresh                   Commands to refresh the data in your Radarr instance
  clear-blocklist           Clear the blocklist
  download-release          Manually download the given release for the specified movie ID
  manual-search             Trigger a manual search of releases for the movie with the given ID
  search-new-movie          Search for a new film to add to Radarr
  start-task                Start the specified Radarr task
  test-indexer              Test the indexer with the given ID. Note that a successful test returns an empty JSON body; i.e. '{}'
  test-all-indexers         Test all indexers
  trigger-automatic-search  Trigger an automatic search for the movie with the specified ID
  help                      Print this message or the help of the given subcommand(s)

Options:
      --config <CONFIG>               The Managarr configuration file to use
  -h, --help                          Print help
```

**Pro Tip:** The CLI is even more powerful and useful when used in conjunction with the `jq` CLI tool. This allows you to parse the JSON response from the Managarr CLI and use it in your scripts; For example, to extract the `movieId` of the movie "Ad Astra", you would run:

```shell
$ managarr radarr list movies | jq '.[] | select(.title == "Ad Astra") | .id'
277
```

# Configuration
Managarr assumes reasonable defaults to connect to each service (i.e. Radarr is on localhost:7878),
but all servers will require you to input the API token.

The configuration file is located somewhere different for each OS

### Linux
```
$HOME/.config/managarr/config.yml
```

### Mac
```
$HOME/Library/Application Support/managarr/config.yml
```

### Windows
```
%APPDATA%/Roaming/managarr/config.yml
```

## Specify Which Configuration File to Use
It can sometimes be useful to specify the configuration file you wish to use. This is useful in cases
where you may have more than one instance of a given Servarr running. Thus, you can specify the 
config file using the `--config` flag:

```shell
managarr --config /path/to/config.yml
```

### Example Configuration:
```yaml
radarr:
  host: 192.168.0.78
  port: 7878
  api_token: someApiToken1234567890
  ssl_cert_path: /path/to/radarr.crt # Required to enable SSL
sonarr:
  uri: http://htpc.local/sonarr # Example of using the 'uri' key instead of 'host' and 'port'
  api_token: someApiToken1234567890
readarr:
  host: 192.168.0.87
  port: 8787
  api_token: someApiToken1234567890
lidarr:
  host: 192.168.0.86
  port: 8686
  api_token: someApiToken1234567890
whisparr:
  host: 192.168.0.69
  port: 6969
  api_token: someApiToken1234567890
  ssl_cert_path: /path/to/whisparr.crt
bazarr:
  host: 192.168.0.67
  port: 6767
  api_token: someApiToken1234567890
prowlarr:
  host: 192.168.0.96
  port: 9696
  api_token: someApiToken1234567890
tautulli:
  host: 192.168.0.81
  port: 8181
  api_token: someApiToken1234567890
```

## Environment Variables
Managarr supports using environment variables on startup so you don't have to always specify certain flags:

| Variable                                | Description                                                                    | Equivalent Flag                  |
|-----------------------------------------|--------------------------------------------------------------------------------|----------------------------------|
| `MANAGARR_CONFIG_FILE`                  | Set the path to the config file                                                | `--config`                       |
| `MANAGARR_DISABLE_SPINNER`              | Disable the CLI spinner (this can be useful when scripting and parsing output) | `--disable-spinner`              |
|-----------------------------------------|--------------------------------------------------------------------------------|----------------------------------|

## Track My Progress for the Beta release (With Sonarr Support!)
Progress for the beta release can be followed on my [Wekan Board](https://wekan.alexjclarke.com/b/dHoGjBb44MHM9HSv4/managarr)
with all items tagged `Beta`.

## Screenshots

![library](screenshots/library.png)
![manual_search](screenshots/manual_search.png)
![logs](screenshots/logs.png)
![new_movie_search](screenshots/new_movie_search.png)
![add_new_movie](screenshots/add_new_movie.png)
![collection_details](screenshots/collection_details.png)
![indexers](screenshots/indexers.png)

## Dependencies
* [ratatui](https://github.com/tui-rs-revival/ratatui)
* [crossterm](https://github.com/crossterm-rs/crossterm)
* [clap](https://github.com/clap-rs/clap)
* [tokio](https://github.com/tokio-rs/tokio)
* [serde](https://github.com/serde-rs/serde)
* [reqwest](https://github.com/seanmonstar/reqwest)

## Servarr Requirements
* [Radarr >= 5.3.6.8612](https://radarr.video/docs/api/)
* [Sonarr >= v3](https://sonarr.tv/docs/api/)
* [Readarr v1](https://readarr.com/docs/api/)
* [Lidarr v1](https://lidarr.audio/docs/api/)
* [Whisparr >= v3](https://whisparr.com/docs/api/)
* [Prowlarr v1](https://prowlarr.com/docs/api/)
* [Bazarr v1.1.4](http://localhost:6767/api)
* [Tautulli >= v2](https://github.com/Tautulli/Tautulli/wiki/Tautulli-API-Reference)

## Creator
* [Alex Clarke](https://github.com/Dark-Alex-17)
