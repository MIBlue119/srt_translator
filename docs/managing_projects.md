# Notes

source: https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

## Rust module system

- Package: A Cargo feature that lets you build, test and share crates
- Crates: A tree modules that produces a lib or executable
- Modules and use: Let you control the organizaiton, scope, and privacy of paths
- Paths: A way of naming an item

### Packages and Crates

- A *crate* is the smallest amount of code that the Rust compiler considers at a
  time
  - two forms
    - a binary crate: programs you can compile to an executable that you can run,
      such as a command-line program or a server.
    - *Library crate*: don’t have a `main` function, and they don’t compile to an
      executable. Instead, they define functionality intended to be shared with
      multiple projects.
  - crate root file (usually *src/lib.rs* for a library crate or
    *src/main.rs* for a binary crate)
    - https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

## Resources

- Easiest Way to Understand Rust Modules Across Multiple Files: https://levelup.gitconnected.com/easiest-way-to-understand-rust-modules-across-multiple-files-234b5018cbfd
- How to import sibling module: https://stackoverflow.com/questions/56680200/how-to-access-modules-from-parent-or-sibling-folder-modules
