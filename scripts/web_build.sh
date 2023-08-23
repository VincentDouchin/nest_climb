cargo build --release --target wasm32-unknown-unknown
cargo build --release 
wasm-bindgen --out-dir ./NestClimb/src/wasm --target web ./target/wasm32-unknown-unknown/release/nest_climb.wasm
cp -a ./assets/. ./NestClimb/public/assets/
cp ./target/release/nest_climb.exe ./

git add ./NestClimb/src/*
git add ./NestClimb/public/*
git commit -m build -- NestClimb/public/* 
git commit -m build -- NestClimb/src/*
git commit -m build -- ./nest_climb.exe
git push origin main