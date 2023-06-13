cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./NestClimb/src/wasm --target web ./target/wasm32-unknown-unknown/release/nest_climb.wasm
cp -a ./assets/. ./NestClimb/public/assets/

git add ./NestClimb/src/*
git add ./NestClimb/public/*
git commit -m build -- NestClimb/public/* 
git commit -m build -- NestClimb/src/*
git push origin main