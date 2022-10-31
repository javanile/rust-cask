

publish:
	git add .
	git commit -am "New release is coming"
	git push
	cargo publish
