# Ansi Señor

This is a simple Rust binary that will run whatever command you specify next with `CLICOLOR_FORCE=1` automatically exported, then both show the output and also capture it in a buffer file, then run an `ansi2html` conversion that then writes an html file with the output with proper ansi coloring to an output file specified.

## Usage

```
$ ansi-senor git status

---
❯ git status                                                                                                            took 9h4m23s
On branch gitbutler/workspace
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   content/2025/2025-11-05-gitbutler-cli.mdx

no changes added to commit (use "git add" and/or "git commit -a")
---

Output saved to /tmp/ansi-senor/git-status-fedd38ae.html
```

You can also specify a `-o file-path.html` to specify a specific output file.
