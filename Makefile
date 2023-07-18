all:
	cargo build --release

format:
	find . -type f -name \*.rs -exec ucf "{}" \;

push:
	git push origin main
	git push gitlab main
