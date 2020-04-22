web-run:
	just web-build
	(sleep 1 && xdg-open http://localhost:8000)&
	just web-serve

web-serve:
	cd web && python -m http.server

web-build:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen ./target/wasm32-unknown-unknown/debug/critfail.wasm --out-dir web/build --web
