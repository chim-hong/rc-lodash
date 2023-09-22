# About rs-lodash
Its's a rust learning repository -- lodash rewritten by rust

# How to use?
```
// first terminal
cd rs-lodash
cargo watch -s "wasm-pack build --target bundler --out-dir ./pkg"

// second terminal
cd rs-lodash/pkg
npm link

// third terminal
cd rs-lodash/www
npm install
npm link rs-lodash
npm run dev

```
