# fotosave

Save the image in your clipboard to a file, from the terminal.

```sh
fotosave screenshot.png
```

Supports any format the [`image`](https://crates.io/crates/image) crate can encode — PNG, JPEG, BMP, WebP, etc. The format is inferred from the file extension.

## Installation

```sh
cargo install --path .
```

## Usage

```
fotosave <output-file>
```

Copy an image to your clipboard (screenshot, browser right-click → Copy Image, etc.), then run:

```sh
fotosave out.png     # save as PNG
fotosave out.jpg     # save as JPEG
fotosave out.bmp     # save as BMP
```

## Platform support

| Platform | Status |
|----------|--------|
| macOS    | Supported |
| Linux (X11 / Wayland) | Supported |

## Dependencies

- [`arboard`](https://crates.io/crates/arboard) — cross-platform clipboard access
- [`image`](https://crates.io/crates/image) — image encoding

## License

MIT
