# To-Do App
Not really todo app, just learning some basics of yew and wasm

## Resesources
- [Yew Sample App](https://yew.rs/getting-started/build-a-sample-app)
- [Yew Docs](https://docs.rs/yew/0.18.0/yew/)
- [Yew Crash Course](https://www.youtube.com/watch?v=lmLiMozWNGA&ab_channel=JeffNoZhao)

## Usage
Prerequisites:
```
cargo install trunk wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```

Run server (automatically updates when developing the app)
```
trunk serve --port=8080
```

Open your browser at localhost:8080