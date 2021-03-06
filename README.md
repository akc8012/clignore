# CLIgnore [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.2-orange)](https://crates.io/crates/clignore)
> *clignorance is bliss*

CLIgnore is a simple command-line tool to find .gitignore files based on your language or framework. It uses the [GitHub API](https://developer.github.com/v3/) to search for template files from GitHub's official [repo](https://github.com/github/gitignore), so you can automagically download and use them in your projects in no-time.

## Rationale
.gitignore files are annoying and I hate them. While many IDEs and version control tools generate .gitignore files for you, that's not always the case. So instead of constantly Googling for the gitignore repo, finding the right template, copying the template, and pasting it in my project *like a schmuck*... I made a tool that does it for me (and you!)

## Usage
### Find
![Usage Screenshot](screenshot.png)

Use `clignore find` to search based on your language or framework, and select the file you need. If CLIgnore finds only 1 file matching your search, it'll be automatically downloaded.

### Everything else
```
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <token>    Sets the OAuth2 token for authenticated requests [env: TOKEN=]

SUBCOMMANDS:
    authenticated    Checks the GitHub API to see if you have an authorization token present
    find             Finds files by name
    help             Prints this message or the help of the given subcommand(s)
    list             Lists all gitignore template files
```

### Setting an [OAuth2](https://developer.github.com/apps/building-oauth-apps/) token
The GitHub API rate-limits users to 60 requests in 1 hour. If, for some reason, you need more than this, CLIgnore allows you to set an OAuth2 token via an environment variable:
```
TOKEN=<your super-secret token here> clignore find python
```

Authenticated requests can make up to 5000 requests per hour. More on that [here](https://developer.github.com/v3/#rate-limiting).

## Installation
CLIgnore is written in [Rust](https://www.rust-lang.org/). The recommended way to install Rust is from the [official download page](https://www.rust-lang.org/tools/install).

Once Rust is installed, use `cargo install`:
```
cargo install clignore
```

Cargo will build the CLIgnore binary and place it in `$HOME/.cargo`. You'll then (hopefully) be able to use `clignore`.

## Roadmap
- Some kind of CI/CD would be nice
- Along that same line, it would be helpful to have pre-built binaries somewhere
- Haven't done any testing on ~~Windows or~~ Mac. Should probably do that. **(Update: Tested on Windows, it works!)**
- Need to determine the minimum version of rustc necessary to build (add to readme)
- The error handling is pretty lazy. I'm just doing a bunch of `unwrap`s in `main`, so while errors are displayed to the user, they're not very pretty. 
- All the source files are hanging out at the root `src/` because I got confused by the module system. Ideally I'd like to figure this out and move things into folders.