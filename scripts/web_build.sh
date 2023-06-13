cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./NestClimb/assets --target web ./target/wasm32-unknown-unknown/release/nest_climb.wasm
cp -a ./assets/. ./NestClimb/public/assets/
git add ./NestClimb/public/assets/*
git commit -m build -- NestClimb/*
git push origin main