cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./dist/ --target web ./target/wasm32-unknown-unknown/release/nest_climb.wasm
cp -a ./assets/. ./dist/assets/
git add dist/*
git commit -m build -- dist/*
git push origin main