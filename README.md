# cros
> _"Crossword Object Syntax"_

A markup language for writing crossword puzzles.

## Example

`cros` follows a TOML-like syntax.

```toml
# example.toml
[puzzle]
title = "A crossword puzzle"
author = "cal"
size = 5

[grid]
rows = [
    "STACK",
    "CURL#",
    "PR#AT",
    "#SYNC",
    "CONGO",
]

[across]
"[1,1]" = "LIFO data structure."
"[2,1]" = "If `wget` isn't installed, use ..."
"[3,1]" = "Don't forget to merge this."
"[3,7]" = "Unix command to run a command once at a specific time"
"[4,2]" = "SYNC"
"[1,5]" = "The country whose TLD is '.cd'"

[down]
"[1,1]" = "Tool to copy files over SSH"
"[1,2]" = "DBaaS based on SQLite"
"[1,3]" = "CPU register for passing parameters to subfunctions (acronym)"
"[1,4]" = "Compiler front end included in LLVM"
"[3,5]" = "Compiler trick for final recursion (acronym)"
"[4,4]" = "Two letters used for confirmation in a CLI program"
```

Alternatively, there is a simplified W.I.P. syntax using its own DSL:


```txt
# example.cros
title "A crossword puzzle"
author "cal"
size 5

grid
    STACK
    CURL#
    PR#AT
    #SYNC
    CONGO

across
    [1,1] "LIFO data structure."
    [2,1] "If `wget` isn't installed, use ..."
    [3,1] "Don't forget to merge this."
    [3,7] "Unix command to run a command once at a specific time"
    [4,2] "SYNC"
    [1,5] "The country whose TLD is '.cd'"

down
    [1,1] "Tool to copy files over SSH"
    [1,2] "DBaaS based on SQLite"
    [1,3] "CPU register for passing parameters to subfunctions (acronym)"
    [1,4] "Compiler front end included in LLVM"
    [3,5] "Compiler trick for final recursion (acronym)"
    [4,4] "Two letters used for confirmation in a CLI program"
```

## Features

- Convert `cros` puzzles into formats like [`.puz`](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki) and JSON.
- Check that a crossword is valid and has rotational symmetry.
