This repo is for checking feature scope of WASM Rust SDK APIs.

## How to build and run the proxy:
1. git clone
2. cargo +nightly build --target=wasm32-unknown-unknown --release
3. sudo dockerd (in Terminal 1)
4. sudo docker-compose (in Terminal 2)
5. curl  -H "key":"323232" 0.0.0.0:18000
