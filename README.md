# cros
> _"Crossword Object Syntax"_

A markup language for writing crossword puzzles.

## Example

```txt
# example.cros
title "A crossword puzzle"
author "cal"

grid
    STACK
    CURL#
    PR#AT
    #SYNC
    CONGO

across
    "LIFO data structure"
    "If `wget` isn't installed, use ..."
    "Don't forget to merge this"
    "Unix command to run a command once at a specific time"
    "Use a-? for concurrency"
    "The country whose TLD is '.cd'"

down
    "Tool to copy files over SSH"
    "DBaaS based on SQLite"
    "CPU register for passing parameters to subfunctions (abbr.)"
    "Compiler front end included in LLVM"
    "Compiler trick for final recursion (abbr.)"
    "Two letters used for confirmation in a CLI program"
```

### Result

<table>
  <tbody>
    <tr><td>S</td><td>T</td><td>A</td><td>C</td><td>K</td></tr>
    <tr><td>C</td><td>U</td><td>R</td><td>L</td><td>#</td></tr>
    <tr><td>P</td><td>R</td><td>#</td><td>A</td><td>T</td></tr>
    <tr><td>#</td><td>S</td><td>Y</td><td>N</td><td>C</td></tr>
    <tr><td>C</td><td>O</td><td>N</td><td>G</td><td>O</td></tr>
  </tbody>
</table>

## Features

- Automatically infers grid size, word coordinates, and clue numbers.
- Convert `cros` puzzles into formats like [`.puz`](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki) and JSON.
- Check that a crossword is valid and has rotational symmetry.
