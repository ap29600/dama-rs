# Dama
![](assets/icon.png)

## Disclaimer

I don't know what I am doing, this is a learning experience for me.
I intend for this to become a usable menu, but it might take a while for me to get there.

## Desktop Agnostic Menu Aggregate

This program aims to be a hackable, easy to use menu that can be paired to 
lightweight window managers in order to change settings on the fly.


## feature roadmap

[x] correctly render hardcoded components
[ ] parse components from a `json` file at startup
[ ] interact with provided scripts (e.g. set the correct value for the brightness slider at startup)

## Dependencies

Dama is built with rust, using the `relm` and `gtk` crates and uses `cargo` to manage the build process.

## Installation

A makefile will be provided in the near future.

## writing your own pages

This is currently only possible by modifying the source code.
I plan to use a json parser to deserialize the layout from a `dama.json` file
