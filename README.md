# bark

a (very wip) minimal desktop shell, starting with a GTK4 bar for niri.

## requirements

- niri
- gtk4
- gtk4-layer-shell
- rust and cargo (when building from source)

## install

### cargo

```sh
cargo install bark-shell
```

### build from source

```sh
cargo build --release --locked
./target/release/bark
```

## hacking

this project uses devenv, so install that then run `devenv shell`
