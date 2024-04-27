setup-web-build:
  rustup target add wasm32-unknown-unknown
  cargo install --locked trunk

web-build:
  trunk build --release --features comfy/ci-release
