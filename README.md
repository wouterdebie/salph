# NATO

Small application that helps you spell sentences in the NATO alphabet.

## Usage:

As a command line parameter

```
$ nato Lorem ipsum dolor sit amet
lorem: lima oscar romeo echo mike
ipsum: india papa sierra uniform mike
dolor: delta oscar lima oscar romeo
sit: sierra india tango
amet: alfa mike echo tango
```

From stdin

```
$ echo "Lorem ipsum dolor sit amet" | nato
Lorem ipsum dolor sit amet
lorem: lima oscar romeo echo mike
ipsum: india papa sierra uniform mike
dolor: delta oscar lima oscar romeo
sit: sierra india tango
amet: alfa mike echo tango
```

## Building:

Make sure to have Rust installed. Then build with `cargo build`.
