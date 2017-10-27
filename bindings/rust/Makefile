# Rust binding for Keystone engine. Remco Verhoef <remco@honeytrap.io>

.PHONY: gen_const install clean check

gen_const:
	cd .. && python const_generator.py rust
	cargo fmt

install:
	cargo build

clean:
	rm -rf target/ libkeystone-sys/target/

check:
	cargo test
