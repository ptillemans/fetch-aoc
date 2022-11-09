Fetch Advent Of Code challenge
==============================

A small rust app to create a project for solving the advent of code challenges.

It will create in the current folder an subfolder with the source code for a small scaffold to start solving the challenge. It downloads the input and the challenge in markdown format.

In previous years I noticed the parsing of the input data is often enough of a task to deal with it separately from the main challenge so that is separated out in a `InputModel` struct which has a stub implementation of the `Try_From` trait to parse the input into. This allows the main solving functions to start from well typed input data.

A Custom `AocError` is derived with *thiserror* which is probably overkill. But overkill is underrated.

You can specify the date in the command line, which defaults to today's date so a strategic application of a cron job can ensure your scaffold is waiting for the challenge in near realtime.

Installation
------------

Build and install it :

    cargo install path=.

You want to install it since you'll want to run it from your challenge folder.

Usage
-----

It is a CLI tool and assumed to be run from the folder where you want to create the challenge projects.

    $ fetch-aoc --help
    Usage: fetch-aoc [OPTIONS] --session <SESSION>

    Options:
      -y, --year <YEAR>        [env: AOC_YEAR=]
      -d, --day <DAY>          [env: AOC_DAY=]
      -s, --session <SESSION>  [env: AOC_SESSION=*******]
      -h, --help               Print help information
      -V, --version            Print version information

The tool has 3 arguments :

 - --year, -y : the year of the challenge
 - --day, -d : the day in december
 - --session, -d: the session id from your logged in browser session

These parameters can also be given using the environment variable *AOC_YEAR*, *AOC_DAY* and *AOC_SESSION*.

 if these date parameters are not given they will be derived from the current date.

 The session can be found by logging into [Advent of Code site](https://adventofcode.com), inspecting the page, go to the network section, refresh the page, and look for the cookies. You'll want the string value of the *session* key. You can stuff that in a *.env* file (if you have direnv or similar enabled) or in your startup shell script. 

After running it will create a folder with content:

    $ fetch-aoc -d 5 -y 2018
    year: 2018
    day: 5
    Good luck
    $ find aoc-2018-day-5
    aoc-2018-day-5
    aoc-2018-day-5/Cargo.toml
    aoc-2018-day-5/src
    aoc-2018-day-5/src/main.rs
    aoc-2018-day-5/data
    aoc-2018-day-5/data/challenge.md
    aoc-2018-day-5/data/input.txt


