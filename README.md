# Monarch
A simple CLI tool for adding/reading secret messages on images.

## Usage
Encode using cargo: `cargo run -- encode <Path/to/image/file> -m "<your secret message>" -o <optional output file (overwrites original image if left out)>`
Decode using cargo: `cargo run -- decode <Path/to/image/file>`
With executable just replace "cargo run --" with the executable name

## Notes
Monarch uses the alpha channel to encode the message. This means that if the output image format does not support alpha channels (ie. jpeg) the message will not encode.

check [here](https://github.com/image-rs/image) for a table of supported image formats. It is possible to encode and convert a jpeg to a png at the same time by just supplying Monarch with the given file extensions (eg. `monarch encode <Path/to/image/file.jpeg>  -m "<secret message>" -o <Path/to/image/file.png>`).

## Looking Forward
### Modes of Steganography
Monarch will support more modes to encode messages
### More Secret-y
Monarch should hide messages better.
current images encoded with messages won't look any different unless looking at the alpha channel (then it become obvious that something is there).
randomizing the rest of the alpha values not used for the message should help as well as different better modes.

