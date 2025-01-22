# ** windows **
# https://bevy-cheatbook.github.io/setup/cross/macos-windows.html
# with the `--no-default-features` we disable `dynamic_linking`
cargo build --target=x86_64-pc-windows-gnu --release --no-default-features
