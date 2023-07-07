# porytext-rs

This is a Rust CLI tool for converting plaintext files to gText entries for use with the  Pokémon generation 3
decompilation projects ([pokeruby](https://github.com/pret/pokeruby),
[pokeemerald](https://github.com/pret/pokeemerald), and [pokefirered](https://github.com/pret/pokefirered)).

## Usage

```
> ./porytext --help
A simple CLI tool to convert plain text to gText entries

Usage: porytext [OPTIONS] <FILE>

Arguments:
  <FILE>  The input file to read from

Options:
  -f, --font-id <FONT_ID>        Set the id for the font to use, default is defined in the font config file
  -c, --font-config <FILE>       Set the path to the font config file, default is ./font_config.json
  -b, --block-name <BLOCK_NAME>  The name of the generated gText block
  -h, --help                     Print help
  -V, --version                  Print version
```

Example:

`input.txt`:
```
The quick, brown fox jumps over a lazy dog.

DJs flock by when MTV ax quiz prog.

One morning, when Gregor Samsa woke from troubled dreams, he found himself transformed in his bed into a horrible vermin.

Waltz, bad nymph, for quick jigs vex!
```

```
> ./porytext -b Porytext_Test input.txt
gText_PoryText_Test::
        .string "The quick, brown fox jumps over a lazy\n"
        .string "dog.\p"
        .string "DJs flock by when MTV ax quiz prog.\p"
        .string "One morning, when Gregor Samsa woke\n"
        .string "from troubled dreams, he found himself\l"
        .string "transformed in his bed into a horrible\l"
        .string "vermin.\p"
        .string "Waltz, bad nymph, for quick jigs vex!$"
```
