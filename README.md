# Dama
![](assets/icon.png)

## Disclaimer

I don't know what I am doing, this is a learning experience for me.
I intend for this to become a usable menu, but it might take a while for me to get there.

## Desktop Agnostic Menu Aggregate

This program aims to be a hackable, easy to use menu that can be paired to 
lightweight window managers in order to change settings on the fly.


## feature roadmap

- [x] correctly render hardcoded components
- [x] parse components from a `json` file at startup
- [] interact with provided scripts
  - [x] execute commands (e.g. change brightness with a slider)
  - [ ] get values back from commands (e.g. set the correct value for the brightness slider at startup)
- [ ] add support for image widgets
- [ ] style the layout in a sensible way
  - [ ] allow setting names for tabs
  - [ ] move the tab list to the side
  - [ ] make sliders fill all horizontal available space


## Dependencies

Dama is built with rust, using the `gtk` crate and uses `cargo` to manage the build process.

## Installation

A makefile will be provided in the near future.

## writing your own menu entries

menu entries are read from a file called `dama.json` in the directory you launch the executable in,
but in the future its default location will be `XDG_CONFIG_HOME/dama/config.json`.

Available entries are of types:

```json
{"Notebook" : [/* list of children*/]},

{"Box": ["Vertical", // or  Horizontal
      [/*list of children*/]
]},

{"Label": ["some text"]},

{"Button": ["the button's label", 
    "notify-send \"click!\""]}, // the command to be executed

{"Scale": [0.0,   // the minimum value
           100.0, // the maximum value
           "xbacklight -set"] // the command to be executed when the slider is moved.
}                             // the current value of the slider is added to the end 
                              // of this string, rounded to an integer.
```

all commands are executed with `sh -c`.

The top-level object must be of type `Notebook`, but nesting is not restricted beyond that.

