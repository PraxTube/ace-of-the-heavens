# Development

This game is developed completely open source.
I don't plan to make any money with it at all.

## Features

- Open console using `F1` when in game
- Take screenshots with `F12`

## Formatting

Use `cargo clippy` to format code, the CI will check clippy.

## Releases

To use the CI to build and release builds you need to push a tag.
The tag should have the format `vN.N.N` with `N` being digits,
any tag will trigger the pipeline, but tags should follow this versioning style.
