cformat
=======

Purpose
-------

**cformat** is a program for conveniently formatting changes made to C or
C++ files using `clang-format`. Contrary to most formatters, the program
allows for *incremental* reformatting of files. That is, when lines in a
file are changed they and only they will be reformatted (optionally with
some context), while the remainder of the file will be preserved as-is.
Such behavior can be tremendously useful in incrementally migrating over
a code base to being formatted in an automated fashion.

Given that the program is merely a wrapper around
[`clang-format`][clang-format], it automatically picks up the
corresponding configuration.


Usage
-----

**cformat** interprets a patch in unified diff format and infers files and
changed lines based on it. This patch is read from standard input.

While not at all tied to `git`, we use it here for illustration purposes
(and because it is likely the context in which the program will be
used).
To simplify usage, we introduce a ``git`` alias (``git cfp`` -- *"c
format patch"*) that retrieves any currently staged changes and pipes
them to the program. An alias for annotating the currently staged
changes could look like this:

```git
[alias]
  cfp = "!cfp() { git diff --no-prefix --relative --staged --unified=0 \"$@\" | cformat; }; cfp"
```

Once you have staged changes, you can run `git cfp` and affected lines
will get reformatted according to `clang-format`'s rules.

Three aspects of the above definition are of particular relevance here:
- `--no-prefix` is important to make sure that paths contained in
  emitted diffs do not contain any `a/` prefixes but map right to files
  in the file system
- `--relative` is necessary because without it `git` emits file names
  relative to the repository root, whereas `cformat` may be invoked from a
  sub-directory, in which case paths would not necessarily map to files
- `--unified=0` ensures that only lines that were actually touched are
  reformatted, as opposed to lines contained in the context that is used
  for fuzzy matching; increase the context to format additional lines
  surrounding changes


Installation
------------

**cformat** is written in Rust and requires the Cargo package manager to be
built. It can be installed using `cargo install cformat`. It relies on
`clang-format` being installed and runnable.


[clang-format]: https://clang.llvm.org/docs/ClangFormat.html
