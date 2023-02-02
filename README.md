# bevy-pixel-tanks
attempt to impletement battle city game in bevy

For development
Run with log debug stacktrace on error and reload on save changes:
RUST_BACKTRACE=1 RUST_LOG="warn,chess-bevy-game=debug"  cargo watch -q -c -x 'run --features bevy/dynamic'

Install Cargo Watch
cargo install cargo-watch

Run in browser
https://bevy-cheatbook.github.io/platforms/wasm.html

rustup target install wasm32-unknown-unknown

cargo install wasm-server-runner

cargo run --target wasm32-unknown-unknown

wasm-server-runner target/wasm32-unknown-unknown/debug/bevy-pixel-tanks.wasm


cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/

для создания спрайта 
https://www.piskelapp.com/p/create/sprite