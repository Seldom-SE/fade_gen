# `fade_gen`

Generates filters that fade to the given color for
[`seldom_pixel`](https://github.com/Seldom-SE/seldom_pixel/). Works for `seldom_pixel` 0.8 and
probably other version.

## Usage

```
Usage: fade_gen <PALETTE_PATH> <TO> <FRAMES> <OUT_PATH>

Arguments:
  <PALETTE_PATH>
  <TO>            Index of the color to which the output will fade. To find the index, count the colors in your palette from left to right and top to bottom (as if reading English), starting from 0, until you reach the target color.
  <FRAMES>        number of frames
  <OUT_PATH>

Options:
  -h, --help  Print help
```

## License

`fade_gen` is dual-licensed under MIT and Apache 2.0 at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
