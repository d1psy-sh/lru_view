# LRU View

## Motivation

This is a tool with which you can open any file from the command line.
It should be a extension to xdg-open and work on win, linux and mac.
As a student I often have to open multipel PDFs or other files, in the most cases
they are the same and I have to open them multiple times a week to read through
make my notes etc. .
While doing this I have to enter the full file path by hand way to often. What
I like to build is a tool which keeps my least recently used files in a list which
I can access by number and update that list. The LRU item should always be the first
and if i have to open a file by hand I want it to be automatically added to the list.

## Configuration

A configuration file is needed and should be stored in `~/.config/lru_view/config`:

- [ ] choose which file type to open with which application
- [ ] choose how many items are saved in the lru

## Ideas

- [ ] fzf filtering
- [ ] delete items from list manually
- [ ] config file
    - [ ] always have a save fallback like open with browser
    - [ ] save default configs so I can change it when I use windows
    - [ ] items should be opened disowned from the host session
- [ ] lru
    - [ ] test the lru
    - [x] implement LRU with counter
- [x] and a map
