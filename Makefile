
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

test-create-server: build
	@cargo run create:server test1
