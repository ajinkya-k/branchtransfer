
> [!CAUTION]
> This tool is still under development!
> Use with Caution!

# branchtransfer

A command line tool to transfer files from one git branch to another


## What does this tool do?

It provides a binary that copies files from a `source` subdirectory in the current working directory in a git repo to the and replaces the contents of the `target` subdirectory in the `targetbranch`. Then, the changes are committed to the `targetbranch`.


## Inspiration

This tool is indebted to three tools that precede it:

- [ghp-import](https://github.com/c-w/ghp-import)
- [quarto-cli](http://github.com/quarto-dev/quarto-cli)
- [Documenter.jl](https://github.com/JuliaDocs/Documenter.jl)


`ghp-import` mostly does what this `branchtransfer` does.
So why write a new tool?
There are a few reasons:

- **this is an excuse for me to learn Rust!**
- `ghp-import` as it says in its documentation will destroy the target branch
- `ghp-import` also copies any untracked files in the working directory
- to the best of my understanding (and I could be wrong) `ghp-import` does everything in current directory

`branchtransfer` uses git worktrees to copy files from one branch to another.
Thus, all potentially destructive operations occur in a separate directory altogether.
Additionally, to reduce potential issues, the worktree corresponding to the target directory is opened in a temporary directory.

