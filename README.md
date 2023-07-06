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
