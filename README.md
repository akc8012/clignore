# CLIgnore
> *clignorance is bliss*

CLIgnore is a simple command-line tool to find .gitignore files based on your language or framework. It uses the [GitHub API](https://developer.github.com/v3/) to search for template files from GitHub's official [repo](https://github.com/github/gitignore), so you can automagically download and use them in your projects in no-time.

## Rationale
.gitignore files are annoying and I hate them. While many IDEs and version control tools generate .gitignore files for you, that's not always the case. So instead of constantly Googling for the gitignore repo, finding the right template, copying the template, and pasting it in my project *like a schmuck*... I made a tool that does it for me (and you!)

## Usage
### Find
![Usage Screenshot](screenshot.png)

Use `clignore find` to search based on your langauge or framework, and select the file you need. If CLIgnore finds only 1 file matching your search, it'll be automatically download.

### Everything else
```
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <token>    Sets the GitHub authorization token [env: TOKEN=]

SUBCOMMANDS:
    authenticated    Checks the GitHub API to see if you have an authorization token present
    find             Finds files by name
    help             Prints this message or the help of the given subcommand(s)
    list             Lists all gitignore template files

SUBCOMMANDS:

    authenticated    Checks the GitHub API to see if you have an authorization token present
    find             Finds files by name
    help             Prints this message or the help of the given subcommand(s)
    list             Lists all gitignore template files
```

### Setting an auth token
If, for some reason, you need to make over 60 requests in 1 hour, CLIgnore allows you to set an auth token via an enviornment variable:

`TOKEN=9431e108b67d1efa9df54e6351da1951bcd9be32 clignore find python`

## Installation

## Roadmap
- Some kind of CI/CD would be nice
- The error handling is pretty lazy. I'm just doing a bunch of `unwrap`s in `main`, so while errors are displayed to the user, they're not very pretty. 