version = "1.0"

binary:
	cargo build --release --target x86_64-unknown-linux-musl

docker: binary
	cp target/x86_64-unknown-linux-musl/release/rdserver docker
	docker build -t rdserver:${version} docker

clean:
	cargo clean