# Dama
![](assets/icon2.png)

## Desktop Agnostic Menu Aggregate

This program aims to be a hackable, easy to use menu that can be paired to
lightweight window managers in order to change settings on the fly.

**This is a learning experience for me, most of what i'm doing is probably not a best practice.**

## Looks

here's a screenshot with the `adapta` gtk theme

![](assets/screenshot.png)

**branches named with version numbers are frozen releases, though the main branch
should always be kept functional and backwards compatible**

## feature roadmap

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
  - [x] import a CSS stylesheet
  - [x] allow setting names for tabs
  - [x] move the tab list to the side


## Dependencies

Dama is built with rust, using the `gtk` crate and uses `cargo` to manage the build process.

## Installation


just run the command:
```
make install
```

## Styling

Dama will parse a css file located at either `$XDG_CONFIG_HOME/dama/style.css`
or `$HOME/.dama/style.css`, with the former taking precedence.

## Configuration

menu entries are read from locations listed in a file called `config`. The
program will look for it in `$XDG_CONFIG_HOME/dama/` if the variable is set, or
in `$HOME/.config/dama` if it is not.

if that file doesn't exist, dama will try to read from `$HOME/.dama/config`.

each line of your `config` should be the full path to a yaml or json file
describing a menu page. This page must consist of exactly one top-level widget,
which may have children.

you can specify single pages to be loaded as command-line arguments, for
example `dama -p:/home/user/page1.yml -p:/home/user/page2.yml`, in which case
the configuration file is never read. The paths need to be absoltue.


Available wigets are of types:

```yaml
# shows one widget at a time
Notebook :
  children:
    -  # child 1
    -  # child 2
       # etc ... 

# organizes other widgets
Box : 
  title: "name"  # (Optional)
         # this is used to set the tab's name if the box 
         # is a direct child of a notebook.
         # Otherwise, it is ignored and can be left empty.
  orientation: "Vertical" # or  "Horizontal" (Optional)
  children:
    -  # child 1
    -  # child 2
    -  # etc ... 

# displays some text
Label:
  text: "some text"

# displays a picture
Image : 
  path: "/absolute/path/to/image"
        # the image will not be resized, you will have to resize 
        # the source file for the time being

# performs an action, but has no state
Button : 
  text: "the button's label" # (Optional)
  on_click: "notify-send \"click!\""
        # the command to be executed on click 

# holds a boolean value
CheckBox : 
  text: "the label" # (Optional)
  initialize: "echo true"
        # command providing initial state
  on_click: "notify-send $DAMA_VAL"]
        # the command to be executed on toggle
        # the value $DAMA_VAL is available and
        # contains the target state fo the checkbox.

# a linear slider
Scale : 
  range: { low:  0.0, high: 100.0 } # (Optional)
  initialize: "xbacklight -get"
        # the command to run in order to get the initial value.
        # this will be clamped between maximum and minimum values.
  on_update: "xbacklight -set $DAMA_VAL"
        # the command to be executed when the slider is moved.
        # the target value of the slider is available through                              
        # the environment variable $DAMA_VAL, rounded to an integer.

# choose between different string values
ComboBox :
  initialize: "echo \"option 1\noption2\noption3\""
        # a command providing a list of options separated by newlines
  select: "echo option2"
        # a command providing the option that should be selected 
        # by default. If this is not a valid option in the list above, 
        # then nothing will be selected
  on_update: "notify-send \"$DAMA_VAL\""
        # the command to be executed on change
```

Or with the json syntax:

```json
{ "Notebook" : 
  { "children": [ { "child 1"},  {"child 2"}, {" ... "} ] }
},

{"Box": { 
  "title": "Title",
  "orientation": "Vertical",
  "children": [{ "child 1"},  {"child 2 ..."}] 
}},

{"Label" : { "text":"some text" }},

{"Image" : { "path":"/absolute/path/to/image" }},

{"Button" : { "text":"text" , "on_click":"command"}}, 

{"CheckBox": 
  { "text": "text", 
    "initialize":"initial command", 
    "on_click": "update command" } },

{"Scale" : { 
  "range": { "low": 0.0 , "high": 100.0 }, 
  "initialize":"initial command", 
  "on_update": "update command" } },

{"ComboBox": {
  "initialize": "list command", 
  "select": "initial command", 
  "on_update": "update command" }}
```

All widgets additionally accept the `css` field, which should be a string containing valid css code.
Properties set this way are not inherited, so this is more for precision/tweaking the appearance of
single widgets.

A toplevel `Notebook` is implicitly added as a container for your pages. page names are handled
by reading the label of a top-level box, and user-defined Notebooks also behave this way.

all commands are executed with `sh -c`.

In a horizontal `Box`, if the first element is a `Label` it will expand to push
the remaining elements to the right of the window. This should result in a tidier layout:



```
Without label expansion:
 ┌──────────────────────────────────────────────┐
 │ Regular Label  [Btn][Btn]                    │ 
 │ Slightly longer Label  [Btn]                 │ 
 │ Short Label  [Large Button]                  │
 └──────────────────────────────────────────────┘

With label expansion:
 ┌──────────────────────────────────────────────┐
 │ Regular Label                    [Btn][Btn]  │
 │ Slightly longer Label                 [Btn]  │ 
 │ Short Label                  [Large Button]  │
 └──────────────────────────────────────────────┘
```

## Lazy loading

as of version 1.2.1 on the experimental branch, the contents of a Box will only be loaded on the first
draw call to that box. This is because the loading scripts are run sequentially, so having many pages
executing potentially long processes will slow down the startup; running the scripts on the first draw 
means that only pages you are looking at will be loaded.



