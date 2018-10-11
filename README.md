# `upsidedown-rs`

A friend was disappointed that I had this in Perl. This updated edition is just
for them.

## Example

```shell
# Building it
cargo build
target/debug/upsidedown-rs "This will be upside down\!"
¡uʍop ǝpısdn ǝq ןןıʍ sıɥʇ

# or just `cargo run`
cargo run -- "This will be upside down\!"
¡uʍop ǝpısdn ǝq ןןıʍ sıɥʇ
```

## Limitations

Doesn't handle uppercase characters, so it lowercases them first.
