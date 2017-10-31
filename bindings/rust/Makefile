# Rust binding for Keystone engine. Remco Verhoef <remco@honeytrap.io>

.PHONY: gen_const install clean check

gen_const:
	cd .. && python const_generator.py rust
	cargo fmt

build:
	rsync -a ../.. keystone-sys/keystone --exclude bindings --filter ":- ../../.gitignore"
	cargo build -vv

package: build
	cd keystone-sys && cargo package
	cargo package

clean:
	rm -rf target/ keystone-sys/target/ keystone-sys/keystone/

check:
	cargo test
