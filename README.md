# Dama
![](assets/icon.png)

## Disclaimer

I don't know what I am doing, this is a learning experience for me.
I intend for this to become a usable menu, but it might take a while for me to get there.

## Desktop Agnostic Menu Aggregate

This program aims to be a hackable, easy to use menu that can be paired to 
lightweight window managers in order to change settings on the fly.

## Looks

here's a screenshot with the `adapta` gtk theme

![](assets/screenshot.png)

## feature roadmap

- [x] correctly render hardcoded components
- [x] parse components from a `json` file at startup
  - [x] look in multiple locations before giving up
- [X] interact with provided scripts
  - [x] execute commands (e.g. change brightness with a slider)
  - [X] get values back from commands (e.g. set the correct value for the brightness slider at startup)
- [ ] add support for image widgets
- [ ] add support for checkbox widgets
- [ ] style the layout in a sensible way
  - [x] allow setting names for tabs
  - [ ] move the tab list to the side
  - [x] make sliders fill all horizontal available space


## Dependencies

Dama is built with rust, using the `gtk` crate and uses `cargo` to manage the build process.

## Installation

A makefile will be provided in the near future.

## writing your own menu entries

menu entries are read from a file called `dama.json`.
The program will look for it in `$XDG_CONFIG_HOME/dama/config.json` 
if the variable is set, or in `$HOME/.config/dama.json` if it is not.

if that file doesn't exist, dama will try to read from `$HOME/.dama.json`.

Available entries are of types:

```
{"Notebook" : [
      // list of children
]},

{"Box": [ "name", 
	  // this is used to set the tab's name if the box 
	  // is a direct child of a notebook.
	  // Otherwise, it is ignored and can be left empty.
	  "Vertical", 
          // or  Horizontal
      [
	  // list of children
      ]
]},

{"Label": ["some text"]},

{"Button": ["the button's label", 
            "notify-send \"click!\""]
            // the command to be executed on click 
}, 

{"Scale": [0.0,   
           // the minimum value
           100.0,  
           // the maximum value
           "xbacklight -get",
           // the command to run in order to get the initial value.
           // this will be clamped between maximum and minimum values.
           "xbacklight -set"] 
           // the command to be executed when the slider is moved.
           // the current value of the slider is added to the end                               
           // of this string, rounded to an integer.
}                             
```

all commands are executed with `sh -c`.

