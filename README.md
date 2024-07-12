# Prerequisites

- wezterm
- npm
- npx
- bat

# How to run presentation

Start wezterm in the presentation directory

## presenting slides

Slides shou8ld be presented in the following order:
- intro*.png
- why*.png
- usecases*.png
- libraries*.png
- disclaimer*.png

```
. ./aliases.sh
imgcat slides/usecases.005.png
```

## presenting rust source code

In use cases after presenting slide usually goes source code

### output highlighted source code
```
bat src/count_cord.rs
```

### then we can run examples
```
cargo run --release --example count_word -- enwik8.txt world
RUSTFLAGS="-C target-cpu=native" cargo run --release --example count_word -- enwik8.txt world
```

# How to rebuild presentation slides

```
npm install
cd slides
npx marp --theme gaia --images png *.md --allow-local-files
```
