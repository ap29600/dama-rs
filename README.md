# Dama
![](assets/icon2.png)

## Desktop Agnostic Menu Aggregate

This program aims to be a hackable, easy to use menu that can be paired to 
lightweight window managers in order to change settings on the fly.

**This is a learning experience for me, most of what i'm doing is probably not a best practice.**

## Looks

here's a screenshot with the `adapta` gtk theme

![](assets/screenshot.png)

## feature roadmap

**version 1.0 has been reached, possibly breaking changes will not occur on the main branch**

changes might still be made to layout styling / anything that doesn't involve user configuration files

- [x] correctly render hardcoded components
- [x] parse components from a file at startup
  - [x] look in multiple locations before giving up
- [X] interact with provided scripts
  - [x] execute commands (e.g. change brightness with a slider)
  - [X] get values back from commands (e.g. set the correct value for the brightness slider at startup)
  - [X] run `Scale` commands asynchronously to avoid blocking the main thread 
    (thread sync method kindly suggested by Alice Ryhl)
- [x] add support for image widgets
  - [ ] dynamically resize the image
- [x] add support for checkbox widgets
- [x] style the layout in a sensible way
  - [x] allow setting names for tabs
  - [x] move the tab list to the side


## Dependencies

Dama is built with rust, using the `gtk` crate and uses `cargo` to manage the build process.

## Installation


just run the command:
```
make install
```

## writing your own menu entries

menu entries are read from locations listed in a file called `config`.
The program will look for it in `$XDG_CONFIG_HOME/dama/` 
if the variable is set, or in `$HOME/.config/dama` if it is not.

if that file doesn't exist, dama will try to read from `$HOME/.dama/config`.

each line of your `config` should be the full path to a yaml or json file describing a menu page.
This page must consist of exactly one top-level widget, which may have children.

Available wigets are of types:

```yaml
Notebook :
    -   # child 1
    -   # child 2
        # etc ... 

Box : 
    - "name"
         # this is used to set the tab's name if the box 
         # is a direct child of a notebook.
         # Otherwise, it is ignored and can be left empty.
    - "Vertical" # or  "Horizontal"
    -    -    # child 1
         -    # child 2
              # etc ... 

Label : "some text"

Image : "/absolute/path/to/image"
        # the image will not be resized, you will have to resize 
        # the source file for the time being

Button : 
    - "the button's label"
    - "notify-send \"click!\""
        # the command to be executed on click 

Checkbox : 
    - "the label"
    - "echo true"
        # command providing initial state
    - "notify-send $DAMA_VAL"]
        # the command to be executed on toggle
        # the value $DAMA_VAL is available and
        # contains the target state fo the checkbox.

Scale : 
    - 0.0  
        # the minimum value
    - 100.0
        # the maximum value
    - "xbacklight -get"
        # the command to run in order to get the initial value.
        # this will be clamped between maximum and minimum values.
    - "xbacklight -set $DAMA_VAL"
        # the command to be executed when the slider is moved.
        # the target value of the slider is available through                              
        # the environment variable $DAMA_VAL, rounded to an integer.
```

Or with the json syntax:

```json
{ "Notebook" : 
      [ { "child 1"},  
      {"child 2"},
      {" ... "} ]
},

{ "Box" : 
    [ "name",
      "Vertical",
      [ { "child 1"},  
      {"child 2 ..."} ]
    ]
},

{ "Label" : "some text" },

{ "Image" : "/absolute/path/to/image" },

{ "Button" : ["text" , "command"] }, 

{ "Checkbox" : [ "text", "initial command", "update command"] },

{ "Scale" : [0.0 , 100.0,  "initial command", "update command"] } 

```

A toplevel `Notebook` is implicitly added as a container for your pages. page names are handled
by reading the label of a top-level box, and user-defined Notebooks also behave this way.

all commands are executed with `sh -c`.

In a horizontal `Box`, if the first element is a `Label`, it will expand to push
the remaining elements to the right of the window. This should result in a tidier layout:

```
Without label expansion:
 /----------------------------------------------\
 | Regular Label  [Btn][Btn]                    | 
 | Slightly longer Label  [Btn]                 | 
 | Short Label  [Large Button]                  |
 \----------------------------------------------/

With label expansion:
 /----------------------------------------------\
 | Regular Label                    [Btn][Btn]  |
 | Slightly longer Label                 [Btn]  | 
 | Short Label                  [Large Button]  |
 \----------------------------------------------/
```




