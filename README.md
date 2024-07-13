### Overview ###

Tool for updating pages on the unofficial Anbennar gameplay wiki.

### Requirements ###
- Rust
- Anbennar mod files (`git submodule update --init --recursive`)
- EU4 game files in `./basegame/`
- [ImageMagick](https://imagemagick.org) and [TexConv](https://github.com/Microsoft/DirectXTex/wiki/Texconv) in `./magick/` for art conversion

### Usage ###
- Expects these environment variables

| Variable   | Example                            |
|------------|------------------------------------|
| `API_URL`  | `https://wiki.example.com/api.php` |
| `BOTNAME`  | `admin@wiki-bot`                   |
| `BOTPASS`  | `the-bot-account-token`            |
