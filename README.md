# KeyPress

A Rust WebAssembly Websit example for practising english for chinese.

## How to build

The `KeyPress` is built with trunk. You can install it with the following command:
```
# Install trunk and wasm-bindgen-cli
# At some point in the future, trunk will automatically download wasm-bindgen
cargo install trunk wasm-bindgen-cli
```

Build and run is as easy as running a single command:
```
# build
cargo build --release
```

```
# build and serve
trunk serve --release
```

## Example

You can visit in the website [https://rayylee.github.io/keypress](https://rayylee.github.io/keypress)

![screenshot](/assets/screenshot.png)

## Acknowledgments

- [qwerty-learner](https://github.com/Kaiyiwing/qwerty-learner) - Thanks for designing the view!
- [yew](https://github.com/yewstack/yew) - Thanks for the examples!
