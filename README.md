
> [!CAUTION]
> This tool is still under development!
> Use with Caution!

# branchtransfer

A command line tool to transfer files from one git branch to another

# Preliminary API

This tool is still in active development.

For now, it uses the following syntax:

```bash
bxfer -b <targetbranch> -t <target> <source>
```
This copies the contents of `source` directory in the current working directory and replaces the contents of the `target` directory in the `targetbranch` with them, and finally commits the changes to the `targetbranch`.
If the target directory is not provided, the contents are copied into the `source` directory in the `targetbranch`.

## What does this tool do?

It provides a binary that replaces the contents of the `target` subdirectory in the `targetbranch` with the contents of the `source` subdirectory in the current working directory in a git repo. Then, the changes are committed to the `targetbranch`.


## Inspiration

This tool is indebted to three tools that precede it:

- [`ghp-import`](https://github.com/c-w/ghp-import): almost exactly the same functionality
- [`quarto-cli`](http://github.com/quarto-dev/quarto-cli): similar functionality bundled in with _other_ stuff. It renders the `quarto` project, and copies the `source` directory configured in the `_quarto.yml` file to the target branch
- [`Documenter.jl`](https://github.com/JuliaDocs/Documenter.jl): similar functionality but bundled with _other_ stuff. It first renders the documentation, and the file transfer only works on CI machines. This is the genesis of this project! I wanted to manually transfer rendered documentation to the `gh-pages` branch on my local machine, but I couldn't force it to do that. It is possible that this functionality exists, and _I_ failed to see how to trigger the file transfer to `gh-pages`. But again, the file transfer only happens after a render, and this project aims to provide a standalone utility like `ghp-import`. I must add that I learned a lot from the code in this project. In fact, the idea of using a temporary directory came from the `deploydocs` function.


`ghp-import` mostly does what this `branchtransfer` does.
So why write a new tool?
There are a few reasons:

- **this is an excuse for me to learn Rust!**
- `ghp-import` as it says in its documentation will destroy the target branch. This tool only rewrites the contents of the target folder.
- `ghp-import` also copies any untracked files in the working directory
- to the best of my understanding (and I could be wrong) `ghp-import` does everything in current directory

`branchtransfer` uses git worktrees to copy files from one branch to another.
Thus, all potentially destructive operations occur in a separate directory altogether.
Additionally, to reduce potential issues, the worktree corresponding to the target directory is opened in a temporary directory (h/t `Documenter.jl`).
