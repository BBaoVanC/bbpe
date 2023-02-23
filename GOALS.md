# Goals

First, start with the non-gloals:

## Non-goals

- compete with existing browsers
  - as in, we shouldn't build a Chromium reskin
  - and use vague, pompous marketing terms
  - or marketing at all; it should primarily be a tool to learn about web development

## Goals

- work on all major platforms (Windows, macOS, Linux)
- make use of existing browser engines, ui frameworks, etc to distribute the work
- see if it is possible to not just fork and reskin Chromium/Firefox

### Long-term goals (if it somehow even gets this far)

- build a modular browser which can support multiple engines, UI frameworks, etc
  - could make adoption of new browser engines easier by making them wrapped and compile-time swappable

## How should we get there

1. Start simple; use a large ecosystem (such as WebKit?) to get *something* working.
2. Then, slowly strip out parts into our own implementation (such as rendering perhaps?).
3. If that works out, try and wrap and modularize the components to make them replacable with other implementations.
4. Buy a lottery ticket because the chances we even get to step 2 are miniscule.
