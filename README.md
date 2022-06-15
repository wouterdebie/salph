# salph

Small application and rust library that helps you spell sentences in different spelling alphabets.

## Installation

`cargo install salph`

## Usage (cli)

```
USAGE:
    salph [OPTIONS] [SENTENCE]...

ARGS:
    <SENTENCE>...

OPTIONS:
    -a, --alphabet <ALPHABET>              Alphabet to use [env: SALPH=] [default: nato]
    -h, --help                             Print help information
    -l, --list-alphabets                   List available alphabets
    -s, --show-alphabet <SHOW_ALPHABET>    Show the contents of an alphabet
    -V, --version                          Print version information
```

`salph` can also be used through `stdin`:

`$ echo "some sentence" | salph`

Note that the alphabet can be set through either the `-a` command line option or the `SALPH` environment variable.

## Usage (library)

See usage documentation at https://docs.rs/salph/latest/salph/

## Building

Make sure to have Rust installed. Then build with `cargo build`.

## Alphabets

The list of supported alphabets is available at https://docs.rs/salph/latest/salph/enum.Alphabet.html and are kept in the [alphabets](https://github.com/wouterdebie/salph/tree/main/alphabets) directory. Most alphabets are taken from https://en.wikipedia.org/wiki/Spelling_alphabet. If you want to include other alphabets or have corrections, please add them and create a pull-request.

## Contributing

Please refer to each project's style and contribution guidelines for submitting patches and additions. In general, we follow the "fork-and-pull" Git workflow.

1.  **Fork** the repo on GitHub
2.  **Clone** the project to your own machine
3.  **Commit** changes to your own branch
4.  **Push** your work back up to your fork
5.  Submit a **Pull request** so that we can review your changes

NOTE: Be sure to merge the latest from "upstream" before making a pull request!
