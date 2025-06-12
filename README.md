

## wasm todo with: Rust React Postgres - Docker ...sidekick:wasm

cd ./frontend 
npm install --save-dev esbuild-copy-static-files
cd ./rust-interface
wasm-pack build --target web
cd ../
npm run build
cargo run -p to-do-actix-server

