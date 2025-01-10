install-watch:
	cargo install cargo-watch --locked

watch:
	cargo watch -q -c -w src/ -x run

watch-test:
	cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"