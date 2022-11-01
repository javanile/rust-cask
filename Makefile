
build:
	@cargo build

publish:
	git add .
	git commit -am "New release is coming"
	git push
	cargo publish

## ====
## Test
## ====

test-help: build
	@cargo run -- --help

test-create-server: build
	@cargo run -- create:server test1
	@cat $${HOME}/.cask/test1.yml
