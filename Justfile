web-run:
	just web-build
	(sleep 1 && xdg-open http://localhost:8000)&
	just web-serve

web-serve:
	cd web && python -m http.server

web-build:
	cargo fmt
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen ./target/wasm32-unknown-unknown/debug/critfail.wasm --out-dir web/build --web

web-release:
	cargo fmt
	cargo build --target wasm32-unknown-unknown --release --no-default-features --features gui
	rm -rf ./web/build
	wasm-bindgen ./target/wasm32-unknown-unknown/release/critfail.wasm --out-dir web/build --web
	wasm-opt -Oz web/build/critfail_bg.wasm -o web/build/critfail_bg.wasm

web-deploy: web-release
	echo rm -vrI $CRITFAIL_SERVER_DIR
	ssh $CRITFAIL_SERVER rm -vrI $CRITFAIL_SERVER_DIR
	scp -r ./web $CRITFAIL_SERVER:$CRITFAIL_SERVER_DIR

web-size: web-release
	ls -lh web/build/critfail_bg.wasm
	stat -c %s web/build/critfail_bg.wasm

build:
	cargo fmt
	cargo build

run +ARGS="":
	cargo fmt
	cargo run {{ARGS}}

clean:
	cargo clean
	rm -rf web/build
