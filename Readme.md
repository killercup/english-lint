# English Lint

Find common stylistic problems in english texts like technical or scientific documents. Based on [write-good][npm-write-good] and [this article][matt].

This repository contains both a library and a simple CLI tool.

[npm-write-good]: https://github.com/btford/write-good
[matt]: http://matt.might.net/articles/shell-scripts-for-passive-voice-weasel-words-duplicates/

[![Build Status](https://travis-ci.org/killercup/english-lint.svg?branch=master)](https://travis-ci.org/killercup/english-lint)

## CLI Usage

First, install the CLI tool using `cargo install english-lint` (assumes you have recent Rust and Cargo version installed on your system).

You can either pipe text data into it (e.g. `echo "Hello world" | english-lint`) or call it with a file name paramter (`english-lint my-thesis.md`). It outputs one line for each suggestion containing the name of the lint group and the line number with start/end character indizes. If you don't see any output, _english-lint_ thinks your text is already perfect :)

```sh
$ english-lint Readme.md
wordy: 'shall' (22:72-77)
wordy: 'additional' (23:39-49)
```

### Library Usage

First, add _english-lint_ as a dependency to your project (e.g. using `cargo add english-lint` using [cargo-edit](https://github.com/killercup/cargo-edit)) and include it with `extern crate english_lint`;

Currently, this library only exposes two things: A free function, `english_lint::lint` and the struture this function returns, `english_lint::Hint`.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
